//! Tests for types module

use devsweep::types::{CheckResult, CleanupItem, ItemDetail};
use std::path::PathBuf;

#[test]
fn test_cleanup_item_creation() {
    let item = CleanupItem::new("test item", 1024, "1 KB");
    assert_eq!(item.item_type, "test item");
    assert_eq!(item.size, 1024);
    assert_eq!(item.size_str, "1 KB");
    assert!(!item.safe_to_delete);
    assert!(item.path.is_none());
    assert!(item.warning.is_none());
}

#[test]
fn test_cleanup_item_with_path() {
    let path = PathBuf::from("/tmp/test");
    let item = CleanupItem::new("test", 100, "100 B").with_path(path.clone());

    assert!(item.path.is_some());
    assert_eq!(item.path.unwrap(), path);
}

#[test]
fn test_cleanup_item_with_safe_to_delete() {
    let item = CleanupItem::new("test", 100, "100 B").with_safe_to_delete(true);

    assert!(item.safe_to_delete);
}

#[test]
fn test_cleanup_item_with_warning() {
    let item = CleanupItem::new("test", 100, "100 B").with_warning("Be careful!");

    assert!(item.warning.is_some());
    assert_eq!(item.warning.unwrap(), "Be careful!");
}

#[test]
fn test_cleanup_item_with_cleanup_command() {
    let item = CleanupItem::new("test", 100, "100 B").with_cleanup_command("rm -rf /tmp/test");

    assert!(item.cleanup_command.is_some());
    assert_eq!(item.cleanup_command.unwrap(), "rm -rf /tmp/test");
}

#[test]
fn test_cleanup_item_with_details() {
    let detail = ItemDetail::new("sub-item", 50, "50 B");
    let item = CleanupItem::new("test", 100, "100 B").with_details(vec![detail]);

    assert!(item.details.is_some());
    assert_eq!(item.details.unwrap().len(), 1);
}

#[test]
fn test_cleanup_item_builder_chain() {
    let item = CleanupItem::new("test", 1000, "1 KB")
        .with_path(PathBuf::from("/tmp/test"))
        .with_safe_to_delete(true)
        .with_warning("Test warning");

    assert!(item.path.is_some());
    assert!(item.safe_to_delete);
    assert!(item.warning.is_some());
}

#[test]
fn test_item_detail_creation() {
    let detail = ItemDetail::new("detail", 256, "256 B");

    assert_eq!(detail.name, "detail");
    assert_eq!(detail.size, 256);
    assert_eq!(detail.size_str, "256 B");
    assert!(detail.path.is_none());
    assert!(detail.extra_info.is_none());
}

#[test]
fn test_item_detail_with_path() {
    let path = PathBuf::from("/test/path");
    let detail = ItemDetail::new("detail", 100, "100 B").with_path(path.clone());

    assert!(detail.path.is_some());
    assert_eq!(detail.path.unwrap(), path);
}

#[test]
fn test_item_detail_with_extra_info() {
    let detail = ItemDetail::new("detail", 100, "100 B").with_extra_info("Extra information");

    assert!(detail.extra_info.is_some());
    assert_eq!(detail.extra_info.unwrap(), "Extra information");
}

#[test]
fn test_check_result_creation() {
    let result = CheckResult::new("Test Category");

    assert_eq!(result.name, "Test Category");
    assert_eq!(result.total_size, 0);
    assert_eq!(result.items.len(), 0);
    assert!(result.status.is_none());
}

#[test]
fn test_check_result_add_item() {
    let mut result = CheckResult::new("Test");
    let item1 = CleanupItem::new("item1", 100, "100 B");
    let item2 = CleanupItem::new("item2", 200, "200 B");

    result.add_item(item1);
    result.add_item(item2);

    assert_eq!(result.items.len(), 2);
    assert_eq!(result.total_size, 300);
}

#[test]
fn test_check_result_total_size_accumulation() {
    let mut result = CheckResult::new("Test");

    for i in 1..=10 {
        let item = CleanupItem::new(&format!("item{}", i), i * 100, "");
        result.add_item(item);
    }

    // Total should be 100 + 200 + 300 + ... + 1000 = 5500
    assert_eq!(result.total_size, 5500);
    assert_eq!(result.items.len(), 10);
}
