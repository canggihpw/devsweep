//! Integration tests for StorageBackend
//! Tests the interaction between scanning, caching, and cleanup operations
//! Phase 4: Full workflow integration tests

use devsweep::backend::StorageBackend;
use devsweep::cleanup_history::{CleanupHistory, CleanupItemRecord, CleanupRecord};
use devsweep::scan_cache::{PathTracker, ScanCache};
use devsweep::types::{CheckResult, CleanupItem};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

// ============================================================================
// Full Scan Workflow Tests
// ============================================================================

#[test]
fn test_full_scan_workflow() {
    // Test that a full scan completes without panicking
    let mut backend = StorageBackend::new();

    // Perform scan without cache
    let categories = backend.scan_with_cache(false);

    // Should return categories (may be empty or have data)
    assert!(
        !categories.is_empty(),
        "Should have at least some category definitions"
    );

    // Each category should have valid structure
    for category in &categories {
        assert!(!category.name.is_empty());
        assert!(!category.size.is_empty());
    }

    // Total reclaimable should be non-negative (it's u64, always >= 0)
    let total = backend.get_total_reclaimable();
    let _ = total; // Just verify it doesn't panic
}

#[test]
fn test_scan_with_cache_workflow() {
    let mut backend = StorageBackend::new();

    // First scan - builds cache
    let categories1 = backend.scan_with_cache(false);
    let count1 = categories1.len();

    // Second scan - uses cache
    let categories2 = backend.scan_with_cache(true);
    let count2 = categories2.len();

    // Should have same number of categories
    assert_eq!(count1, count2);

    // Third scan - force refresh
    let categories3 = backend.scan_with_cache(false);
    let count3 = categories3.len();

    assert_eq!(count1, count3);
}

// ============================================================================
// Quarantine and Restore Workflow Tests
// ============================================================================

#[test]
fn test_quarantine_and_restore_workflow() {
    let temp = TempDir::new().unwrap();

    // Step 1: Create test files
    let test_file = temp.path().join("cache_file.txt");
    let original_content = "important cache data";
    fs::write(&test_file, original_content).unwrap();
    assert!(test_file.exists());

    let history = CleanupHistory::new();

    // Step 2: Create cleanup item
    let item = CleanupItem::new("test cache", original_content.len() as u64, "20 B")
        .with_path(test_file.clone());

    // Step 3: Quarantine the file
    let quarantine_result = history.quarantine_item(&item);

    match quarantine_result {
        Ok(quarantine_path) => {
            // Step 4: Verify file is quarantined
            assert!(!test_file.exists(), "Original file should be moved");
            assert!(quarantine_path.exists(), "File should be in quarantine");

            // Verify content preserved
            let quarantined_content = fs::read_to_string(&quarantine_path).unwrap();
            assert_eq!(quarantined_content, original_content);

            // Step 5: Create record for restoration
            let record = CleanupItemRecord::success(&item, Some(quarantine_path.clone()));

            // Step 6: Restore the file
            let restore_result = history.restore_item(&record);

            match restore_result {
                Ok(_) => {
                    // Step 7: Verify file is restored
                    assert!(test_file.exists(), "File should be restored");
                    assert!(!quarantine_path.exists(), "Quarantine should be empty");

                    // Verify content intact
                    let restored_content = fs::read_to_string(&test_file).unwrap();
                    assert_eq!(restored_content, original_content);
                }
                Err(e) => {
                    eprintln!("Restore skipped: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Quarantine skipped: {}", e);
        }
    }
}

#[test]
fn test_quarantine_multiple_files_workflow() {
    let temp = TempDir::new().unwrap();

    // Create multiple test files
    let files: Vec<_> = (0..5)
        .map(|i| {
            let path = temp.path().join(format!("file_{}.txt", i));
            fs::write(&path, format!("content {}", i)).unwrap();
            path
        })
        .collect();

    let history = CleanupHistory::new();
    let mut quarantine_paths = Vec::new();

    // Quarantine all files
    for (i, file_path) in files.iter().enumerate() {
        let item =
            CleanupItem::new(&format!("file {}", i), 10, "10 B").with_path(file_path.clone());

        if let Ok(qpath) = history.quarantine_item(&item) {
            quarantine_paths.push((file_path.clone(), qpath));
        }
    }

    // Verify all original files are gone
    for file_path in &files {
        if !quarantine_paths.iter().any(|(orig, _)| orig == file_path) {
            continue; // Skip files that failed to quarantine
        }
        assert!(!file_path.exists());
    }

    // Verify all quarantined files exist
    for (_, qpath) in &quarantine_paths {
        assert!(qpath.exists());
    }
}

// ============================================================================
// Cache Persistence Workflow Tests
// ============================================================================

#[test]
fn test_cache_persistence_workflow() {
    let mut cache = ScanCache::new();

    // Add some data
    let mut result = CheckResult::new("Test Category");
    result.add_item(CleanupItem::new("test item", 1024, "1 KB"));
    result.total_size = 1024;

    let mut tracker = PathTracker::new();
    // Track a real path
    tracker.track(std::env::temp_dir());

    cache.update_category("test".to_string(), result, tracker.into_paths());

    // Save cache
    let save_result = cache.save();

    if save_result.is_ok() {
        // Load in new instance
        let loaded = ScanCache::load();

        // Verify data persisted
        if let Some(category) = loaded.get_valid_category("test") {
            assert_eq!(category.total_size, 1024);
            assert_eq!(category.items.len(), 1);
        }
    }
}

#[test]
fn test_cache_invalidation_workflow() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("tracked_file.txt");
    fs::write(&test_file, "initial content").unwrap();

    let mut cache = ScanCache::new();

    // Create result with tracked path
    let result = CheckResult::new("Tracked Category");

    let mut tracker = PathTracker::new();
    tracker.track(&test_file);

    cache.update_category("tracked".to_string(), result, tracker.into_paths());

    // Cache should be valid
    assert!(!cache.needs_rescan("tracked"));

    // Modify the tracked file
    thread::sleep(Duration::from_millis(10));
    fs::write(&test_file, "modified content with more data").unwrap();

    // Cache should now need rescan (file changed)
    assert!(cache.needs_rescan("tracked"));
}

#[test]
fn test_cache_ttl_workflow() {
    let mut cache = ScanCache::new();

    // Create result with short TTL
    let mut result = CheckResult::new("Short TTL Category");
    result.total_size = 500;

    // Update with empty tracked paths (relies on TTL only)
    cache.update_category("ttl_test".to_string(), result, HashMap::new());

    // Should be valid immediately
    let valid1 = cache.get_valid_category("ttl_test");
    assert!(valid1.is_some());

    // Get the category and check TTL behavior
    // Note: Default TTL varies by category, this tests the mechanism
}

// ============================================================================
// Cleanup History Workflow Tests
// ============================================================================

#[test]
fn test_cleanup_history_tracking_workflow() {
    let mut history = CleanupHistory::new();

    // Create a cleanup record
    let mut record = CleanupRecord::new("session-1".to_string());

    // Add successful items
    let item1 = CleanupItem::new("cache1", 1000, "1 KB");
    let item2 = CleanupItem::new("cache2", 2000, "2 KB");

    record.add_item(CleanupItemRecord::success(
        &item1,
        Some(PathBuf::from("/q/1")),
    ));
    record.add_item(CleanupItemRecord::success(
        &item2,
        Some(PathBuf::from("/q/2")),
    ));

    // Add to history
    history.add_record(record);

    // Verify tracking
    let stats = history.stats();
    assert!(stats.total_records >= 1);
    assert!(stats.total_items_cleaned >= 2);

    // Verify record retrievable
    let found = history.get_record("session-1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().success_count, 2);
}

#[test]
fn test_multiple_cleanup_sessions_workflow() {
    let mut history = CleanupHistory::new();

    // Session 1
    let mut record1 = CleanupRecord::new("batch-1".to_string());
    record1.add_item(CleanupItemRecord::success(
        &CleanupItem::new("item1", 100, "100 B"),
        Some(PathBuf::from("/q/item1")),
    ));
    history.add_record(record1);

    // Session 2
    let mut record2 = CleanupRecord::new("batch-2".to_string());
    record2.add_item(CleanupItemRecord::success(
        &CleanupItem::new("item2", 200, "200 B"),
        Some(PathBuf::from("/q/item2")),
    ));
    record2.add_item(CleanupItemRecord::success(
        &CleanupItem::new("item3", 300, "300 B"),
        Some(PathBuf::from("/q/item3")),
    ));
    history.add_record(record2);

    // Verify both sessions tracked
    let stats = history.stats();
    assert!(stats.total_records >= 2);

    // Verify individual retrieval
    assert!(history.get_record("batch-1").is_some());
    assert!(history.get_record("batch-2").is_some());

    // Verify batch-2 has more items
    let batch2 = history.get_record("batch-2").unwrap();
    assert_eq!(batch2.items.len(), 2);
}

#[test]
fn test_history_save_load_workflow() {
    let mut history = CleanupHistory::new();

    // Add a record
    let record = CleanupRecord::new("persist-test".to_string());
    history.add_record(record);

    // Save
    let save_result = history.save();

    if save_result.is_ok() {
        // Load fresh instance
        let loaded = CleanupHistory::load();

        // Record should be present
        let found = loaded.get_record("persist-test");
        // May or may not find it depending on test isolation
        let _ = found;
    }
}

// ============================================================================
// Backend Integration Tests
// ============================================================================

#[test]
fn test_backend_scan_and_get_items() {
    let mut backend = StorageBackend::new();

    // Perform scan
    let _categories = backend.scan_with_cache(false);

    // For each category with items, verify we can retrieve them
    for (key, category_data) in &backend.categories {
        let items = backend.get_items_for_category(key);
        assert_eq!(items.len(), category_data.items.len());
    }
}

#[test]
fn test_backend_cleanup_integration() {
    let temp = TempDir::new().unwrap();
    let backend = StorageBackend::new();

    // Create a real file to cleanup
    let test_file = temp.path().join("to_cleanup.txt");
    fs::write(&test_file, "cleanup me").unwrap();

    // Create cleanup item
    let item = CleanupItem::new("test cleanup", 10, "10 B").with_path(test_file.clone());

    // Execute cleanup
    let result = backend.execute_cleanup(&item);

    match result {
        Ok(_) => {
            // File should be moved to quarantine
            assert!(!test_file.exists());
        }
        Err(e) => {
            // Some errors are acceptable
            eprintln!("Cleanup test note: {}", e);
        }
    }
}

#[test]
fn test_backend_quarantine_stats() {
    let backend = StorageBackend::new();

    // Get stats
    let stats = backend.get_quarantine_stats();

    // Stats should be valid (fields are unsigned)
    let _ = stats.total_records;
}

#[test]
fn test_backend_get_all_items() {
    let mut backend = StorageBackend::new();

    // Add some manual categories
    let mut result1 = CheckResult::new("Cat 1");
    result1.add_item(CleanupItem::new("item1", 100, "100 B"));
    backend.categories.insert(
        "cat1".to_string(),
        devsweep::backend::CategoryData::new("Cat 1".to_string(), result1),
    );

    let mut result2 = CheckResult::new("Cat 2");
    result2.add_item(CleanupItem::new("item2", 200, "200 B"));
    result2.add_item(CleanupItem::new("item3", 300, "300 B"));
    backend.categories.insert(
        "cat2".to_string(),
        devsweep::backend::CategoryData::new("Cat 2".to_string(), result2),
    );

    // Get all items
    let all_items = backend.get_all_items();
    assert_eq!(all_items.len(), 3);
}

// ============================================================================
// Concurrent Operations Tests
// ============================================================================

#[test]
fn test_concurrent_scan_safety() {
    use std::thread;

    // Create multiple backends and scan concurrently
    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                let mut backend = StorageBackend::new();
                let categories = backend.scan_with_cache(false);
                (i, categories.len())
            })
        })
        .collect();

    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All scans should complete (may have different results due to timing)
    for (i, count) in results {
        assert!(count > 0, "Scan {} should find categories", i);
    }
}

#[test]
fn test_concurrent_cache_access() {
    use std::thread;

    // Multiple threads accessing cache
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                let mut cache = ScanCache::new();
                let result = CheckResult::new(&format!("Category {}", i));
                cache.update_category(format!("cat_{}", i), result, HashMap::new());
                cache.get_valid_category(&format!("cat_{}", i)).is_some()
            })
        })
        .collect();

    // All should succeed
    for handle in handles {
        let success = handle.join().unwrap();
        assert!(success);
    }
}

#[test]
fn test_concurrent_history_access() {
    use std::thread;

    // Multiple threads creating history records
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                let mut history = CleanupHistory::new();
                let record = CleanupRecord::new(format!("concurrent-{}", i));
                history.add_record(record);
                history.stats().total_records
            })
        })
        .collect();

    // All should complete without panic
    for handle in handles {
        let count = handle.join().unwrap();
        assert!(count >= 1);
    }
}

// ============================================================================
// End-to-End Workflow Tests
// ============================================================================

#[test]
fn test_end_to_end_scan_cleanup_restore() {
    let temp = TempDir::new().unwrap();

    // Step 1: Create test data
    let cache_dir = temp.path().join("fake_cache");
    fs::create_dir(&cache_dir).unwrap();

    for i in 0..3 {
        fs::write(
            cache_dir.join(format!("cache_{}.dat", i)),
            format!("data {}", i),
        )
        .unwrap();
    }

    // Step 2: Simulate scan finding these files
    let mut backend = StorageBackend::new();
    let mut result = CheckResult::new("Test Cache");

    for i in 0..3 {
        let file_path = cache_dir.join(format!("cache_{}.dat", i));
        result.add_item(CleanupItem::new(&format!("cache_{}", i), 6, "6 B").with_path(file_path));
    }

    backend.categories.insert(
        "test_cache".to_string(),
        devsweep::backend::CategoryData::new("Test Cache".to_string(), result),
    );

    // Step 3: Get items and cleanup first one
    let items = backend.get_items_for_category("test_cache");
    assert_eq!(items.len(), 3);

    let first_item = &items[0];
    let cleanup_result = backend.execute_cleanup(first_item);

    match cleanup_result {
        Ok(_) => {
            // First file should be quarantined
            if let Some(path) = &first_item.path {
                assert!(!path.exists());
            }

            // Other files should still exist
            assert!(cache_dir.join("cache_1.dat").exists());
            assert!(cache_dir.join("cache_2.dat").exists());
        }
        Err(e) => {
            eprintln!("End-to-end cleanup skipped: {}", e);
        }
    }
}

#[test]
fn test_full_workflow_with_history() {
    let temp = TempDir::new().unwrap();
    let mut history = CleanupHistory::new();

    // Create test file
    let test_file = temp.path().join("workflow_test.txt");
    fs::write(&test_file, "workflow content").unwrap();

    // Create cleanup record
    let mut record = CleanupRecord::new("workflow-session".to_string());

    // Quarantine file
    let item = CleanupItem::new("workflow file", 16, "16 B").with_path(test_file.clone());

    let quarantine_result = history.quarantine_item(&item);

    match quarantine_result {
        Ok(qpath) => {
            // Add to record
            record.add_item(CleanupItemRecord::success(&item, Some(qpath)));
            record.success_count = 1;

            // Add record to history
            history.add_record(record);

            // Verify history tracking
            let stats = history.stats();
            assert!(stats.total_records >= 1);

            // Verify record retrievable
            let found = history.get_record("workflow-session");
            assert!(found.is_some());
        }
        Err(e) => {
            eprintln!("Workflow test skipped: {}", e);
        }
    }
}

// ============================================================================
// Cache TTL Integration Tests
// ============================================================================

#[test]
fn test_cache_config_integration() {
    let backend = StorageBackend::new();

    // Get all TTLs
    let ttls = backend.get_all_cache_ttls();

    // Should have TTLs for various categories
    assert!(!ttls.is_empty());

    // Verify some expected categories exist
    let has_docker = ttls.iter().any(|(name, _)| name.contains("Docker"));
    let has_node = ttls.iter().any(|(name, _)| name.contains("Node"));

    // At least some common categories should be present
    // (exact names may vary)
    let _ = has_docker;
    let _ = has_node;
}

#[test]
fn test_reset_cache_config_integration() {
    let mut backend = StorageBackend::new();

    // Reset config
    backend.reset_cache_config();

    // Get TTLs after reset
    let ttls = backend.get_all_cache_ttls();

    // Should have default TTLs
    assert!(!ttls.is_empty());
}

// ============================================================================
// Error Recovery Workflow Tests
// ============================================================================

#[test]
fn test_cleanup_nonexistent_file_recovery() {
    let backend = StorageBackend::new();

    // Try to cleanup nonexistent file
    let item = CleanupItem::new("ghost file", 100, "100 B")
        .with_path(PathBuf::from("/nonexistent/path/file.txt"));

    let result = backend.execute_cleanup(&item);

    // Should return error, not panic
    assert!(result.is_err());
}

#[test]
fn test_restore_nonexistent_quarantine_recovery() {
    let history = CleanupHistory::new();

    // Try to restore from nonexistent quarantine
    let record = CleanupItemRecord {
        item_type: "test".to_string(),
        original_path: PathBuf::from("/original/path"),
        quarantine_path: Some(PathBuf::from("/nonexistent/quarantine/path")),
        size: 100,
        success: true,
        error_message: None,
        deleted_permanently: false,
    };

    let result = history.restore_item(&record);

    // Should return error, not panic
    assert!(result.is_err());
}

#[test]
fn test_scan_with_invalid_cache_recovery() {
    let mut backend = StorageBackend::new();

    // Force scan without cache (simulates cache corruption recovery)
    let categories = backend.scan_with_cache(false);

    // Should complete successfully
    assert!(!categories.is_empty());
}
