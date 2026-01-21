//! Utils module tests

use devsweep::utils::{format_size, sort_versions};

#[test]
fn test_format_size() {
    let result = format_size(1024);
    assert!(result.contains("kiB") || result.contains("KB") || result.contains("KiB"));

    assert!(!format_size(0).is_empty());
    assert!(!format_size(512).is_empty());
    assert!(!format_size(1024 * 1024).is_empty());
}

#[test]
fn test_sort_versions() {
    let mut versions = vec![
        "1.0.0".to_string(),
        "2.0.0".to_string(),
        "1.10.0".to_string(),
        "1.2.0".to_string(),
    ];
    sort_versions(&mut versions);
    assert_eq!(versions[0], "1.0.0");
    assert_eq!(versions[1], "1.2.0");
    assert_eq!(versions[2], "1.10.0");
    assert_eq!(versions[3], "2.0.0");
}
