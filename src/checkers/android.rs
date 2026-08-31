use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir};

/// Check for Android/Java toolchain caches (Gradle, Android SDK cache).
pub fn check_android() -> CheckResult {
    let mut result = CheckResult::new("Android");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    // Gradle build caches (re-downloaded on next build; also regenerable).
    let gradle_caches = home.join(".gradle/caches");
    if gradle_caches.exists() {
        let size = get_dir_size(&gradle_caches);
        if size > 0 {
            let item = CleanupItem::new("Gradle caches (.gradle/caches)", size, &format_size(size))
                .with_path(gradle_caches)
                .with_safe_to_delete(true)
                .with_warning("Re-downloaded on the next Gradle build");
            result.add_item(item);
        }
    }

    // Android toolchain cache.
    let android_cache = home.join(".android/cache");
    if android_cache.exists() {
        let size = get_dir_size(&android_cache);
        if size > 0 {
            let item = CleanupItem::new("Android cache (.android/cache)", size, &format_size(size))
                .with_path(android_cache)
                .with_safe_to_delete(true);
            result.add_item(item);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_android_returns_valid_structure() {
        let result = check_android();
        assert_eq!(result.name, "Android");
        for item in &result.items {
            assert!(!item.item_type.is_empty());
            assert!(item.path.is_some());
        }
    }
}
