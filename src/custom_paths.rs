//! Custom scan paths module
//!
//! Allows users to add custom directories to scan for cleanup.
//! Paths are persisted to disk and scanned alongside built-in checkers.

use crate::types::{CheckResult, CleanupItem};
use crate::utils::format_size;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Configuration file name for custom paths
const CONFIG_FILE: &str = "custom_paths.json";

/// A custom path configured by the user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPath {
    /// The path to scan
    pub path: PathBuf,
    /// User-provided label for this path
    pub label: String,
    /// Whether this path is enabled for scanning
    pub enabled: bool,
    /// Whether to scan recursively (include subdirectories)
    pub recursive: bool,
}

impl CustomPath {
    pub fn new(path: PathBuf, label: String) -> Self {
        Self {
            path,
            label,
            enabled: true,
            recursive: true,
        }
    }
}

/// Manager for custom scan paths
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomPathsConfig {
    pub paths: Vec<CustomPath>,
}

impl CustomPathsConfig {
    /// Load configuration from disk
    pub fn load() -> Self {
        let config_path = Self::config_path();

        if !config_path.exists() {
            return Self::default();
        }

        match fs::read_to_string(&config_path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save configuration to disk
    pub fn save(&self) -> Result<(), String> {
        let config_path = Self::config_path();

        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize: {}", e))?;

        fs::write(&config_path, content).map_err(|e| format!("Failed to write config: {}", e))
    }

    /// Get the configuration file path
    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("devsweep")
            .join(CONFIG_FILE)
    }

    /// Add a new custom path
    pub fn add_path(&mut self, path: PathBuf, label: String) -> Result<(), String> {
        // Validate path exists
        if !path.exists() {
            return Err(format!("Path does not exist: {}", path.display()));
        }

        // Check for duplicates
        if self.paths.iter().any(|p| p.path == path) {
            return Err(format!("Path already added: {}", path.display()));
        }

        self.paths.push(CustomPath::new(path, label));
        self.save()
    }

    /// Remove a custom path by index
    pub fn remove_path(&mut self, index: usize) -> Result<(), String> {
        if index >= self.paths.len() {
            return Err("Invalid path index".to_string());
        }

        self.paths.remove(index);
        self.save()
    }

    /// Toggle a path's enabled status
    pub fn toggle_path(&mut self, index: usize) -> Result<(), String> {
        if index >= self.paths.len() {
            return Err("Invalid path index".to_string());
        }

        self.paths[index].enabled = !self.paths[index].enabled;
        self.save()
    }

    /// Get all enabled paths
    pub fn enabled_paths(&self) -> Vec<&CustomPath> {
        self.paths.iter().filter(|p| p.enabled).collect()
    }
}

/// Scan custom paths and return cleanup items
pub fn check_custom_paths() -> CheckResult {
    let mut result = CheckResult::new("Custom Paths");

    let config = CustomPathsConfig::load();

    for custom_path in config.enabled_paths() {
        if !custom_path.path.exists() {
            continue;
        }

        let size = calculate_path_size(&custom_path.path, custom_path.recursive);

        if size == 0 {
            continue;
        }

        let item = CleanupItem::new(&custom_path.label, size, &format_size(size))
            .with_path(custom_path.path.clone())
            .with_safe_to_delete(false) // User should confirm custom paths
            .with_warning("User-configured custom path");

        result.add_item(item);
    }

    result
}

/// Calculate the size of a path
fn calculate_path_size(path: &PathBuf, recursive: bool) -> u64 {
    if path.is_file() {
        return path.metadata().map(|m| m.len()).unwrap_or(0);
    }

    if recursive {
        WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter_map(|e| e.metadata().ok())
            .map(|m| m.len())
            .sum()
    } else {
        // Non-recursive: only direct children
        fs::read_dir(path)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter_map(|e| e.metadata().ok())
                    .filter(|m| m.is_file())
                    .map(|m| m.len())
                    .sum()
            })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_custom_path_new() {
        let path = PathBuf::from("/test/path");
        let custom = CustomPath::new(path.clone(), "Test Label".to_string());

        assert_eq!(custom.path, path);
        assert_eq!(custom.label, "Test Label");
        assert!(custom.enabled);
        assert!(custom.recursive);
    }

    #[test]
    fn test_custom_paths_config_default() {
        let config = CustomPathsConfig::default();
        assert!(config.paths.is_empty());
    }

    #[test]
    fn test_calculate_path_size() {
        let temp = tempdir().unwrap();
        let file_path = temp.path().join("test.txt");

        // Create a file with known content
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Hello, World!").unwrap();

        let size = calculate_path_size(&file_path, false);
        assert_eq!(size, 13); // "Hello, World!" is 13 bytes
    }

    #[test]
    fn test_check_custom_paths_empty() {
        let result = check_custom_paths();
        assert_eq!(result.name, "Custom Paths");
        // Result depends on user's config, but should not panic
    }
}
