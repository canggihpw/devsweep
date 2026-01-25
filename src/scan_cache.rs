use crate::types::{CheckResult, CleanupItem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Metadata for tracking file/directory changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathMetadata {
    pub modified_time: Option<SystemTime>,
    pub size: u64,
    pub is_dir: bool,
}

impl PathMetadata {
    pub fn from_path(path: &Path) -> Option<Self> {
        let metadata = fs::metadata(path).ok()?;
        Some(Self {
            modified_time: metadata.modified().ok(),
            size: metadata.len(),
            is_dir: metadata.is_dir(),
        })
    }

    pub fn has_changed(&self, path: &Path) -> bool {
        match Self::from_path(path) {
            Some(current) => {
                // Check if modified time or size changed
                self.modified_time != current.modified_time || self.size != current.size
            }
            None => true, // Path no longer exists, so it changed
        }
    }
}

/// Cached result for a single category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedCategoryResult {
    pub name: String,
    pub items: Vec<CleanupItem>,
    pub total_size: u64,
    pub scan_timestamp: SystemTime,
    pub tracked_paths: HashMap<PathBuf, PathMetadata>,
    #[serde(default)]
    pub ttl_seconds: Option<u64>,
}

impl CachedCategoryResult {
    pub fn new(
        name: String,
        result: CheckResult,
        tracked_paths: HashMap<PathBuf, PathMetadata>,
    ) -> Self {
        Self {
            name,
            items: result.items,
            total_size: result.total_size,
            scan_timestamp: SystemTime::now(),
            tracked_paths,
            ttl_seconds: None,
        }
    }

    pub fn with_ttl(mut self, ttl_seconds: u64) -> Self {
        self.ttl_seconds = Some(ttl_seconds);
        self
    }

    /// Check if this cached result is still valid
    pub fn is_valid(&self) -> bool {
        // Check TTL first
        if let Some(ttl) = self.ttl_seconds {
            if let Ok(age) = self.scan_timestamp.elapsed() {
                if age.as_secs() > ttl {
                    return false; // Expired by TTL
                }
            }
        }

        // Check if any tracked paths have changed
        for (path, metadata) in &self.tracked_paths {
            if !path.exists() || metadata.has_changed(path) {
                return false;
            }
        }
        true
    }

    /// Convert back to CheckResult
    pub fn to_check_result(&self) -> CheckResult {
        CheckResult {
            name: self.name.clone(),
            status: None,
            items: self.items.clone(),
            total_size: self.total_size,
            extra_data: Default::default(),
        }
    }
}

/// Configuration for category-specific cache behavior
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheConfig {
    pub category_ttls: HashMap<String, u64>,
}

impl CacheConfig {
    pub fn default_config() -> Self {
        let mut category_ttls = HashMap::new();

        // Recommended TTLs based on category characteristics (in seconds)
        category_ttls.insert("Trash".to_string(), 0); // Never cache
        category_ttls.insert("General Caches".to_string(), 30); // 30 seconds
        category_ttls.insert("Docker".to_string(), 300); // 5 minutes
        category_ttls.insert("Homebrew".to_string(), 3600); // 1 hour
        category_ttls.insert("Node.js/npm/yarn".to_string(), 600); // 10 minutes
        category_ttls.insert("Python".to_string(), 600); // 10 minutes
        category_ttls.insert("Rust/Cargo".to_string(), 300); // 5 minutes
        category_ttls.insert("Xcode".to_string(), 300); // 5 minutes
        category_ttls.insert("Java (Gradle/Maven)".to_string(), 600); // 10 minutes
        category_ttls.insert("Go".to_string(), 600); // 10 minutes
        category_ttls.insert("node_modules in Projects".to_string(), 300); // 5 minutes
        category_ttls.insert("IDE Caches".to_string(), 600); // 10 minutes
        category_ttls.insert("Shell Caches".to_string(), 300); // 5 minutes

        Self { category_ttls }
    }

    pub fn get_ttl(&self, category: &str) -> Option<u64> {
        self.category_ttls.get(category).copied()
    }

    /// Set TTL for a specific category (in seconds) and save
    pub fn set_ttl(&mut self, category: &str, ttl_seconds: u64) {
        self.category_ttls.insert(category.to_string(), ttl_seconds);
        let _ = self.save();
    }

    pub fn load() -> Self {
        let config_path = Self::config_file_path();
        if let Ok(data) = fs::read_to_string(&config_path) {
            if let Ok(config) = serde_json::from_str(&data) {
                return config;
            }
        }
        Self::default_config()
    }

    pub fn save(&self) -> Result<(), String> {
        let config_path = Self::config_file_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&config_path, json).map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    fn config_file_path() -> PathBuf {
        let cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        cache_dir
            .join("development-cleaner")
            .join("cache_config.json")
    }
}

/// Cache for scan results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanCache {
    pub categories: HashMap<String, CachedCategoryResult>,
    pub last_full_scan: Option<SystemTime>,
    #[serde(skip)]
    pub config: CacheConfig,
}

impl ScanCache {
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
            last_full_scan: None,
            config: CacheConfig::default_config(),
        }
    }

    /// Load cache from disk
    pub fn load() -> Self {
        let cache_path = Self::cache_file_path();
        let mut cache = if let Ok(data) = fs::read_to_string(&cache_path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::new()
        };

        // Load config separately
        cache.config = CacheConfig::load();
        cache
    }

    /// Save cache to disk
    pub fn save(&self) -> Result<(), String> {
        let cache_path = Self::cache_file_path();

        // Ensure parent directory exists
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create cache directory: {}", e))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize cache: {}", e))?;

        fs::write(&cache_path, json).map_err(|e| format!("Failed to write cache file: {}", e))?;

        Ok(())
    }

    /// Get the cache file path
    fn cache_file_path() -> PathBuf {
        let cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        cache_dir
            .join("development-cleaner")
            .join("scan_cache.json")
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.categories.clear();
        self.last_full_scan = None;
    }

    /// Update cache with new scan result
    pub fn update_category(
        &mut self,
        name: String,
        result: CheckResult,
        tracked_paths: HashMap<PathBuf, PathMetadata>,
    ) {
        let mut cached = CachedCategoryResult::new(name.clone(), result, tracked_paths);

        // Apply TTL from config
        if let Some(ttl) = self.config.get_ttl(&name) {
            cached = cached.with_ttl(ttl);
        }

        self.categories.insert(name, cached);
        self.last_full_scan = Some(SystemTime::now());
    }

    /// Get cached result if still valid
    pub fn get_valid_category(&self, name: &str) -> Option<CheckResult> {
        if let Some(cached) = self.categories.get(name) {
            if cached.is_valid() {
                return Some(cached.to_check_result());
            }
        }
        None
    }

    /// Check if a category needs rescanning
    pub fn needs_rescan(&self, name: &str) -> bool {
        match self.categories.get(name) {
            Some(cached) => !cached.is_valid(),
            None => true,
        }
    }

    /// Get the cache config
    pub fn get_config(&self) -> &CacheConfig {
        &self.config
    }

    /// Update the cache config
    pub fn set_config(&mut self, config: CacheConfig) {
        self.config = config;
        let _ = self.config.save();
    }
}

impl Default for ScanCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to track paths during scanning
pub struct PathTracker {
    paths: HashMap<PathBuf, PathMetadata>,
}

impl PathTracker {
    pub fn new() -> Self {
        Self {
            paths: HashMap::new(),
        }
    }

    /// Track a single path
    pub fn track(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        if let Some(metadata) = PathMetadata::from_path(path) {
            self.paths.insert(path.to_path_buf(), metadata);
        }
    }

    /// Get all tracked paths
    pub fn into_paths(self) -> HashMap<PathBuf, PathMetadata> {
        self.paths
    }
}

impl Default for PathTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_path_metadata_change_detection() {
        let temp_file = std::env::temp_dir().join("test_metadata.txt");

        // Create file
        fs::write(&temp_file, "initial").unwrap();
        let meta1 = PathMetadata::from_path(&temp_file).unwrap();

        // Wait a bit to ensure modified time changes
        thread::sleep(Duration::from_millis(10));

        // Modify file
        fs::write(&temp_file, "modified").unwrap();
        assert!(meta1.has_changed(&temp_file));

        // Cleanup
        let _ = fs::remove_file(&temp_file);
    }

    #[test]
    fn test_cache_save_load() {
        let mut cache = ScanCache::new();
        let result = CheckResult {
            name: "Test".to_string(),
            status: None,
            items: vec![],
            total_size: 1024,
            extra_data: Default::default(),
        };

        let mut tracker = PathTracker::new();
        tracker.track(std::env::temp_dir());

        cache.update_category("Test".to_string(), result, tracker.into_paths());

        // Save and load
        cache.save().unwrap();
        let loaded = ScanCache::load();

        assert!(loaded.categories.contains_key("Test"));
    }
}
