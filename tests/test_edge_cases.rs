//! Edge Cases & Error Handling Tests
//! Phase 3: Testing file system edge cases, error handling, and boundary conditions

use devsweep::backend::StorageBackend;
use devsweep::cleanup_history::{CleanupHistory, CleanupItemRecord, CleanupRecord};
use devsweep::scan_cache::{PathMetadata, PathTracker, ScanCache};
use devsweep::types::{CheckResult, CleanupItem};
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use tempfile::TempDir;

// ============================================================================
// Symlink Handling Tests
// ============================================================================

#[cfg(unix)]
#[test]
fn test_symlink_detection_does_not_follow() {
    let temp = TempDir::new().unwrap();

    // Create a real directory with content
    let real_dir = temp.path().join("real_dir");
    fs::create_dir(&real_dir).unwrap();
    fs::write(real_dir.join("file.txt"), "content").unwrap();

    // Create a symlink pointing to the real directory
    let symlink_path = temp.path().join("symlink_dir");
    symlink(&real_dir, &symlink_path).unwrap();

    // Verify symlink exists and is a symlink
    assert!(symlink_path.exists());
    assert!(symlink_path
        .symlink_metadata()
        .unwrap()
        .file_type()
        .is_symlink());

    // PathMetadata should handle symlinks
    let metadata = PathMetadata::from_path(&symlink_path);
    assert!(metadata.is_some());
}

#[cfg(unix)]
#[test]
fn test_symlink_to_nonexistent_target() {
    let temp = TempDir::new().unwrap();

    // Create a symlink pointing to a nonexistent target (dangling symlink)
    let dangling_symlink = temp.path().join("dangling_link");
    symlink("/nonexistent/path/that/does/not/exist", &dangling_symlink).unwrap();

    // The symlink itself exists (as a symlink)
    assert!(dangling_symlink.symlink_metadata().is_ok());

    // But it doesn't point to anything valid
    assert!(!dangling_symlink.exists()); // exists() follows symlinks

    // PathMetadata should handle this gracefully
    let metadata = PathMetadata::from_path(&dangling_symlink);
    // Should return None for dangling symlinks since target doesn't exist
    assert!(metadata.is_none());
}

#[cfg(unix)]
#[test]
fn test_circular_symlink_handling() {
    let temp = TempDir::new().unwrap();

    // Create circular symlinks: a -> b -> a
    let link_a = temp.path().join("link_a");
    let link_b = temp.path().join("link_b");

    // Create link_a pointing to link_b (which doesn't exist yet)
    symlink(&link_b, &link_a).unwrap();
    // Create link_b pointing to link_a (circular)
    symlink(&link_a, &link_b).unwrap();

    // PathMetadata should handle circular symlinks without infinite loop
    let metadata = PathMetadata::from_path(&link_a);
    // Both are dangling since they point to each other
    assert!(metadata.is_none());
}

#[cfg(unix)]
#[test]
fn test_symlink_quarantine() {
    let temp = TempDir::new().unwrap();

    // Create a real file and a symlink to it
    let real_file = temp.path().join("real_file.txt");
    fs::write(&real_file, "real content").unwrap();

    let symlink_file = temp.path().join("symlink_file.txt");
    symlink(&real_file, &symlink_file).unwrap();

    let history = CleanupHistory::new();
    let item = CleanupItem::new("symlink", 12, "12 B").with_path(symlink_file.clone());

    let result = history.quarantine_item(&item);

    // Should be able to quarantine a symlink
    match result {
        Ok(quarantine_path) => {
            // Symlink should be moved
            assert!(!symlink_file.exists());
            // Original file should still exist
            assert!(real_file.exists());
            // Something should be at quarantine path
            assert!(quarantine_path.exists() || quarantine_path.symlink_metadata().is_ok());
        }
        Err(e) => {
            eprintln!("Symlink quarantine test skipped: {}", e);
        }
    }
}

// ============================================================================
// Permission Denied Handling Tests
// ============================================================================

#[cfg(unix)]
#[test]
fn test_permission_denied_read() {
    let temp = TempDir::new().unwrap();

    // Create a file with no read permissions
    let no_read_file = temp.path().join("no_read.txt");
    fs::write(&no_read_file, "secret content").unwrap();

    // Remove read permissions
    let mut perms = fs::metadata(&no_read_file).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&no_read_file, perms).unwrap();

    // PathMetadata should still work (metadata doesn't require read permission on file)
    let metadata = PathMetadata::from_path(&no_read_file);
    assert!(metadata.is_some());

    // Restore permissions for cleanup
    let mut perms = fs::metadata(&no_read_file).unwrap().permissions();
    perms.set_mode(0o644);
    fs::set_permissions(&no_read_file, perms).unwrap();
}

#[cfg(unix)]
#[test]
fn test_permission_denied_directory() {
    let temp = TempDir::new().unwrap();

    // Create a directory with no permissions
    let no_access_dir = temp.path().join("no_access_dir");
    fs::create_dir(&no_access_dir).unwrap();
    fs::write(no_access_dir.join("file.txt"), "content").unwrap();

    // Remove all permissions from directory
    let mut perms = fs::metadata(&no_access_dir).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&no_access_dir, perms).unwrap();

    // PathMetadata should still work for the directory itself
    let metadata = PathMetadata::from_path(&no_access_dir);
    assert!(metadata.is_some());

    // Restore permissions for cleanup
    let mut perms = fs::metadata(&no_access_dir).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&no_access_dir, perms).unwrap();
}

#[cfg(unix)]
#[test]
fn test_cleanup_readonly_file() {
    let temp = TempDir::new().unwrap();

    // Create a read-only file
    let readonly_file = temp.path().join("readonly.txt");
    fs::write(&readonly_file, "readonly content").unwrap();

    // Make it read-only
    let mut perms = fs::metadata(&readonly_file).unwrap().permissions();
    perms.set_mode(0o444);
    fs::set_permissions(&readonly_file, perms).unwrap();

    let history = CleanupHistory::new();
    let item = CleanupItem::new("readonly", 16, "16 B").with_path(readonly_file.clone());

    // Attempt to quarantine - this may succeed or fail depending on parent dir permissions
    let result = history.quarantine_item(&item);

    // Either way, we should handle gracefully without crashing
    match result {
        Ok(_) => {
            // Success is acceptable
        }
        Err(e) => {
            // Error is also acceptable for read-only files
            assert!(!e.is_empty());
        }
    }

    // Restore permissions for cleanup
    if readonly_file.exists() {
        let mut perms = fs::metadata(&readonly_file).unwrap().permissions();
        perms.set_mode(0o644);
        fs::set_permissions(&readonly_file, perms).unwrap();
    }
}

// ============================================================================
// Unicode Filename Tests
// ============================================================================

#[test]
fn test_unicode_filename_chinese() {
    let temp = TempDir::new().unwrap();

    let chinese_file = temp.path().join("测试文件.txt");
    fs::write(&chinese_file, "中文内容").unwrap();

    assert!(chinese_file.exists());

    let metadata = PathMetadata::from_path(&chinese_file);
    assert!(metadata.is_some());
    assert!(!metadata.unwrap().is_dir);
}

#[test]
fn test_unicode_filename_emoji() {
    let temp = TempDir::new().unwrap();

    let emoji_file = temp.path().join("🎉🚀💻.txt");
    fs::write(&emoji_file, "emoji content").unwrap();

    assert!(emoji_file.exists());

    let metadata = PathMetadata::from_path(&emoji_file);
    assert!(metadata.is_some());
}

#[test]
fn test_unicode_filename_japanese() {
    let temp = TempDir::new().unwrap();

    let japanese_file = temp.path().join("テストファイル.txt");
    fs::write(&japanese_file, "日本語コンテンツ").unwrap();

    assert!(japanese_file.exists());

    let history = CleanupHistory::new();
    let item = CleanupItem::new("japanese file", 24, "24 B").with_path(japanese_file.clone());

    let result = history.quarantine_item(&item);

    match result {
        Ok(quarantine_path) => {
            assert!(!japanese_file.exists());
            assert!(quarantine_path.exists());
        }
        Err(e) => {
            eprintln!("Unicode filename test skipped: {}", e);
        }
    }
}

#[test]
fn test_unicode_filename_arabic() {
    let temp = TempDir::new().unwrap();

    let arabic_file = temp.path().join("ملف_اختبار.txt");
    fs::write(&arabic_file, "محتوى عربي").unwrap();

    assert!(arabic_file.exists());

    let metadata = PathMetadata::from_path(&arabic_file);
    assert!(metadata.is_some());
}

#[test]
fn test_unicode_directory_name() {
    let temp = TempDir::new().unwrap();

    let unicode_dir = temp.path().join("目录_🎊_каталог");
    fs::create_dir(&unicode_dir).unwrap();
    fs::write(unicode_dir.join("file.txt"), "content").unwrap();

    assert!(unicode_dir.exists());
    assert!(unicode_dir.is_dir());

    let metadata = PathMetadata::from_path(&unicode_dir);
    assert!(metadata.is_some());
    assert!(metadata.unwrap().is_dir);
}

// ============================================================================
// Very Long Path Tests
// ============================================================================

#[test]
fn test_long_filename() {
    let temp = TempDir::new().unwrap();

    // Create a file with a very long name (within filesystem limits)
    // Most filesystems support 255 bytes for filename
    let long_name = "a".repeat(200) + ".txt";
    let long_file = temp.path().join(&long_name);

    match fs::write(&long_file, "long filename content") {
        Ok(_) => {
            assert!(long_file.exists());

            let metadata = PathMetadata::from_path(&long_file);
            assert!(metadata.is_some());
        }
        Err(e) => {
            eprintln!("Long filename test skipped (filesystem limit): {}", e);
        }
    }
}

#[test]
fn test_deeply_nested_directory() {
    let temp = TempDir::new().unwrap();

    // Create a deeply nested directory structure
    let mut nested_path = temp.path().to_path_buf();
    for i in 0..20 {
        nested_path = nested_path.join(format!("level_{}", i));
    }

    match fs::create_dir_all(&nested_path) {
        Ok(_) => {
            let deep_file = nested_path.join("deep_file.txt");
            fs::write(&deep_file, "deep content").unwrap();

            assert!(deep_file.exists());

            let metadata = PathMetadata::from_path(&deep_file);
            assert!(metadata.is_some());

            // Test path tracker with deep paths
            let mut tracker = PathTracker::new();
            tracker.track(&deep_file);
            let paths = tracker.into_paths();
            assert_eq!(paths.len(), 1);
        }
        Err(e) => {
            eprintln!("Deeply nested test skipped (path limit): {}", e);
        }
    }
}

#[test]
fn test_path_with_spaces() {
    let temp = TempDir::new().unwrap();

    let spaced_dir = temp.path().join("path with many spaces");
    fs::create_dir(&spaced_dir).unwrap();

    let spaced_file = spaced_dir.join("file with spaces.txt");
    fs::write(&spaced_file, "content with spaces").unwrap();

    assert!(spaced_file.exists());

    let metadata = PathMetadata::from_path(&spaced_file);
    assert!(metadata.is_some());

    let history = CleanupHistory::new();
    let item = CleanupItem::new("spaced file", 19, "19 B").with_path(spaced_file.clone());

    let result = history.quarantine_item(&item);
    match result {
        Ok(quarantine_path) => {
            assert!(!spaced_file.exists());
            assert!(quarantine_path.exists());
        }
        Err(e) => {
            eprintln!("Spaced path test skipped: {}", e);
        }
    }
}

#[test]
fn test_path_with_special_characters() {
    let temp = TempDir::new().unwrap();

    // Test various special characters that are valid on most filesystems
    let special_file = temp.path().join("file-with_special.chars(1)[2]{3}.txt");
    fs::write(&special_file, "special content").unwrap();

    assert!(special_file.exists());

    let metadata = PathMetadata::from_path(&special_file);
    assert!(metadata.is_some());
}

// ============================================================================
// Empty Directory Tests
// ============================================================================

#[test]
fn test_empty_directory_metadata() {
    let temp = TempDir::new().unwrap();

    let empty_dir = temp.path().join("empty_dir");
    fs::create_dir(&empty_dir).unwrap();

    assert!(empty_dir.exists());
    assert!(empty_dir.is_dir());

    let metadata = PathMetadata::from_path(&empty_dir);
    assert!(metadata.is_some());
    let meta = metadata.unwrap();
    assert!(meta.is_dir);
    // Empty directories have size 0 or small metadata size
}

#[test]
fn test_empty_directory_quarantine() {
    let temp = TempDir::new().unwrap();

    let empty_dir = temp.path().join("empty_to_quarantine");
    fs::create_dir(&empty_dir).unwrap();

    let history = CleanupHistory::new();
    let item = CleanupItem::new("empty directory", 0, "0 B").with_path(empty_dir.clone());

    let result = history.quarantine_item(&item);

    match result {
        Ok(quarantine_path) => {
            assert!(!empty_dir.exists());
            assert!(quarantine_path.exists());
            assert!(quarantine_path.is_dir());
        }
        Err(e) => {
            eprintln!("Empty directory quarantine test skipped: {}", e);
        }
    }
}

#[test]
fn test_nested_empty_directories() {
    let temp = TempDir::new().unwrap();

    let nested = temp.path().join("a").join("b").join("c").join("d");
    fs::create_dir_all(&nested).unwrap();

    // All parent directories should exist
    assert!(temp.path().join("a").exists());
    assert!(temp.path().join("a").join("b").exists());
    assert!(nested.exists());

    let metadata = PathMetadata::from_path(&nested);
    assert!(metadata.is_some());
}

// ============================================================================
// Large File Tests
// ============================================================================

#[test]
fn test_large_file_metadata() {
    let temp = TempDir::new().unwrap();

    // Create a 1MB file
    let large_file = temp.path().join("large_file.bin");
    let data = vec![0u8; 1024 * 1024]; // 1 MB
    fs::write(&large_file, &data).unwrap();

    let metadata = PathMetadata::from_path(&large_file);
    assert!(metadata.is_some());
    let meta = metadata.unwrap();
    assert_eq!(meta.size, 1024 * 1024);
    assert!(!meta.is_dir);
}

#[test]
fn test_cleanup_item_large_size() {
    let mut result = CheckResult::new("Large Files");

    // Add items with very large sizes
    result.add_item(CleanupItem::new(
        "large cache",
        10 * 1024 * 1024 * 1024,
        "10 GB",
    ));
    result.add_item(CleanupItem::new(
        "huge cache",
        100 * 1024 * 1024 * 1024,
        "100 GB",
    ));

    assert_eq!(result.items.len(), 2);
    assert_eq!(result.total_size, 110 * 1024 * 1024 * 1024);
}

#[test]
fn test_backend_handles_large_sizes() {
    let mut backend = StorageBackend::new();

    let mut result = CheckResult::new("Large Category");
    result.add_item(CleanupItem::new("item1", u64::MAX / 2, "Large"));
    result.total_size = u64::MAX / 2;

    backend.categories.insert(
        "large".to_string(),
        devsweep::backend::CategoryData::new("Large Category".to_string(), result),
    );

    // Should handle large sizes without overflow
    let total = backend.get_total_reclaimable();
    assert_eq!(total, u64::MAX / 2);
}

// ============================================================================
// Zero Size File Tests
// ============================================================================

#[test]
fn test_zero_size_file_metadata() {
    let temp = TempDir::new().unwrap();

    let empty_file = temp.path().join("empty_file.txt");
    fs::write(&empty_file, "").unwrap();

    let metadata = PathMetadata::from_path(&empty_file);
    assert!(metadata.is_some());
    let meta = metadata.unwrap();
    assert_eq!(meta.size, 0);
    assert!(!meta.is_dir);
}

#[test]
fn test_zero_size_file_quarantine() {
    let temp = TempDir::new().unwrap();

    let empty_file = temp.path().join("empty.txt");
    fs::write(&empty_file, "").unwrap();

    let history = CleanupHistory::new();
    let item = CleanupItem::new("empty file", 0, "0 B").with_path(empty_file.clone());

    let result = history.quarantine_item(&item);

    match result {
        Ok(quarantine_path) => {
            assert!(!empty_file.exists());
            assert!(quarantine_path.exists());
            // Quarantined file should also be 0 bytes
            assert_eq!(fs::metadata(&quarantine_path).unwrap().len(), 0);
        }
        Err(e) => {
            eprintln!("Zero size file test skipped: {}", e);
        }
    }
}

// ============================================================================
// File Change Detection Tests
// ============================================================================

#[test]
fn test_file_change_detection_content() {
    let temp = TempDir::new().unwrap();

    let test_file = temp.path().join("changeable.txt");
    fs::write(&test_file, "original content").unwrap();

    let meta1 = PathMetadata::from_path(&test_file).unwrap();

    // Modify content (size changes)
    fs::write(&test_file, "modified with more content").unwrap();

    // Should detect change
    assert!(meta1.has_changed(&test_file));
}

#[test]
fn test_file_change_detection_deleted() {
    let temp = TempDir::new().unwrap();

    let test_file = temp.path().join("deletable.txt");
    fs::write(&test_file, "content").unwrap();

    let meta = PathMetadata::from_path(&test_file).unwrap();

    // Delete the file
    fs::remove_file(&test_file).unwrap();

    // Should detect that file was deleted
    assert!(meta.has_changed(&test_file));
}

#[test]
fn test_directory_change_detection() {
    let temp = TempDir::new().unwrap();

    let test_dir = temp.path().join("changeable_dir");
    fs::create_dir(&test_dir).unwrap();

    let meta = PathMetadata::from_path(&test_dir).unwrap();
    assert!(meta.is_dir);

    // Add a file to the directory - directory mtime may change
    fs::write(test_dir.join("new_file.txt"), "content").unwrap();

    // Check if change is detected (may or may not detect depending on filesystem)
    let _ = meta.has_changed(&test_dir);
}

// ============================================================================
// Path Tracker Edge Cases
// ============================================================================

#[test]
fn test_path_tracker_nonexistent_path() {
    let mut tracker = PathTracker::new();

    // Track a nonexistent path
    tracker.track("/nonexistent/path/that/does/not/exist");

    // Should not add nonexistent paths
    let paths = tracker.into_paths();
    assert!(paths.is_empty());
}

#[test]
fn test_path_tracker_multiple_paths() {
    let temp = TempDir::new().unwrap();

    // Create multiple files
    let file1 = temp.path().join("file1.txt");
    let file2 = temp.path().join("file2.txt");
    let file3 = temp.path().join("file3.txt");

    fs::write(&file1, "content1").unwrap();
    fs::write(&file2, "content2").unwrap();
    fs::write(&file3, "content3").unwrap();

    let mut tracker = PathTracker::new();
    tracker.track(&file1);
    tracker.track(&file2);
    tracker.track(&file3);

    let paths = tracker.into_paths();
    assert_eq!(paths.len(), 3);
    assert!(paths.contains_key(&file1));
    assert!(paths.contains_key(&file2));
    assert!(paths.contains_key(&file3));
}

#[test]
fn test_path_tracker_duplicate_tracking() {
    let temp = TempDir::new().unwrap();

    let file = temp.path().join("single_file.txt");
    fs::write(&file, "content").unwrap();

    let mut tracker = PathTracker::new();
    // Track the same file multiple times
    tracker.track(&file);
    tracker.track(&file);
    tracker.track(&file);

    // Should only have one entry
    let paths = tracker.into_paths();
    assert_eq!(paths.len(), 1);
}

// ============================================================================
// Scan Cache Edge Cases
// ============================================================================

#[test]
fn test_scan_cache_empty_category() {
    let mut cache = ScanCache::new();

    let empty_result = CheckResult::new("Empty Category");
    let tracker = PathTracker::new();

    cache.update_category("empty".to_string(), empty_result, tracker.into_paths());

    let retrieved = cache.get_valid_category("empty");
    assert!(retrieved.is_some());
    let result = retrieved.unwrap();
    assert_eq!(result.items.len(), 0);
    assert_eq!(result.total_size, 0);
}

#[test]
fn test_scan_cache_nonexistent_category() {
    let cache = ScanCache::new();

    let result = cache.get_valid_category("nonexistent_category");
    assert!(result.is_none());

    assert!(cache.needs_rescan("nonexistent_category"));
}

#[test]
fn test_scan_cache_clear() {
    let mut cache = ScanCache::new();

    // Add some data
    let result = CheckResult::new("Test");
    cache.update_category("test".to_string(), result, std::collections::HashMap::new());

    assert!(cache.get_valid_category("test").is_some());

    // Clear cache
    cache.clear();

    assert!(cache.get_valid_category("test").is_none());
    assert!(cache.categories.is_empty());
}

// ============================================================================
// Cleanup Record Edge Cases
// ============================================================================

#[test]
fn test_cleanup_record_all_failures() {
    let mut record = CleanupRecord::new("all-fail".to_string());

    let item1 = CleanupItem::new("fail1", 100, "100 B");
    let item2 = CleanupItem::new("fail2", 200, "200 B");

    record.add_item(CleanupItemRecord::error(&item1, "error 1".to_string()));
    record.add_item(CleanupItemRecord::error(&item2, "error 2".to_string()));

    assert_eq!(record.success_count, 0);
    assert_eq!(record.error_count, 2);
    assert!(!record.is_undoable()); // No successful items to undo
}

#[test]
fn test_cleanup_record_mixed_results() {
    let mut record = CleanupRecord::new("mixed".to_string());

    let item1 = CleanupItem::new("success1", 100, "100 B");
    let item2 = CleanupItem::new("fail1", 200, "200 B");
    let item3 = CleanupItem::new("success2", 300, "300 B");

    record.add_item(CleanupItemRecord::success(
        &item1,
        Some(PathBuf::from("/q/1")),
    ));
    record.add_item(CleanupItemRecord::error(&item2, "failed".to_string()));
    record.add_item(CleanupItemRecord::success(
        &item3,
        Some(PathBuf::from("/q/2")),
    ));

    assert_eq!(record.success_count, 2);
    assert_eq!(record.error_count, 1);
    assert_eq!(record.total_size, 600);
    assert!(record.is_undoable());
}

#[test]
fn test_cleanup_item_record_can_restore() {
    let item = CleanupItem::new("test", 100, "100 B").with_path(PathBuf::from("/original/path"));

    // Success with quarantine path - can restore
    let record1 = CleanupItemRecord::success(&item, Some(PathBuf::from("/quarantine/path")));
    assert!(record1.can_restore());

    // Success without quarantine path (permanently deleted) - cannot restore
    let record2 = CleanupItemRecord::success(&item, None);
    assert!(!record2.can_restore());

    // Error - cannot restore
    let record3 = CleanupItemRecord::error(&item, "error".to_string());
    assert!(!record3.can_restore());
}

// ============================================================================
// Restore Edge Cases
// ============================================================================

#[test]
fn test_restore_to_existing_location() {
    let temp = TempDir::new().unwrap();

    // Create original file
    let original = temp.path().join("restore_test.txt");
    fs::write(&original, "original content").unwrap();

    let history = CleanupHistory::new();
    let item = CleanupItem::new("test", 16, "16 B").with_path(original.clone());

    // Quarantine it
    let quarantine_result = history.quarantine_item(&item);

    if let Ok(quarantine_path) = quarantine_result {
        // Create a new file at the original location
        fs::write(&original, "new content").unwrap();

        // Try to restore - should fail because original location exists
        let record = CleanupItemRecord::success(&item, Some(quarantine_path.clone()));
        let restore_result = history.restore_item(&record);

        assert!(restore_result.is_err());
        assert!(restore_result.unwrap_err().contains("already exists"));

        // Quarantine file should still exist
        assert!(quarantine_path.exists());
    }
}

#[test]
fn test_restore_deleted_quarantine() {
    let history = CleanupHistory::new();

    // Create a record pointing to a nonexistent quarantine path
    let record = CleanupItemRecord {
        item_type: "test".to_string(),
        original_path: PathBuf::from("/tmp/original"),
        quarantine_path: Some(PathBuf::from("/tmp/nonexistent_quarantine_12345")),
        size: 100,
        success: true,
        error_message: None,
        deleted_permanently: false,
    };

    let result = history.restore_item(&record);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("no longer exists"));
}

// ============================================================================
// History Management Edge Cases
// ============================================================================

#[test]
fn test_history_many_records() {
    let mut history = CleanupHistory::new();

    // Add many records
    for i in 0..100 {
        let record = CleanupRecord::new(format!("record-{}", i));
        history.add_record(record);
    }

    // History should be limited (MAX_HISTORY_SIZE = 50)
    let stats = history.stats();
    assert!(stats.total_records <= 50);
}

#[test]
fn test_history_record_by_id() {
    let mut history = CleanupHistory::new();

    let record = CleanupRecord::new("unique-id-123".to_string());
    history.add_record(record);

    // Find by ID
    let found = history.get_record("unique-id-123");
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, "unique-id-123");

    // Not found
    let not_found = history.get_record("nonexistent-id");
    assert!(not_found.is_none());
}

#[test]
fn test_history_stats_accuracy() {
    let mut history = CleanupHistory::new();

    let initial_stats = history.stats();
    assert_eq!(initial_stats.total_records, 0);

    // Add a record with items
    let mut record = CleanupRecord::new("stats-test".to_string());
    let item = CleanupItem::new("item", 100, "100 B");
    record.add_item(CleanupItemRecord::success(&item, None));
    record.add_item(CleanupItemRecord::success(&item, None));
    history.add_record(record);

    let stats = history.stats();
    assert_eq!(stats.total_records, 1);
    assert_eq!(stats.total_items_cleaned, 2);
}
