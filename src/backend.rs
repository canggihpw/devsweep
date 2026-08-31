use crate::checkers;
use crate::cleanup_history::{CleanupHistory, CleanupItemRecord, CleanupRecord};
use crate::custom_paths;
use crate::scan_cache::{PathMetadata, PathTracker, ScanCache};
use crate::types::{CheckResult, CleanupItem};
use crate::utils::format_size;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::SystemTime;

// Type alias for checker functions to simplify type signatures.
pub type CheckerFn = fn() -> CheckResult;

/// Single source of truth for the categories this app scans, in display order.
///
/// The display name here is the canonical category name that is used everywhere
/// else: the scan cache key, the TTL config, the super-category bucket
/// (`SuperCategoryType::from_category_name`), and the settings tab.
pub fn all_category_checks() -> Vec<(&'static str, CheckerFn)> {
    vec![
        ("Docker", checkers::check_docker),
        ("Homebrew", checkers::check_homebrew),
        ("Swift Package Manager", checkers::check_swiftpm),
        ("Node.js/npm/yarn", checkers::check_npm_yarn),
        ("Bun", checkers::check_bun),
        ("Python", checkers::check_python),
        ("Rust/Cargo", checkers::check_rust),
        ("Xcode", checkers::check_xcode),
        ("Flutter", checkers::check_flutter),
        ("Java (Gradle/Maven)", checkers::check_gradle_maven),
        ("Go", checkers::check_go),
        ("Android", checkers::check_android),
        ("IDE Caches", checkers::check_ide_caches),
        ("Shell Caches", checkers::check_shell_caches),
        ("Database Caches", checkers::check_db_caches),
        ("System Logs", checkers::check_system_logs),
        ("Browser Caches", checkers::check_browser_caches),
        ("node_modules in Projects", checkers::check_node_modules),
        ("Git Repositories", checkers::check_git_repos),
        ("General Caches", checkers::check_general_caches),
        ("Custom Paths", custom_paths::check_custom_paths),
        ("Trash", checkers::check_trash),
    ]
}

#[derive(Debug, Clone)]
pub struct CategoryData {
    pub name: String,
    pub size: String,
    pub total_size: u64,
    pub item_count: i32,
    pub items: Vec<CleanupItem>,
}

/// A single step in a scan, in canonical display order.
#[derive(Debug, Clone)]
pub enum ScanTask {
    /// Category must be scanned (cache missing, expired, or forced).
    Run { name: String, check: CheckerFn },
    /// Category can be served straight from a still-valid cache entry.
    Cached { name: String },
}

impl CategoryData {
    pub fn new(name: String, result: CheckResult) -> Self {
        let item_count = result.items.len() as i32;
        Self {
            name,
            size: format_size(result.total_size),
            total_size: result.total_size,
            item_count,
            items: result.items,
        }
    }
}

pub struct StorageBackend {
    pub categories: HashMap<String, CategoryData>,
    pub scan_cache: ScanCache,
    pub cleanup_history: CleanupHistory,
}

impl StorageBackend {
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
            scan_cache: ScanCache::load(),
            cleanup_history: CleanupHistory::load(),
        }
    }

    /// Decide which categories need a real scan vs. which can come from cache.
    ///
    /// The caller is responsible for running the checks (optionally in parallel,
    /// streaming results) and committing them via [`Self::commit_category`] /
    /// [`Self::commit_cached_category`].
    pub fn plan_scan(&self, use_cache: bool) -> Vec<ScanTask> {
        all_category_checks()
            .into_iter()
            .map(|(name, check)| {
                let needs_rescan = !use_cache || self.scan_cache.needs_rescan(name);
                if needs_rescan {
                    ScanTask::Run {
                        name: name.to_string(),
                        check,
                    }
                } else {
                    ScanTask::Cached {
                        name: name.to_string(),
                    }
                }
            })
            .collect()
    }

    /// Run a single checker (no lock / no shared state) -> result + tracked paths.
    pub fn run_check(check: CheckerFn) -> (CheckResult, HashMap<PathBuf, PathMetadata>) {
        let result = check();

        let mut tracker = PathTracker::new();
        for item in &result.items {
            if let Some(path) = &item.path {
                tracker.track(path);
            }
        }
        (result, tracker.into_paths())
    }

    /// Commit a freshly-scanned category: update the cache and the in-memory
    /// category map, and return the display data for the UI.
    pub fn commit_category(
        &mut self,
        name: String,
        result: CheckResult,
        tracked_paths: HashMap<PathBuf, PathMetadata>,
    ) -> CategoryData {
        // Hand the items to the cache once (cloned here); the original `result`
        // is moved into the CategoryData for display.
        let cache_result = CheckResult {
            name: name.clone(),
            status: None,
            total_size: result.total_size,
            items: result.items.clone(),
            extra_data: Default::default(),
        };
        self.scan_cache
            .update_category(name.clone(), cache_result, tracked_paths);

        let category_data = CategoryData::new(name.clone(), result);
        self.categories.insert(name.clone(), category_data.clone());
        category_data
    }

    /// Commit a cached category without re-validating it (validity was already
    /// confirmed when the plan was built).
    pub fn commit_cached_category(&mut self, name: String) -> Option<CategoryData> {
        let category_data = self
            .scan_cache
            .get_cached_result(&name)
            .map(|result| CategoryData::new(name.clone(), result))?;
        self.categories.insert(name.clone(), category_data.clone());
        Some(category_data)
    }

    /// Scan with optional cache usage (incremental scanning).
    ///
    /// Runs the planned checks in parallel, commits them in canonical order, and
    /// returns the results. This is the blocking form used by tests and callers
    /// that want everything back at once; the UI uses [`Self::plan_scan`] +
    /// incremental commits instead to stream progress.
    pub fn scan_with_cache(&mut self, use_cache: bool) -> Vec<CategoryData> {
        self.categories.clear();

        let plan = self.plan_scan(use_cache);

        // Determine which checks actually need to run, and run them in parallel.
        let checks_to_run: Vec<(String, CheckerFn)> = plan
            .iter()
            .filter_map(|task| match task {
                ScanTask::Run { name, check } => Some((name.clone(), *check)),
                ScanTask::Cached { .. } => None,
            })
            .collect();

        let mut results_map: HashMap<String, (CheckResult, HashMap<PathBuf, PathMetadata>)> =
            checks_to_run
                .par_iter()
                .map(|(name, check)| {
                    let (result, paths) = Self::run_check(*check);
                    (name.clone(), (result, paths))
                })
                .collect();

        let mut final_results = Vec::new();

        // Commit in canonical order.
        for task in plan {
            match task {
                ScanTask::Run { name, .. } => {
                    if let Some((result, tracked_paths)) = results_map.remove(&name) {
                        let category_data =
                            self.commit_category(name.clone(), result, tracked_paths);
                        final_results.push(category_data);
                    }
                }
                ScanTask::Cached { name } => {
                    if let Some(category_data) = self.commit_cached_category(name) {
                        final_results.push(category_data);
                    }
                }
            }
        }

        // Save cache
        let _ = self.scan_cache.save();

        final_results
    }

    pub fn get_total_reclaimable(&self) -> u64 {
        self.categories.values().map(|c| c.total_size).sum()
    }

    pub fn execute_cleanup(&self, item: &CleanupItem) -> Result<String, String> {
        // Handle different types of cleanup
        if let Some(cmd) = &item.cleanup_command {
            self.execute_shell_command(cmd)
        } else if let Some(path) = &item.path {
            self.delete_path(path)
        } else {
            Err("No cleanup action available for this item".to_string())
        }
    }

    /// Execute cleanup with history tracking and quarantine support
    pub fn execute_cleanup_with_history(
        &mut self,
        items: &[CleanupItem],
        use_quarantine: bool,
    ) -> Result<String, String> {
        let record_id = format!(
            "cleanup_{}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        let mut record = CleanupRecord::new(record_id.clone());
        let mut success_count = 0;
        let mut error_count = 0;
        let mut error_messages = Vec::new();

        for item in items {
            // Check if this item should NOT be quarantined (special cases)
            let should_not_quarantine = if let Some(path) = &item.path {
                // Items that should be deleted directly, not quarantined:
                // 1. Trash itself
                // 2. Items using cleanup commands instead of path deletion
                path.ends_with(".Trash")
            } else {
                // Items without path (e.g., using cleanup_command) - delete directly
                false
            };

            let result = if should_not_quarantine || item.cleanup_command.is_some() {
                // Delete directly, not quarantined (can't undo)
                match self.execute_cleanup(item) {
                    Ok(msg) => {
                        success_count += 1;
                        record.add_item(CleanupItemRecord::success(item, None));
                        Ok(msg)
                    }
                    Err(e) => {
                        error_count += 1;
                        let error_msg = format!("{}: {}", item.item_type, e);
                        error_messages.push(error_msg.clone());
                        record.add_item(CleanupItemRecord::error(item, e.clone()));
                        Err(e)
                    }
                }
            } else if use_quarantine && item.path.is_some() {
                // Move to quarantine (can undo later)
                match self.cleanup_history.quarantine_item(item) {
                    Ok(quarantine_path) => {
                        success_count += 1;
                        record.add_item(CleanupItemRecord::success(item, Some(quarantine_path)));
                        Ok(format!("Quarantined: {}", item.item_type))
                    }
                    Err(e) => {
                        error_count += 1;
                        let error_msg = format!("{}: {}", item.item_type, e);
                        error_messages.push(error_msg.clone());
                        record.add_item(CleanupItemRecord::error(item, e.clone()));
                        Err(e)
                    }
                }
            } else {
                // Delete permanently (no undo possible)
                match self.execute_cleanup(item) {
                    Ok(msg) => {
                        success_count += 1;
                        record.add_item(CleanupItemRecord::success(item, None));
                        Ok(msg)
                    }
                    Err(e) => {
                        error_count += 1;
                        let error_msg = format!("{}: {}", item.item_type, e);
                        error_messages.push(error_msg.clone());
                        record.add_item(CleanupItemRecord::error(item, e.clone()));
                        Err(e)
                    }
                }
            };

            match result {
                Ok(msg) => println!("✓ {}", msg),
                Err(e) => eprintln!("✗ {}", e),
            }
        }

        // Add record to history
        self.cleanup_history.add_record(record);
        let _ = self.cleanup_history.save();

        // Invalidate cache for affected categories
        self.invalidate_cache_for_items(items);

        if error_count == 0 {
            Ok(format!("Successfully cleaned {} items", success_count))
        } else {
            // Print detailed errors to console
            if !error_messages.is_empty() {
                eprintln!("\n❌ Cleanup Errors:");
                for error_msg in &error_messages {
                    eprintln!("  • {}", error_msg);
                }
            }
            Err(format!(
                "Cleaned {} items with {} errors",
                success_count, error_count
            ))
        }
    }

    /// Undo the last cleanup operation
    pub fn undo_cleanup(&mut self, record_id: &str) -> Result<String, String> {
        let result = self.cleanup_history.undo(record_id)?;

        // Invalidate cache since files were restored
        self.scan_cache.clear();
        let _ = self.scan_cache.save();

        if result.error_count == 0 {
            Ok(format!(
                "Successfully restored {} items",
                result.success_count
            ))
        } else {
            Err(format!(
                "Restored {} items with {} errors",
                result.success_count, result.error_count
            ))
        }
    }

    /// Invalidate cache for items that were cleaned
    fn invalidate_cache_for_items(&mut self, _items: &[CleanupItem]) {
        // Simple approach: clear entire cache
        // More sophisticated: only invalidate affected categories
        self.scan_cache.clear();
        let _ = self.scan_cache.save();
    }

    /// Get all category TTL settings
    pub fn get_all_cache_ttls(&self) -> Vec<(String, u64)> {
        let config = self.scan_cache.get_config();
        config
            .category_ttls
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }

    /// Set TTL for a specific category (in seconds)
    pub fn set_cache_ttl(&mut self, category: &str, ttl_seconds: u64) {
        let mut config = self.scan_cache.get_config().clone();
        config.set_ttl(category, ttl_seconds);
        self.scan_cache.set_config(config);
    }

    /// Reset cache configuration to defaults
    pub fn reset_cache_config(&mut self) {
        use crate::scan_cache::CacheConfig;
        let default_config = CacheConfig::default_config();
        self.scan_cache.set_config(default_config);
    }

    fn execute_shell_command(&self, command: &str) -> Result<String, String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    fn delete_path(&self, path: &PathBuf) -> Result<String, String> {
        if !path.exists() {
            return Err(format!("Path does not exist: {}", path.display()));
        }

        // Special handling for Trash - delete contents, not the directory itself
        if path.ends_with(".Trash") {
            return self.empty_trash();
        }

        if path.is_dir() {
            fs::remove_dir_all(path).map_err(|e| format!("Failed to delete directory: {}", e))?;
            Ok(format!("Deleted directory: {}", path.display()))
        } else {
            fs::remove_file(path).map_err(|e| format!("Failed to delete file: {}", e))?;
            Ok(format!("Deleted file: {}", path.display()))
        }
    }

    fn empty_trash(&self) -> Result<String, String> {
        // Use shell command to empty trash contents (not delete the .Trash folder itself)
        // This avoids permission issues with the .Trash directory
        let home =
            dirs::home_dir().ok_or_else(|| "Could not determine home directory".to_string())?;

        let trash_path = home.join(".Trash");

        if !trash_path.exists() {
            return Ok("Trash is already empty".to_string());
        }

        // Delete all contents of .Trash/* but not .Trash itself
        let entries = fs::read_dir(&trash_path)
            .map_err(|e| format!("Failed to read Trash directory: {}", e))?;

        let mut deleted_count = 0;
        let mut errors = Vec::new();

        for entry in entries {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    let result = if path.is_dir() {
                        fs::remove_dir_all(&path)
                    } else {
                        fs::remove_file(&path)
                    };

                    match result {
                        Ok(_) => deleted_count += 1,
                        Err(e) => errors.push(format!("{}: {}", path.display(), e)),
                    }
                }
                Err(e) => errors.push(format!("Failed to read entry: {}", e)),
            }
        }

        if !errors.is_empty() {
            Err(format!(
                "Emptied {} items from Trash, but {} errors occurred:\n{}",
                deleted_count,
                errors.len(),
                errors.join("\n")
            ))
        } else {
            Ok(format!(
                "Successfully emptied Trash ({} items deleted)",
                deleted_count
            ))
        }
    }

    #[allow(dead_code)]
    pub fn get_items_for_category(&self, category_name: &str) -> Vec<CleanupItem> {
        self.categories
            .get(category_name)
            .map(|c| c.items.clone())
            .unwrap_or_default()
    }

    #[allow(dead_code)]
    pub fn get_all_items(&self) -> Vec<(String, CleanupItem)> {
        let mut all_items = Vec::new();
        for (category_name, category_data) in &self.categories {
            for item in &category_data.items {
                all_items.push((category_name.clone(), item.clone()));
            }
        }
        all_items
    }

    /// Get all quarantine records
    pub fn get_quarantine_records(&self) -> Vec<&CleanupRecord> {
        self.cleanup_history.get_records().iter().collect()
    }

    /// Clear all quarantine history and files
    pub fn clear_all_quarantine(&mut self) -> Result<String, String> {
        let stats = self.cleanup_history.stats();

        self.cleanup_history.clear_all()?;

        Ok(format!(
            "Cleared {} records and freed quarantine space",
            stats.total_records
        ))
    }

    /// Delete a specific quarantine item permanently
    pub fn delete_quarantine_item(
        &mut self,
        record_id: &str,
        item_index: usize,
    ) -> Result<String, String> {
        self.cleanup_history
            .delete_quarantine_item(record_id, item_index)
    }

    /// Get quarantine statistics
    pub fn get_quarantine_stats(&self) -> crate::cleanup_history::HistoryStats {
        self.cleanup_history.stats()
    }
}

impl Default for StorageBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// The canonical category name set. Keep this in sync with
    /// `SuperCategoryType::from_category_name` and the TTL defaults; this test
    /// fails loudly if `all_category_checks()` drifts (typo, rename, removed
    /// category) so the three sources can't silently disagree.
    #[test]
    fn test_all_category_checks_is_stable() {
        let actual: BTreeSet<&str> = all_category_checks()
            .into_iter()
            .map(|(name, _)| name)
            .collect();

        let expected: BTreeSet<&str> = [
            "Android",
            "Browser Caches",
            "Bun",
            "Custom Paths",
            "Database Caches",
            "Docker",
            "Flutter",
            "General Caches",
            "Git Repositories",
            "Go",
            "Homebrew",
            "IDE Caches",
            "Java (Gradle/Maven)",
            "Node.js/npm/yarn",
            "Python",
            "Rust/Cargo",
            "Shell Caches",
            "Swift Package Manager",
            "System Logs",
            "Trash",
            "Xcode",
            "node_modules in Projects",
        ]
        .into_iter()
        .collect();

        assert_eq!(actual, expected);
    }
}
