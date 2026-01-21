use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir};

pub fn check_xcode() -> CheckResult {
    let mut result = CheckResult::new("Xcode");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    // Derived Data
    let derived_data = home.join("Library/Developer/Xcode/DerivedData");
    if derived_data.exists() {
        let size = get_dir_size(&derived_data);
        if size > 0 {
            let item = CleanupItem::new("Derived Data", size, &format_size(size))
                .with_path(derived_data)
                .with_safe_to_delete(true);
            result.add_item(item);
        }
    }

    // Archives
    let archives = home.join("Library/Developer/Xcode/Archives");
    if archives.exists() {
        let size = get_dir_size(&archives);
        if size > 0 {
            let item = CleanupItem::new("Archives", size, &format_size(size))
                .with_path(archives)
                .with_warning("Contains app archives - review before deleting");
            result.add_item(item);
        }
    }

    // iOS Device Support
    let device_support = home.join("Library/Developer/Xcode/iOS DeviceSupport");
    if device_support.exists() {
        let size = get_dir_size(&device_support);
        if size > 0 {
            let item = CleanupItem::new("iOS Device Support", size, &format_size(size))
                .with_path(device_support)
                .with_warning("Needed for debugging on specific iOS versions");
            result.add_item(item);
        }
    }

    // watchOS Device Support
    let watch_support = home.join("Library/Developer/Xcode/watchOS DeviceSupport");
    if watch_support.exists() {
        let size = get_dir_size(&watch_support);
        if size > 0 {
            let item = CleanupItem::new("watchOS Device Support", size, &format_size(size))
                .with_path(watch_support)
                .with_warning("Needed for debugging on specific watchOS versions");
            result.add_item(item);
        }
    }

    // Simulator devices
    let simulators = home.join("Library/Developer/CoreSimulator/Devices");
    if simulators.exists() {
        let size = get_dir_size(&simulators);
        if size > 0 {
            let item = CleanupItem::new("Simulator Devices", size, &format_size(size))
                .with_path(simulators)
                .with_warning("Use 'xcrun simctl delete unavailable' to clean old simulators")
                .with_cleanup_command("xcrun simctl delete unavailable");
            result.add_item(item);
        }
    }

    // Caches
    let xcode_caches = home.join("Library/Developer/Xcode/Caches");
    if xcode_caches.exists() {
        let size = get_dir_size(&xcode_caches);
        if size > 0 {
            let item = CleanupItem::new("Xcode Caches", size, &format_size(size))
                .with_path(xcode_caches)
                .with_safe_to_delete(true);
            result.add_item(item);
        }
    }

    result
}
