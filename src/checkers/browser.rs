use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir};

pub fn check_browser_caches() -> CheckResult {
    let mut result = CheckResult::new("Browser Caches");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    // Safari
    let safari_caches = [
        ("Safari Cache", home.join("Library/Caches/com.apple.Safari")),
        (
            "Safari Webpage Previews",
            home.join("Library/Caches/com.apple.Safari/Webpage Previews"),
        ),
        (
            "Safari Service Worker",
            home.join("Library/Containers/com.apple.Safari/Data/Library/Caches"),
        ),
    ];

    for (name, path) in safari_caches {
        if path.exists() {
            let size = get_dir_size(&path);
            if size > 10 * 1024 * 1024 {
                // > 10MB
                let item = CleanupItem::new(&format!("Safari: {}", name), size, &format_size(size))
                    .with_path(path)
                    .with_safe_to_delete(true)
                    .with_warning("Safari will rebuild cache on next use");
                result.add_item(item);
            }
        }
    }

    // Google Chrome
    let chrome_base = home.join("Library/Application Support/Google/Chrome");
    if chrome_base.exists() {
        check_chromium_browser(&mut result, &chrome_base, "Chrome");
    }

    // Google Chrome Canary
    let chrome_canary_base = home.join("Library/Application Support/Google/Chrome Canary");
    if chrome_canary_base.exists() {
        check_chromium_browser(&mut result, &chrome_canary_base, "Chrome Canary");
    }

    // Brave Browser
    let brave_base = home.join("Library/Application Support/BraveSoftware/Brave-Browser");
    if brave_base.exists() {
        check_chromium_browser(&mut result, &brave_base, "Brave");
    }

    // Microsoft Edge
    let edge_base = home.join("Library/Application Support/Microsoft Edge");
    if edge_base.exists() {
        check_chromium_browser(&mut result, &edge_base, "Edge");
    }

    // Vivaldi
    let vivaldi_base = home.join("Library/Application Support/Vivaldi");
    if vivaldi_base.exists() {
        check_chromium_browser(&mut result, &vivaldi_base, "Vivaldi");
    }

    // Opera
    let opera_base = home.join("Library/Application Support/com.operasoftware.Opera");
    if opera_base.exists() {
        check_chromium_browser(&mut result, &opera_base, "Opera");
    }

    // Arc Browser
    let arc_base = home.join("Library/Application Support/Arc");
    if arc_base.exists() {
        check_chromium_browser(&mut result, &arc_base, "Arc");
    }

    // Firefox
    let firefox_base = home.join("Library/Application Support/Firefox/Profiles");
    if firefox_base.exists() {
        check_firefox_browser(&mut result, &firefox_base, "Firefox");
    }

    // Firefox Developer Edition
    let firefox_dev_base =
        home.join("Library/Application Support/Firefox Developer Edition/Profiles");
    if firefox_dev_base.exists() {
        check_firefox_browser(&mut result, &firefox_dev_base, "Firefox Dev");
    }

    // Firefox Nightly
    let firefox_nightly_base = home.join("Library/Application Support/Firefox Nightly/Profiles");
    if firefox_nightly_base.exists() {
        check_firefox_browser(&mut result, &firefox_nightly_base, "Firefox Nightly");
    }

    // Chromium cache in Library/Caches
    let chrome_cache = home.join("Library/Caches/Google/Chrome");
    if chrome_cache.exists() {
        let size = get_dir_size(&chrome_cache);
        if size > 10 * 1024 * 1024 {
            let item = CleanupItem::new("Chrome: System Cache", size, &format_size(size))
                .with_path(chrome_cache)
                .with_safe_to_delete(true);
            result.add_item(item);
        }
    }

    let brave_cache = home.join("Library/Caches/BraveSoftware/Brave-Browser");
    if brave_cache.exists() {
        let size = get_dir_size(&brave_cache);
        if size > 10 * 1024 * 1024 {
            let item = CleanupItem::new("Brave: System Cache", size, &format_size(size))
                .with_path(brave_cache)
                .with_safe_to_delete(true);
            result.add_item(item);
        }
    }

    let edge_cache = home.join("Library/Caches/Microsoft Edge");
    if edge_cache.exists() {
        let size = get_dir_size(&edge_cache);
        if size > 10 * 1024 * 1024 {
            let item = CleanupItem::new("Edge: System Cache", size, &format_size(size))
                .with_path(edge_cache)
                .with_safe_to_delete(true);
            result.add_item(item);
        }
    }

    let firefox_cache = home.join("Library/Caches/Firefox");
    if firefox_cache.exists() {
        let size = get_dir_size(&firefox_cache);
        if size > 10 * 1024 * 1024 {
            let item = CleanupItem::new("Firefox: System Cache", size, &format_size(size))
                .with_path(firefox_cache)
                .with_safe_to_delete(true);
            result.add_item(item);
        }
    }

    result
}

fn check_chromium_browser(
    result: &mut CheckResult,
    base_path: &std::path::Path,
    browser_name: &str,
) {
    use std::fs;

    // Check Default profile and numbered profiles
    let profiles_to_check = [
        "Default",
        "Profile 1",
        "Profile 2",
        "Profile 3",
        "Profile 4",
        "Profile 5",
    ];

    for profile_name in profiles_to_check {
        let profile_path = base_path.join(profile_name);
        if !profile_path.exists() {
            continue;
        }

        // Cache directory
        let cache_path = profile_path.join("Cache");
        if cache_path.exists() {
            let size = get_dir_size(&cache_path);
            if size > 10 * 1024 * 1024 {
                let item = CleanupItem::new(
                    &format!("{}: {} Cache", browser_name, profile_name),
                    size,
                    &format_size(size),
                )
                .with_path(cache_path)
                .with_safe_to_delete(true);
                result.add_item(item);
            }
        }

        // Code Cache (JavaScript compiled cache)
        let code_cache_path = profile_path.join("Code Cache");
        if code_cache_path.exists() {
            let size = get_dir_size(&code_cache_path);
            if size > 10 * 1024 * 1024 {
                let item = CleanupItem::new(
                    &format!("{}: {} Code Cache", browser_name, profile_name),
                    size,
                    &format_size(size),
                )
                .with_path(code_cache_path)
                .with_safe_to_delete(true);
                result.add_item(item);
            }
        }

        // GPUCache
        let gpu_cache_path = profile_path.join("GPUCache");
        if gpu_cache_path.exists() {
            let size = get_dir_size(&gpu_cache_path);
            if size > 5 * 1024 * 1024 {
                let item = CleanupItem::new(
                    &format!("{}: {} GPU Cache", browser_name, profile_name),
                    size,
                    &format_size(size),
                )
                .with_path(gpu_cache_path)
                .with_safe_to_delete(true);
                result.add_item(item);
            }
        }

        // Service Worker cache
        let sw_cache_path = profile_path.join("Service Worker/CacheStorage");
        if sw_cache_path.exists() {
            let size = get_dir_size(&sw_cache_path);
            if size > 10 * 1024 * 1024 {
                let item = CleanupItem::new(
                    &format!("{}: {} Service Worker Cache", browser_name, profile_name),
                    size,
                    &format_size(size),
                )
                .with_path(sw_cache_path)
                .with_safe_to_delete(true)
                .with_warning("Some web apps may need to re-download data");
                result.add_item(item);
            }
        }
    }

    // Also check for dynamically named profiles
    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("Profile ") && !profiles_to_check.contains(&name.as_str()) {
                let profile_path = entry.path();

                let cache_path = profile_path.join("Cache");
                if cache_path.exists() {
                    let size = get_dir_size(&cache_path);
                    if size > 10 * 1024 * 1024 {
                        let item = CleanupItem::new(
                            &format!("{}: {} Cache", browser_name, name),
                            size,
                            &format_size(size),
                        )
                        .with_path(cache_path)
                        .with_safe_to_delete(true);
                        result.add_item(item);
                    }
                }
            }
        }
    }
}

fn check_firefox_browser(
    result: &mut CheckResult,
    profiles_path: &std::path::Path,
    browser_name: &str,
) {
    use std::fs;

    if let Ok(entries) = fs::read_dir(profiles_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let profile_path = entry.path();
            if !profile_path.is_dir() {
                continue;
            }

            let profile_name = entry.file_name().to_string_lossy().to_string();
            // Firefox profile names are like "xxxxxxxx.default" or "xxxxxxxx.default-release"
            let display_name = if let Some(pos) = profile_name.find('.') {
                &profile_name[pos + 1..]
            } else {
                &profile_name
            };

            // cache2 directory (main cache)
            let cache_path = profile_path.join("cache2");
            if cache_path.exists() {
                let size = get_dir_size(&cache_path);
                if size > 10 * 1024 * 1024 {
                    let item = CleanupItem::new(
                        &format!("{}: {} cache", browser_name, display_name),
                        size,
                        &format_size(size),
                    )
                    .with_path(cache_path)
                    .with_safe_to_delete(true);
                    result.add_item(item);
                }
            }

            // OfflineCache
            let offline_cache_path = profile_path.join("OfflineCache");
            if offline_cache_path.exists() {
                let size = get_dir_size(&offline_cache_path);
                if size > 5 * 1024 * 1024 {
                    let item = CleanupItem::new(
                        &format!("{}: {} offline cache", browser_name, display_name),
                        size,
                        &format_size(size),
                    )
                    .with_path(offline_cache_path)
                    .with_safe_to_delete(true);
                    result.add_item(item);
                }
            }

            // storage/default (IndexedDB, localStorage for websites)
            let storage_path = profile_path.join("storage/default");
            if storage_path.exists() {
                let size = get_dir_size(&storage_path);
                if size > 50 * 1024 * 1024 {
                    let item = CleanupItem::new(
                        &format!("{}: {} site storage", browser_name, display_name),
                        size,
                        &format_size(size),
                    )
                    .with_path(storage_path)
                    .with_warning("Web apps may lose offline data");
                    result.add_item(item);
                }
            }

            // startupCache
            let startup_cache_path = profile_path.join("startupCache");
            if startup_cache_path.exists() {
                let size = get_dir_size(&startup_cache_path);
                if size > 5 * 1024 * 1024 {
                    let item = CleanupItem::new(
                        &format!("{}: {} startup cache", browser_name, display_name),
                        size,
                        &format_size(size),
                    )
                    .with_path(startup_cache_path)
                    .with_safe_to_delete(true);
                    result.add_item(item);
                }
            }
        }
    }
}
