use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir, run_command};
use std::path::PathBuf;

pub fn check_npm_yarn() -> CheckResult {
    let mut result = CheckResult::new("Node.js Package Managers");

    // npm cache
    if let Some(npm_cache) = run_command("npm", &["config", "get", "cache"]) {
        let cache_path = PathBuf::from(&npm_cache);
        if cache_path.exists() {
            let size = get_dir_size(&cache_path);
            if size > 0 {
                let item = CleanupItem::new("npm cache", size, &format_size(size))
                    .with_path(cache_path)
                    .with_safe_to_delete(true);
                result.add_item(item);
            }
        }
    }

    // yarn cache
    if let Some(yarn_cache) = run_command("yarn", &["cache", "dir"]) {
        let cache_path = PathBuf::from(&yarn_cache);
        if cache_path.exists() {
            let size = get_dir_size(&cache_path);
            if size > 0 {
                let item = CleanupItem::new("yarn cache", size, &format_size(size))
                    .with_path(cache_path)
                    .with_safe_to_delete(true);
                result.add_item(item);
            }
        }
    }

    // pnpm cache
    if let Some(home) = home_dir() {
        let pnpm_cache = home.join("Library/pnpm");
        if pnpm_cache.exists() {
            let size = get_dir_size(&pnpm_cache);
            if size > 0 {
                let item = CleanupItem::new("pnpm cache", size, &format_size(size))
                    .with_path(pnpm_cache)
                    .with_safe_to_delete(true);
                result.add_item(item);
            }
        }
    }

    // Global node_modules (Intel Mac location)
    let global_nm = PathBuf::from("/usr/local/lib/node_modules");
    if global_nm.exists() {
        let size = get_dir_size(&global_nm);
        if size > 0 {
            let item =
                CleanupItem::new("Global node_modules (/usr/local)", size, &format_size(size))
                    .with_path(global_nm)
                    .with_warning("Contains globally installed packages - review before cleaning");
            result.add_item(item);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_npm_yarn_returns_result() {
        let result = check_npm_yarn();
        assert_eq!(result.name, "Node.js Package Managers");
        // Result may or may not have items depending on system state
        // total_size is u64, always non-negative by type
    }

    #[test]
    fn test_npm_cache_detection() {
        // This test verifies the function doesn't crash
        // Actual cache presence depends on system configuration
        let _result = check_npm_yarn();

        // If npm is installed, we should get a valid result
        if let Some(_npm_cache) = run_command("npm", &["config", "get", "cache"]) {
            // Test passed - npm is available
            // items.len() is usize, always non-negative by type
        }
    }

    #[test]
    fn test_yarn_cache_detection() {
        // This test verifies the function doesn't crash
        let _result = check_npm_yarn();

        // If yarn is installed, we should get a valid result
        if let Some(_yarn_cache) = run_command("yarn", &["cache", "dir"]) {
            // Test passed - yarn is available
            // items.len() is usize, always non-negative by type
        }
    }

    #[test]
    fn test_pnpm_cache_path_construction() {
        // Test that pnpm path is correctly constructed
        if let Some(home) = home_dir() {
            let pnpm_cache = home.join("Library/pnpm");
            // Path should be valid even if it doesn't exist
            assert!(pnpm_cache.to_str().is_some());
            assert!(pnpm_cache.to_string_lossy().contains("Library/pnpm"));
        }
    }

    #[test]
    fn test_global_node_modules_path() {
        let global_nm = PathBuf::from("/usr/local/lib/node_modules");
        // Path should be valid
        assert!(global_nm.to_str().is_some());
        assert_eq!(global_nm.to_str().unwrap(), "/usr/local/lib/node_modules");
    }

    #[test]
    fn test_check_result_structure() {
        let result = check_npm_yarn();

        // Verify result structure
        assert!(!result.name.is_empty());
        assert_eq!(result.name, "Node.js Package Managers");

        // Verify all items have required fields
        for item in result.items {
            assert!(!item.item_type.is_empty());
            // Size is u64, always non-negative by type
            assert!(!item.size_str.is_empty());
        }
    }

    #[test]
    fn test_cleanup_items_have_paths() {
        let result = check_npm_yarn();

        // All items should have paths if they were detected
        for item in result.items {
            assert!(
                item.path.is_some(),
                "Item '{}' should have a path",
                item.item_type
            );
        }
    }

    #[test]
    fn test_safe_to_delete_flags() {
        let result = check_npm_yarn();

        // Check that npm, yarn, and pnpm caches are marked as safe to delete
        for item in result.items {
            if item.item_type.contains("npm cache")
                || item.item_type.contains("yarn cache")
                || item.item_type.contains("pnpm cache")
            {
                assert!(
                    item.safe_to_delete,
                    "Cache '{}' should be marked as safe to delete",
                    item.item_type
                );
            }
        }
    }

    #[test]
    fn test_global_node_modules_has_warning() {
        let result = check_npm_yarn();

        // Global node_modules should have a warning
        for item in result.items {
            if item.item_type.contains("Global node_modules") {
                assert!(
                    item.warning.is_some(),
                    "Global node_modules should have a warning"
                );
                assert!(item
                    .warning
                    .as_ref()
                    .unwrap()
                    .contains("review before cleaning"));
            }
        }
    }
}
