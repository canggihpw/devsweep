use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir};

/// Check for Flutter/Dart toolchain caches.
pub fn check_flutter() -> CheckResult {
    let mut result = CheckResult::new("Flutter");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    // Dart package cache (~/.pub-cache). Regenerable via `dart pub get` /
    // `flutter pub get`.
    let pub_cache = home.join(".pub-cache");
    if pub_cache.exists() {
        let size = get_dir_size(&pub_cache);
        if size > 0 {
            let item = CleanupItem::new("Dart pub cache (.pub-cache)", size, &format_size(size))
                .with_path(pub_cache)
                .with_safe_to_delete(true)
                .with_warning("Re-downloaded on the next pub get/fetch");
            result.add_item(item);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_flutter_returns_valid_structure() {
        let result = check_flutter();
        assert_eq!(result.name, "Flutter");
        for item in &result.items {
            assert!(!item.item_type.is_empty());
            assert!(item.path.is_some());
        }
    }
}
