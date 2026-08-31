use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir};

/// Check for Bun toolchain caches.
pub fn check_bun() -> CheckResult {
    let mut result = CheckResult::new("Bun");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    let cache_dirs = [
        home.join("Library/Caches/bun"), // global bun cache
        home.join(".bun/install/cache"), // bun install cache
    ];

    for path in cache_dirs.into_iter().filter(|p| p.exists()) {
        let size = get_dir_size(&path);
        if size > 0 {
            let item = CleanupItem::new("bun cache", size, &format_size(size))
                .with_path(path)
                .with_safe_to_delete(true)
                .with_warning("Re-downloaded on next bun install");
            result.add_item(item);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_bun_returns_valid_structure() {
        let result = check_bun();
        assert_eq!(result.name, "Bun");
        // Items may or may not be present depending on whether bun is installed.
        for item in &result.items {
            assert!(!item.item_type.is_empty());
            assert!(item.path.is_some());
        }
    }
}
