//! Scan cache module tests
//! Testing cache TTL, invalidation, and metadata change detection

use devsweep::scan_cache::{CacheConfig, PathTracker, ScanCache};
use devsweep::types::{CheckResult, CleanupItem};
use std::collections::HashMap;
use std::fs;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn test_scan_cache_new() {
    let cache = ScanCache::new();
    assert!(cache.needs_rescan("test_category"));
}

#[test]
fn test_scan_cache_default() {
    let cache = ScanCache::default();
    assert!(cache.needs_rescan("test_category"));
}

#[test]
fn test_needs_rescan_for_new_category() {
    let cache = ScanCache::new();
    assert!(cache.needs_rescan("new_category"));
}

#[test]
fn test_update_category_and_needs_rescan() {
    let mut cache = ScanCache::new();

    // Create a check result
    let result = CheckResult::new("test_category");

    // Update category
    cache.update_category("test_category".to_string(), result, HashMap::new());

    // Should not need rescan immediately (within TTL)
    assert!(!cache.needs_rescan("test_category"));
}

#[test]
fn test_get_valid_category() {
    let mut cache = ScanCache::new();

    // Create and update a category
    let mut result = CheckResult::new("test_category");
    result.total_size = 1000;
    cache.update_category("test_category".to_string(), result, HashMap::new());

    // Should be able to get valid cached result
    let cached = cache.get_valid_category("test_category");
    assert!(cached.is_some());

    if let Some(cached_result) = cached {
        assert_eq!(cached_result.total_size, 1000);
    }
}

#[test]
fn test_get_valid_category_nonexistent() {
    let cache = ScanCache::new();
    let cached = cache.get_valid_category("nonexistent");
    assert!(cached.is_none());
}

#[test]
fn test_clear_cache() {
    let mut cache = ScanCache::new();

    // Add some categories
    cache.update_category("cat1".to_string(), CheckResult::new("cat1"), HashMap::new());
    cache.update_category("cat2".to_string(), CheckResult::new("cat2"), HashMap::new());

    // Clear cache
    cache.clear();

    // Should need rescan now
    assert!(cache.needs_rescan("cat1"));
    assert!(cache.needs_rescan("cat2"));
}

#[test]
fn test_cache_config_default() {
    let config = CacheConfig::default_config();

    // Should have TTLs for common categories
    assert!(config.get_ttl("Docker").is_some());
    assert!(config.get_ttl("Node.js/npm/yarn").is_some());
}

#[test]
fn test_cache_config_get_ttl() {
    let config = CacheConfig::default_config();

    // Trash should have 0 TTL (never cache)
    if let Some(ttl) = config.get_ttl("Trash") {
        assert_eq!(ttl, 0);
    }
}

#[test]
fn test_get_config() {
    let cache = ScanCache::new();
    let config = cache.get_config();

    // Should have some TTL settings
    assert!(config.get_ttl("Docker").is_some());
}

#[test]
fn test_set_config() {
    let mut cache = ScanCache::new();
    let mut config = CacheConfig::default_config();

    // Modify config
    config
        .category_ttls
        .insert("custom_category".to_string(), 9999);

    cache.set_config(config);

    // Verify config was updated
    let updated_config = cache.get_config();
    assert_eq!(updated_config.get_ttl("custom_category"), Some(9999));
}

#[test]
fn test_path_tracker_new() {
    let tracker = PathTracker::new();
    let paths = tracker.into_paths();
    assert!(paths.is_empty());
}

#[test]
fn test_path_tracker_track_file() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, b"test content").unwrap();

    let mut tracker = PathTracker::new();
    tracker.track(&test_file);

    let paths = tracker.into_paths();
    assert_eq!(paths.len(), 1);
    assert!(paths.contains_key(&test_file));
}

#[test]
fn test_path_tracker_track_directory() {
    let temp = TempDir::new().unwrap();

    let mut tracker = PathTracker::new();
    tracker.track(temp.path());

    let paths = tracker.into_paths();
    assert_eq!(paths.len(), 1);
}

#[test]
fn test_path_tracker_multiple_paths() {
    let temp = TempDir::new().unwrap();
    let file1 = temp.path().join("file1.txt");
    let file2 = temp.path().join("file2.txt");
    fs::write(&file1, b"content1").unwrap();
    fs::write(&file2, b"content2").unwrap();

    let mut tracker = PathTracker::new();
    tracker.track(&file1);
    tracker.track(&file2);

    let paths = tracker.into_paths();
    assert_eq!(paths.len(), 2);
}

// ============================================================================
// Phase 1: Quick Wins - Scan Cache Tests
// ============================================================================

#[test]
fn test_cache_ttl_expiration_short_duration() {
    let mut cache = ScanCache::new();

    // Create custom config with short TTL (1 second)
    let mut config = CacheConfig::default_config();
    config.category_ttls.insert("test_category".to_string(), 1);
    cache.set_config(config);

    // Update category
    let result = CheckResult::new("test_category");
    cache.update_category("test_category".to_string(), result, HashMap::new());

    // Should not need rescan immediately
    assert!(!cache.needs_rescan("test_category"));

    // Wait for TTL to expire
    thread::sleep(Duration::from_secs(2));

    // Now should need rescan due to TTL expiration
    assert!(cache.needs_rescan("test_category"));
}

#[test]
fn test_cache_ttl_not_expired_within_window() {
    let mut cache = ScanCache::new();

    // Set TTL to 10 seconds
    let mut config = CacheConfig::default_config();
    config.category_ttls.insert("test_category".to_string(), 10);
    cache.set_config(config);

    // Update category
    let result = CheckResult::new("test_category");
    cache.update_category("test_category".to_string(), result, HashMap::new());

    // Check immediately - should not need rescan
    assert!(!cache.needs_rescan("test_category"));

    // Wait 1 second (well within TTL)
    thread::sleep(Duration::from_secs(1));

    // Still should not need rescan
    assert!(!cache.needs_rescan("test_category"));
}

#[test]
fn test_multiple_categories_independent_caching() {
    let mut cache = ScanCache::new();

    // Update multiple categories
    cache.update_category(
        "nodejs".to_string(),
        CheckResult::new("nodejs"),
        HashMap::new(),
    );
    cache.update_category(
        "docker".to_string(),
        CheckResult::new("docker"),
        HashMap::new(),
    );
    cache.update_category(
        "python".to_string(),
        CheckResult::new("python"),
        HashMap::new(),
    );

    // All should be cached
    assert!(!cache.needs_rescan("nodejs"));
    assert!(!cache.needs_rescan("docker"));
    assert!(!cache.needs_rescan("python"));

    // Clear cache
    cache.clear();

    // All should need rescan
    assert!(cache.needs_rescan("nodejs"));
    assert!(cache.needs_rescan("docker"));
    assert!(cache.needs_rescan("python"));
}

#[test]
fn test_multiple_categories_independent_ttl() {
    let mut cache = ScanCache::new();

    // Set different TTLs
    let mut config = CacheConfig::default_config();
    config.category_ttls.insert("category1".to_string(), 1); // 1 second
    config.category_ttls.insert("category2".to_string(), 100); // 100 seconds
    cache.set_config(config);

    // Update both
    cache.update_category(
        "category1".to_string(),
        CheckResult::new("category1"),
        HashMap::new(),
    );
    cache.update_category(
        "category2".to_string(),
        CheckResult::new("category2"),
        HashMap::new(),
    );

    // Both cached initially
    assert!(!cache.needs_rescan("category1"));
    assert!(!cache.needs_rescan("category2"));

    // Wait for first to expire
    thread::sleep(Duration::from_secs(2));

    // First should need rescan, second should not
    assert!(cache.needs_rescan("category1"));
    assert!(!cache.needs_rescan("category2"));
}

#[test]
fn test_cache_update_resets_timer() {
    let mut cache = ScanCache::new();

    // Set short TTL
    let mut config = CacheConfig::default_config();
    config.category_ttls.insert("test_category".to_string(), 3);
    cache.set_config(config);

    // Update cache
    cache.update_category(
        "test_category".to_string(),
        CheckResult::new("test_category"),
        HashMap::new(),
    );

    // Wait 2 seconds
    thread::sleep(Duration::from_secs(2));

    // Update again (resets timer)
    cache.update_category(
        "test_category".to_string(),
        CheckResult::new("test_category"),
        HashMap::new(),
    );

    // Should not need rescan (timer was reset)
    assert!(!cache.needs_rescan("test_category"));
}

#[test]
fn test_cache_with_zero_ttl() {
    let mut cache = ScanCache::new();

    // Set TTL to 0 (expire immediately after first second)
    let mut config = CacheConfig::default_config();
    config.category_ttls.insert("test_category".to_string(), 0);
    cache.set_config(config);

    // Update cache
    cache.update_category(
        "test_category".to_string(),
        CheckResult::new("test_category"),
        HashMap::new(),
    );

    // Wait 1 second for it to expire (TTL check is age > ttl, not >=)
    thread::sleep(Duration::from_secs(1));

    // Should need rescan with 0 TTL after 1 second
    assert!(cache.needs_rescan("test_category"));
}

#[test]
fn test_cache_with_very_large_ttl() {
    let mut cache = ScanCache::new();

    // Set TTL to 1 year
    let mut config = CacheConfig::default_config();
    config
        .category_ttls
        .insert("test_category".to_string(), 365 * 24 * 60 * 60);
    cache.set_config(config);

    // Update cache
    cache.update_category(
        "test_category".to_string(),
        CheckResult::new("test_category"),
        HashMap::new(),
    );

    // Should not need rescan for a very long time
    assert!(!cache.needs_rescan("test_category"));
}

#[test]
fn test_cache_handles_many_categories() {
    let mut cache = ScanCache::new();

    // Create many categories
    for i in 0..100 {
        let category = format!("category_{}", i);
        cache.update_category(
            category.clone(),
            CheckResult::new(&category),
            HashMap::new(),
        );
    }

    // Verify all are cached
    for i in 0..100 {
        let category = format!("category_{}", i);
        assert!(!cache.needs_rescan(&category));
    }
}

#[test]
fn test_cache_with_tracked_paths() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("tracked.txt");
    fs::write(&test_file, b"content").unwrap();

    let mut cache = ScanCache::new();
    let mut tracker = PathTracker::new();
    tracker.track(&test_file);

    let result = CheckResult::new("test_category");
    cache.update_category("test_category".to_string(), result, tracker.into_paths());

    // Should be cached
    assert!(!cache.needs_rescan("test_category"));
}

#[test]
fn test_get_valid_category_with_items() {
    let mut cache = ScanCache::new();

    let mut result = CheckResult::new("test_category");
    result.add_item(CleanupItem::new("item1", 1000, "1 KB"));
    result.add_item(CleanupItem::new("item2", 2000, "2 KB"));

    cache.update_category("test_category".to_string(), result, HashMap::new());

    let cached = cache.get_valid_category("test_category");
    assert!(cached.is_some());

    if let Some(cached_result) = cached {
        assert_eq!(cached_result.items.len(), 2);
    }
}

#[test]
fn test_cache_category_names_with_special_chars() {
    let mut cache = ScanCache::new();

    // Test with various category names
    let categories = vec![
        "node-modules",
        "cargo_cache",
        "docker.cache",
        "cache with spaces",
    ];

    for category in &categories {
        cache.update_category(
            category.to_string(),
            CheckResult::new(category),
            HashMap::new(),
        );
        assert!(!cache.needs_rescan(category));
    }
}

#[test]
fn test_concurrent_updates_same_category() {
    let mut cache = ScanCache::new();

    // Multiple updates to same category
    cache.update_category("test".to_string(), CheckResult::new("test"), HashMap::new());
    cache.update_category("test".to_string(), CheckResult::new("test"), HashMap::new());
    cache.update_category("test".to_string(), CheckResult::new("test"), HashMap::new());

    // Should still be cached
    assert!(!cache.needs_rescan("test"));
}

#[test]
fn test_save_and_load_cache() {
    let mut cache = ScanCache::new();

    // Add some categories
    cache.update_category("cat1".to_string(), CheckResult::new("cat1"), HashMap::new());

    // Save should not crash
    let _ = cache.save();

    // Load new instance should not crash
    let _loaded = ScanCache::load();
}

#[test]
fn test_cache_config_save_and_load() {
    let mut config = CacheConfig::default_config();
    config.category_ttls.insert("custom".to_string(), 12345);

    // Save should not crash
    let _ = config.save();

    // Load should not crash
    let _loaded = CacheConfig::load();
}
