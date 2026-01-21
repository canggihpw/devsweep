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
            let item = CleanupItem::new("Global node_modules (/usr/local)", size, &format_size(size))
                .with_path(global_nm)
                .with_warning("Contains globally installed packages - review before cleaning");
            result.add_item(item);
        }
    }

    result
}
