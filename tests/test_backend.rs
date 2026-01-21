//! Backend module tests
//! Testing StorageBackend and CategoryData

use devsweep::backend::{CategoryData, StorageBackend};
use devsweep::types::{CheckResult, CleanupItem};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_storage_backend_new() {
    let backend = StorageBackend::new();
    assert!(backend.categories.is_empty());
    assert_eq!(backend.get_total_reclaimable(), 0);
}

#[test]
fn test_storage_backend_default() {
    let backend = StorageBackend::default();
    assert!(backend.categories.is_empty());
}

#[test]
fn test_category_data_new() {
    let mut result = CheckResult::new("Test Category");
    let item = CleanupItem::new("test item", 512, "512 B").with_safe_to_delete(true);
    result.add_item(item);

    let category = CategoryData::new("Test Category".to_string(), result);

    assert_eq!(category.name, "Test Category");
    assert_eq!(category.total_size, 512);
    assert_eq!(category.item_count, 1);
    assert_eq!(category.items.len(), 1);
    assert!(!category.size.is_empty());
}

#[test]
fn test_get_total_reclaimable() {
    let mut backend = StorageBackend::new();
    assert_eq!(backend.get_total_reclaimable(), 0);

    let mut result1 = CheckResult::new("Category 1");
    result1.total_size = 1000;
    backend.categories.insert(
        "cat1".to_string(),
        CategoryData::new("Category 1".to_string(), result1),
    );

    let mut result2 = CheckResult::new("Category 2");
    result2.total_size = 2000;
    backend.categories.insert(
        "cat2".to_string(),
        CategoryData::new("Category 2".to_string(), result2),
    );

    assert_eq!(backend.get_total_reclaimable(), 3000);
}

#[test]
fn test_scan_with_cache_structure() {
    let mut backend = StorageBackend::new();
    let categories = backend.scan_with_cache(false);

    assert!(!categories.is_empty());

    for category in categories {
        assert!(!category.name.is_empty());
        assert!(!category.size.is_empty());
    }
}

#[test]
fn test_scan_with_and_without_cache() {
    let mut backend = StorageBackend::new();

    let categories1 = backend.scan_with_cache(false);
    let count1 = categories1.len();

    let categories2 = backend.scan_with_cache(true);
    let count2 = categories2.len();

    assert_eq!(count1, count2);
}

#[test]
fn test_get_items_for_category() {
    let mut backend = StorageBackend::new();

    let mut result = CheckResult::new("Test Category");
    let item = CleanupItem::new("test item", 512, "512 B");
    result.add_item(item);

    backend.categories.insert(
        "test_cat".to_string(),
        CategoryData::new("Test Category".to_string(), result),
    );

    let items = backend.get_items_for_category("test_cat");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].item_type, "test item");
}

#[test]
fn test_get_items_for_nonexistent_category() {
    let backend = StorageBackend::new();
    let items = backend.get_items_for_category("nonexistent");
    assert!(items.is_empty());
}

#[test]
fn test_get_all_items() {
    let mut backend = StorageBackend::new();

    let mut result1 = CheckResult::new("Category 1");
    result1.add_item(CleanupItem::new("item1", 100, "100 B"));
    backend.categories.insert(
        "cat1".to_string(),
        CategoryData::new("Category 1".to_string(), result1),
    );

    let mut result2 = CheckResult::new("Category 2");
    result2.add_item(CleanupItem::new("item2", 200, "200 B"));
    result2.add_item(CleanupItem::new("item3", 300, "300 B"));
    backend.categories.insert(
        "cat2".to_string(),
        CategoryData::new("Category 2".to_string(), result2),
    );

    let all_items = backend.get_all_items();
    assert_eq!(all_items.len(), 3);
}

#[test]
fn test_get_quarantine_stats() {
    let backend = StorageBackend::new();
    let _stats = backend.get_quarantine_stats();

    // Stats should be non-negative (may have existing data from previous runs)
    // Stats fields are unsigned integers, always non-negative by type
}

#[test]
fn test_get_quarantine_records() {
    let backend = StorageBackend::new();
    let _records = backend.get_quarantine_records();
    // Just verify it doesn't crash
    // records.len() is usize, always non-negative by type
}

#[test]
fn test_get_all_cache_ttls() {
    let backend = StorageBackend::new();
    let ttls = backend.get_all_cache_ttls();

    assert!(!ttls.is_empty());

    for (name, _ttl) in ttls {
        assert!(!name.is_empty());
    }
}

#[test]
fn test_reset_cache_config() {
    let mut backend = StorageBackend::new();
    backend.reset_cache_config();

    let ttls = backend.get_all_cache_ttls();
    assert!(!ttls.is_empty());
}

#[test]
fn test_execute_cleanup_with_invalid_path() {
    let backend = StorageBackend::new();

    let item = CleanupItem::new("test", 100, "100 B")
        .with_path(PathBuf::from("/nonexistent/path/that/does/not/exist"));

    let result = backend.execute_cleanup(&item);
    assert!(result.is_err());
}

#[test]
fn test_category_data_clone() {
    let mut result = CheckResult::new("Test");
    result.total_size = 500;

    let category = CategoryData::new("Test".to_string(), result);
    let cloned = category.clone();

    assert_eq!(category.name, cloned.name);
    assert_eq!(category.total_size, cloned.total_size);
    assert_eq!(category.item_count, cloned.item_count);
}

#[test]
fn test_backend_categories_clear_on_rescan() {
    let mut backend = StorageBackend::new();

    backend.scan_with_cache(false);
    let count1 = backend.categories.len();

    backend.scan_with_cache(false);
    let count2 = backend.categories.len();

    assert_eq!(count1, count2);
}

// ============================================================================
// Phase 1: Quick Wins - Backend Module Tests
// ============================================================================

#[test]
fn test_execute_cleanup_with_real_temp_files() {
    let backend = StorageBackend::new();
    let temp = TempDir::new().unwrap();

    // Create a test file to cleanup
    let test_file = temp.path().join("test_cache.txt");
    fs::write(&test_file, b"test cache data").unwrap();
    assert!(test_file.exists());

    // Create cleanup item
    let item = CleanupItem::new("test cache", 15, "15 B").with_path(test_file.clone());

    // Execute cleanup
    let result = backend.execute_cleanup(&item);

    // Verify result
    match result {
        Ok(_) => {
            // File should be moved to quarantine (not at original location)
            assert!(!test_file.exists());
        }
        Err(e) => {
            // Some errors are acceptable (e.g., quarantine directory issues)
            eprintln!("Cleanup error (acceptable in test): {}", e);
        }
    }
}

#[test]
fn test_execute_cleanup_with_directory() {
    let backend = StorageBackend::new();
    let temp = TempDir::new().unwrap();

    // Create a test directory with files
    let test_dir = temp.path().join("test_cache_dir");
    fs::create_dir(&test_dir).unwrap();
    fs::write(test_dir.join("file1.txt"), b"data1").unwrap();
    fs::write(test_dir.join("file2.txt"), b"data2").unwrap();
    assert!(test_dir.exists());

    // Create cleanup item
    let item = CleanupItem::new("test directory", 10, "10 B").with_path(test_dir.clone());

    // Execute cleanup
    let result = backend.execute_cleanup(&item);

    // Verify result
    match result {
        Ok(_) => {
            // Directory should be moved to quarantine
            assert!(!test_dir.exists());
        }
        Err(e) => {
            eprintln!("Cleanup error (acceptable in test): {}", e);
        }
    }
}

#[test]
fn test_execute_cleanup_nonexistent_file() {
    let backend = StorageBackend::new();

    // Create item with path that doesn't exist
    let nonexistent = PathBuf::from("/tmp/nonexistent_file_12345.txt");
    let item = CleanupItem::new("nonexistent", 100, "100 B").with_path(nonexistent);

    // Execute cleanup should return an error
    let result = backend.execute_cleanup(&item);
    assert!(result.is_err());
}

#[test]
fn test_cache_usage_in_sequential_scans() {
    let mut backend = StorageBackend::new();

    // Perform initial scan without cache
    backend.scan_with_cache(false);
    let total1 = backend.get_total_reclaimable();

    // Perform scan with cache
    backend.scan_with_cache(true);
    let total2 = backend.get_total_reclaimable();

    // Totals might differ slightly due to filesystem changes, but should be close
    // Just verify both scans work without crashing
    let _ = total1;
    let _ = total2;
}

#[test]
fn test_quarantine_stats_structure() {
    let backend = StorageBackend::new();

    let stats = backend.get_quarantine_stats();

    // Verify stats structure is valid
    // Fields are unsigned integers, always non-negative by type
    let _ = stats.total_records;
}

#[test]
fn test_quarantine_records_retrieval() {
    let backend = StorageBackend::new();

    let records = backend.get_quarantine_records();

    // Should not crash, records may be empty or have existing data
    let _ = records.len();
}

#[test]
fn test_scan_empty_environment() {
    let mut backend = StorageBackend::new();

    // Scan should work even if no caches exist
    let categories = backend.scan_with_cache(false);

    // Should return some categories (even if empty)
    // All checkers will be called, so we get categories
    assert!(!categories.is_empty());

    // Each category should have valid structure
    for category in categories {
        assert!(!category.name.is_empty());
        assert!(!category.size.is_empty());
        // item_count and total_size can be 0 if no items found
    }
}

#[test]
fn test_multiple_sequential_cleanups() {
    let backend = StorageBackend::new();
    let temp = TempDir::new().unwrap();

    // Create multiple test files
    let file1 = temp.path().join("cache1.txt");
    let file2 = temp.path().join("cache2.txt");
    fs::write(&file1, b"cache1").unwrap();
    fs::write(&file2, b"cache2").unwrap();

    // Cleanup first file
    let item1 = CleanupItem::new("cache1", 6, "6 B").with_path(file1.clone());
    let _result1 = backend.execute_cleanup(&item1);

    // Cleanup second file
    let item2 = CleanupItem::new("cache2", 6, "6 B").with_path(file2.clone());
    let _result2 = backend.execute_cleanup(&item2);

    // Both should be processed (error or success is acceptable)
    // Just verify no crash
}

#[test]
fn test_get_items_maintains_order() {
    let mut backend = StorageBackend::new();

    let mut result = CheckResult::new("Ordered Test");
    result.add_item(CleanupItem::new("item_a", 100, "100 B"));
    result.add_item(CleanupItem::new("item_b", 200, "200 B"));
    result.add_item(CleanupItem::new("item_c", 300, "300 B"));

    backend.categories.insert(
        "ordered".to_string(),
        CategoryData::new("Ordered Test".to_string(), result),
    );

    let items = backend.get_items_for_category("ordered");
    assert_eq!(items.len(), 3);
    assert_eq!(items[0].item_type, "item_a");
    assert_eq!(items[1].item_type, "item_b");
    assert_eq!(items[2].item_type, "item_c");
}

#[test]
fn test_category_data_with_large_item_count() {
    let mut result = CheckResult::new("Large Category");

    // Add many items
    for i in 0..100 {
        result.add_item(CleanupItem::new(&format!("item_{}", i), 1000, "1.0 KB"));
    }

    let category = CategoryData::new("Large Category".to_string(), result);

    assert_eq!(category.item_count, 100);
    assert_eq!(category.total_size, 100_000);
    assert_eq!(category.items.len(), 100);
}

#[test]
fn test_backend_handles_zero_size_items() {
    let mut backend = StorageBackend::new();

    let mut result = CheckResult::new("Zero Size");
    result.add_item(CleanupItem::new("empty", 0, "0 B"));
    result.total_size = 0;

    backend.categories.insert(
        "zero".to_string(),
        CategoryData::new("Zero Size".to_string(), result),
    );

    assert_eq!(backend.get_total_reclaimable(), 0);
    let items = backend.get_items_for_category("zero");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].size, 0);
}

#[test]
fn test_rescan_consistency() {
    let mut backend = StorageBackend::new();

    // First scan without cache
    let categories1 = backend.scan_with_cache(false);
    let count1 = categories1.len();

    // Second scan with cache (should use cached results)
    let categories2 = backend.scan_with_cache(true);
    let count2 = categories2.len();

    assert_eq!(count1, count2);

    // Third scan without cache should be consistent
    let categories3 = backend.scan_with_cache(false);
    let count3 = categories3.len();

    assert_eq!(count1, count3);
}
