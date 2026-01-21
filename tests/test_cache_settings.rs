//! Cache settings module tests

use devsweep::cache_settings::CacheSettings;

#[test]
fn test_format_ttl() {
    assert_eq!(CacheSettings::format_ttl(0), "Never cached (always fresh)");
    assert_eq!(CacheSettings::format_ttl(30), "30 seconds");
    assert_eq!(CacheSettings::format_ttl(300), "5 minutes");
    assert_eq!(CacheSettings::format_ttl(3600), "1 hours");
    assert_eq!(CacheSettings::format_ttl(7200), "2 hours");
}

#[test]
fn test_presets_exist() {
    let presets = CacheSettings::get_presets();
    assert!(presets.len() >= 3);
}

#[test]
fn test_trash_always_zero() {
    let presets = CacheSettings::get_presets();
    for (_name, categories) in presets {
        for (cat_name, ttl) in categories {
            if cat_name == "Trash" {
                assert_eq!(ttl, 0, "Trash should always have TTL = 0");
            }
        }
    }
}
