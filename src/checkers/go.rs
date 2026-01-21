use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir, run_command};
use std::path::PathBuf;

pub fn check_go() -> CheckResult {
    let mut result = CheckResult::new("Go");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    // Go module cache
    let go_mod_cache = home.join("go/pkg/mod");
    if go_mod_cache.exists() {
        let size = get_dir_size(&go_mod_cache);
        if size > 0 {
            let item = CleanupItem::new("Go module cache", size, &format_size(size))
                .with_path(go_mod_cache)
                .with_warning("Projects will need to re-download modules")
                .with_cleanup_command("go clean -modcache");
            result.add_item(item);
        }
    }

    // Go build cache
    if let Some(go_cache) = run_command("go", &["env", "GOCACHE"]) {
        let cache_path = PathBuf::from(&go_cache);
        if cache_path.exists() {
            let size = get_dir_size(&cache_path);
            if size > 0 {
                let item = CleanupItem::new("Go build cache", size, &format_size(size))
                    .with_path(cache_path)
                    .with_safe_to_delete(true)
                    .with_cleanup_command("go clean -cache");
                result.add_item(item);
            }
        }
    }

    result
}
