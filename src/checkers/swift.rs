use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir};

/// Check for Swift Package Manager build/checkout caches.
pub fn check_swiftpm() -> CheckResult {
    let mut result = CheckResult::new("Swift Package Manager");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    // SwiftPM caches artifacts, checkouts, and repositories here.
    let spm_cache = home.join("Library/Caches/org.swift.swiftpm");
    if spm_cache.exists() {
        let size = get_dir_size(&spm_cache);
        if size > 0 {
            let item = CleanupItem::new(
                "SwiftPM caches (org.swift.swiftpm)",
                size,
                &format_size(size),
            )
            .with_path(spm_cache)
            .with_safe_to_delete(true)
            .with_warning("Re-resolved/cloned on the next Swift build");
            result.add_item(item);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_swiftpm_returns_valid_structure() {
        let result = check_swiftpm();
        assert_eq!(result.name, "Swift Package Manager");
        for item in &result.items {
            assert!(!item.item_type.is_empty());
            assert!(item.path.is_some());
        }
    }
}
