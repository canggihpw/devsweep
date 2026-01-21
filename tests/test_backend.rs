//! Backend module tests
//! Testing StorageBackend and CategoryData

use devsweep::backend::{CategoryData, StorageBackend};
use devsweep::types::{CheckResult, CleanupItem};
use std::path::PathBuf;

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
