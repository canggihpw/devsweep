use crate::types::CleanupItem;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

/// Maximum number of cleanup operations to keep in history
const MAX_HISTORY_SIZE: usize = 50;

/// Maximum size of quarantine directory (in bytes) - 10 GB
const MAX_QUARANTINE_SIZE: u64 = 10 * 1024 * 1024 * 1024;

/// Record of a single cleanup operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupRecord {
    pub id: String,
    pub timestamp: SystemTime,
    pub items: Vec<CleanupItemRecord>,
    pub total_size: u64,
    pub success_count: usize,
    pub error_count: usize,
    pub can_undo: bool,
}

impl CleanupRecord {
    pub fn new(id: String) -> Self {
        Self {
            id,
            timestamp: SystemTime::now(),
            items: Vec::new(),
            total_size: 0,
            success_count: 0,
            error_count: 0,
            can_undo: true,
        }
    }

    pub fn add_item(&mut self, item: CleanupItemRecord) {
        self.total_size += item.size;
        if item.success {
            self.success_count += 1;
        } else {
            self.error_count += 1;
        }
        self.items.push(item);
    }

    pub fn is_undoable(&self) -> bool {
        self.can_undo && self.success_count > 0
    }
}

/// Record of a single cleaned item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupItemRecord {
    pub item_type: String,
    pub original_path: PathBuf,
    pub quarantine_path: Option<PathBuf>,
    pub size: u64,
    pub success: bool,
    pub error_message: Option<String>,
    pub deleted_permanently: bool,
}

impl CleanupItemRecord {
    pub fn success(item: &CleanupItem, quarantine_path: Option<PathBuf>) -> Self {
        Self {
            item_type: item.item_type.clone(),
            original_path: item.path.clone().unwrap_or_default(),
            quarantine_path: quarantine_path.clone(),
            size: item.size,
            success: true,
            error_message: None,
            deleted_permanently: quarantine_path.is_none(),
        }
    }

    pub fn error(item: &CleanupItem, error: String) -> Self {
        Self {
            item_type: item.item_type.clone(),
            original_path: item.path.clone().unwrap_or_default(),
            quarantine_path: None,
            size: item.size,
            success: false,
            error_message: Some(error),
            deleted_permanently: false,
        }
    }

    pub fn can_restore(&self) -> bool {
        self.success && !self.deleted_permanently && self.quarantine_path.is_some()
    }
}

/// Manages cleanup history and undo operations
pub struct CleanupHistory {
    records: VecDeque<CleanupRecord>,
    quarantine_dir: PathBuf,
}

impl CleanupHistory {
    pub fn new() -> Self {
        let quarantine_dir = Self::get_quarantine_dir();
        // Ensure quarantine directory exists
        let _ = fs::create_dir_all(&quarantine_dir);
        Self {
            records: VecDeque::new(),
            quarantine_dir,
        }
    }

    /// Load history from disk
    pub fn load() -> Self {
        let mut history = Self::new();

        let history_file = Self::history_file_path();
        if let Ok(data) = fs::read_to_string(&history_file) {
            if let Ok(records) = serde_json::from_str::<VecDeque<CleanupRecord>>(&data) {
                history.records = records;
            }
        }

        // Ensure quarantine directory exists
        let _ = fs::create_dir_all(&history.quarantine_dir);

        history
    }

    /// Save history to disk
    pub fn save(&self) -> Result<(), String> {
        let history_file = Self::history_file_path();

        // Ensure parent directory exists
        if let Some(parent) = history_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create history directory: {}", e))?;
        }

        let json = serde_json::to_string_pretty(&self.records)
            .map_err(|e| format!("Failed to serialize history: {}", e))?;

        fs::write(&history_file, json)
            .map_err(|e| format!("Failed to write history file: {}", e))?;

        Ok(())
    }

    /// Get the history file path
    fn history_file_path() -> PathBuf {
        let cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        cache_dir
            .join("development-cleaner")
            .join("cleanup_history.json")
    }

    /// Get the quarantine directory path
    fn get_quarantine_dir() -> PathBuf {
        let cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        cache_dir.join("development-cleaner").join("quarantine")
    }

    /// Add a new cleanup record
    pub fn add_record(&mut self, record: CleanupRecord) {
        self.records.push_front(record);

        // Limit history size
        while self.records.len() > MAX_HISTORY_SIZE {
            if let Some(old_record) = self.records.pop_back() {
                // Clean up quarantined files from old records
                self.cleanup_quarantine_for_record(&old_record);
            }
        }

        // Check quarantine size and cleanup if needed
        let _ = self.cleanup_old_quarantine_if_needed();
    }

    /// Get all records
    pub fn get_records(&self) -> &VecDeque<CleanupRecord> {
        &self.records
    }

    /// Get a specific record by ID
    pub fn get_record(&self, id: &str) -> Option<&CleanupRecord> {
        self.records.iter().find(|r| r.id == id)
    }

    /// Move item to quarantine instead of deleting permanently
    pub fn quarantine_item(&self, item: &CleanupItem) -> Result<PathBuf, String> {
        let original_path = item
            .path
            .as_ref()
            .ok_or_else(|| "Item has no path".to_string())?;

        if !original_path.exists() {
            return Err(format!("Path does not exist: {}", original_path.display()));
        }

        // Create unique quarantine path using timestamp and original filename
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let filename = original_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let quarantine_path = self
            .quarantine_dir
            .join(format!("{}_{}", timestamp, filename));

        // Ensure quarantine directory exists
        fs::create_dir_all(&self.quarantine_dir)
            .map_err(|e| format!("Failed to create quarantine directory: {}", e))?;

        // Move to quarantine
        fs::rename(original_path, &quarantine_path)
            .map_err(|e| format!("Failed to move to quarantine: {}", e))?;

        Ok(quarantine_path)
    }

    /// Restore an item from quarantine
    pub fn restore_item(&self, record: &CleanupItemRecord) -> Result<String, String> {
        if !record.can_restore() {
            return Err("Item cannot be restored".to_string());
        }

        let quarantine_path = record
            .quarantine_path
            .as_ref()
            .ok_or_else(|| "No quarantine path".to_string())?;

        if !quarantine_path.exists() {
            return Err("Quarantined file no longer exists".to_string());
        }

        // Check if original location is available
        if record.original_path.exists() {
            return Err(format!(
                "Original location already exists: {}",
                record.original_path.display()
            ));
        }

        // Ensure parent directory exists
        if let Some(parent) = record.original_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create parent directory: {}", e))?;
        }

        // Move back from quarantine
        fs::rename(quarantine_path, &record.original_path)
            .map_err(|e| format!("Failed to restore from quarantine: {}", e))?;

        Ok(format!("Restored: {}", record.original_path.display()))
    }

    /// Undo a cleanup operation
    pub fn undo(&mut self, record_id: &str) -> Result<UndoResult, String> {
        let record = self
            .get_record(record_id)
            .ok_or_else(|| "Record not found".to_string())?
            .clone();

        if !record.is_undoable() {
            return Err("This cleanup cannot be undone".to_string());
        }

        let mut success_count = 0;
        let mut error_count = 0;
        let mut errors = Vec::new();

        for item in &record.items {
            if item.can_restore() {
                match self.restore_item(item) {
                    Ok(msg) => {
                        success_count += 1;
                        println!("✓ {}", msg);
                    }
                    Err(e) => {
                        error_count += 1;
                        errors.push(format!("{}: {}", item.item_type, e));
                        eprintln!("✗ {}", e);
                    }
                }
            }
        }

        // Mark record as undone
        if let Some(rec) = self.records.iter_mut().find(|r| r.id == record_id) {
            rec.can_undo = false;
        }

        let _ = self.save();

        Ok(UndoResult {
            success_count,
            error_count,
            errors,
        })
    }

    /// Clear all history and quarantine
    pub fn clear_all(&mut self) -> Result<(), String> {
        // Remove all quarantined files
        if self.quarantine_dir.exists() {
            fs::remove_dir_all(&self.quarantine_dir)
                .map_err(|e| format!("Failed to remove quarantine directory: {}", e))?;
        }

        // Recreate empty quarantine directory
        fs::create_dir_all(&self.quarantine_dir)
            .map_err(|e| format!("Failed to create quarantine directory: {}", e))?;

        // Clear records
        self.records.clear();

        self.save()?;

        Ok(())
    }

    /// Clean up quarantine files for a specific record
    fn cleanup_quarantine_for_record(&self, record: &CleanupRecord) {
        for item in &record.items {
            if let Some(quarantine_path) = &item.quarantine_path {
                if quarantine_path.exists() {
                    let _ = if quarantine_path.is_dir() {
                        fs::remove_dir_all(quarantine_path)
                    } else {
                        fs::remove_file(quarantine_path)
                    };
                }
            }
        }
    }

    /// Get total size of quarantine directory
    fn get_quarantine_size(&self) -> u64 {
        if !self.quarantine_dir.exists() {
            return 0;
        }

        walkdir::WalkDir::new(&self.quarantine_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|e| e.metadata().ok())
            .filter(|m| m.is_file())
            .map(|m| m.len())
            .sum()
    }

    /// Clean up old quarantine files if size exceeds limit
    fn cleanup_old_quarantine_if_needed(&mut self) -> Result<(), String> {
        let current_size = self.get_quarantine_size();

        if current_size > MAX_QUARANTINE_SIZE {
            // Remove oldest records until size is under limit
            while self.get_quarantine_size() > MAX_QUARANTINE_SIZE * 8 / 10 {
                if let Some(old_record) = self.records.pop_back() {
                    self.cleanup_quarantine_for_record(&old_record);
                } else {
                    break;
                }
            }
            self.save()?;
        }

        Ok(())
    }

    /// Delete a specific quarantine item permanently
    pub fn delete_quarantine_item(
        &mut self,
        record_id: &str,
        item_index: usize,
    ) -> Result<String, String> {
        // Find the record
        let record = self
            .records
            .iter_mut()
            .find(|r| r.id == record_id)
            .ok_or_else(|| "Record not found".to_string())?;

        // Check if item index is valid
        if item_index >= record.items.len() {
            return Err("Item index out of bounds".to_string());
        }

        let item = &record.items[item_index];

        // Clone data we need before mutable borrow
        let item_type = item.item_type.clone();
        let quarantine_path = item.quarantine_path.clone();

        // Only delete if item is in quarantine
        if let Some(qpath) = quarantine_path {
            if qpath.exists() {
                // Delete the quarantined file/directory
                let result = if qpath.is_dir() {
                    fs::remove_dir_all(&qpath)
                } else {
                    fs::remove_file(&qpath)
                };

                result.map_err(|e| format!("Failed to delete quarantined item: {}", e))?;

                // Mark the item as deleted permanently
                let item_mut = &mut record.items[item_index];
                item_mut.deleted_permanently = true;
                item_mut.quarantine_path = None;

                self.save()?;

                Ok(format!("Deleted: {}", item_type))
            } else {
                Err("Quarantined file no longer exists".to_string())
            }
        } else {
            Err("Item is not in quarantine".to_string())
        }
    }

    /// Get statistics about the history
    pub fn stats(&self) -> HistoryStats {
        let undoable_count = self.records.iter().filter(|r| r.is_undoable()).count();
        let total_items: usize = self.records.iter().map(|r| r.items.len()).sum();
        let quarantine_size = self.get_quarantine_size();

        HistoryStats {
            total_records: self.records.len(),
            undoable_records: undoable_count,
            total_items_cleaned: total_items,
            quarantine_size,
        }
    }
}

impl Default for CleanupHistory {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of an undo operation
#[derive(Debug)]
pub struct UndoResult {
    pub success_count: usize,
    pub error_count: usize,
    #[allow(dead_code)]
    pub errors: Vec<String>,
}

/// Statistics about cleanup history
#[derive(Debug)]
pub struct HistoryStats {
    pub total_records: usize,
    #[allow(dead_code)]
    pub undoable_records: usize,
    pub total_items_cleaned: usize,
    pub quarantine_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CleanupItem;

    #[test]
    fn test_cleanup_record_creation() {
        let mut record = CleanupRecord::new("test-1".to_string());
        assert_eq!(record.success_count, 0);
        assert_eq!(record.error_count, 0);
        assert!(record.can_undo);

        let item = CleanupItem::new("Test", 1024, "1 KB");
        let item_record = CleanupItemRecord::success(&item, None);
        record.add_item(item_record);

        assert_eq!(record.success_count, 1);
        assert_eq!(record.total_size, 1024);
    }

    #[test]
    fn test_history_save_load() {
        let mut history = CleanupHistory::new();
        let record = CleanupRecord::new("test-record".to_string());
        history.add_record(record);

        history.save().unwrap();

        let loaded = CleanupHistory::load();
        assert_eq!(loaded.records.len(), 1);
        assert!(loaded.get_record("test-record").is_some());
    }
}
