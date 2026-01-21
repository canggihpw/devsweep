//! Tests for checker modules

use devsweep::checkers;

#[test]
fn test_check_docker_returns_result() {
    let result = checkers::check_docker();
    assert_eq!(result.name, "Docker");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_homebrew_returns_result() {
    let result = checkers::check_homebrew();
    assert_eq!(result.name, "Homebrew");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_python_returns_result() {
    let result = checkers::check_python();
    assert_eq!(result.name, "Python");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_rust_returns_result() {
    let result = checkers::check_rust();
    assert_eq!(result.name, "Rust/Cargo");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_xcode_returns_result() {
    let result = checkers::check_xcode();
    assert_eq!(result.name, "Xcode");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_go_returns_result() {
    let result = checkers::check_go();
    assert_eq!(result.name, "Go");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_gradle_maven_returns_result() {
    let result = checkers::check_gradle_maven();
    assert_eq!(result.name, "Java Build Tools");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_ide_caches_returns_result() {
    let result = checkers::check_ide_caches();
    assert_eq!(result.name, "IDE Caches");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_shell_caches_returns_result() {
    let result = checkers::check_shell_caches();
    assert_eq!(result.name, "Shell Caches");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_db_caches_returns_result() {
    let result = checkers::check_db_caches();
    assert_eq!(result.name, "Database Caches");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_system_logs_returns_result() {
    let result = checkers::check_system_logs();
    assert_eq!(result.name, "System Logs & Crash Reports");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_browser_caches_returns_result() {
    let result = checkers::check_browser_caches();
    assert_eq!(result.name, "Browser Caches");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_node_modules_returns_result() {
    let result = checkers::check_node_modules();
    assert_eq!(result.name, "node_modules in Projects");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_general_caches_returns_result() {
    let result = checkers::check_general_caches();
    assert_eq!(result.name, "General Caches");
    assert!(result.total_size >= 0);
}

#[test]
fn test_check_trash_returns_result() {
    let result = checkers::check_trash();
    assert_eq!(result.name, "Trash");
    assert!(result.total_size >= 0);
}

#[test]
fn test_all_checkers_have_valid_structure() {
    let checkers = vec![
        checkers::check_docker(),
        checkers::check_homebrew(),
        checkers::check_npm_yarn(),
        checkers::check_python(),
        checkers::check_rust(),
        checkers::check_xcode(),
        checkers::check_go(),
        checkers::check_gradle_maven(),
        checkers::check_ide_caches(),
        checkers::check_shell_caches(),
        checkers::check_db_caches(),
        checkers::check_system_logs(),
        checkers::check_browser_caches(),
        checkers::check_node_modules(),
        checkers::check_general_caches(),
        checkers::check_trash(),
    ];

    for result in checkers {
        // Each checker must have a name
        assert!(!result.name.is_empty());

        // Total size must be non-negative
        assert!(result.total_size >= 0);

        // If there are items, verify their structure
        for item in result.items {
            assert!(!item.item_type.is_empty());
            assert!(item.size >= 0);
            assert!(!item.size_str.is_empty());
        }
    }
}
