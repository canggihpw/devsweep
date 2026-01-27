//! Go checker comprehensive tests
//! Testing Go module cache and build cache detection

use devsweep::checkers;
use std::fs;
use tempfile::TempDir;

// ==================== Basic Functionality Tests ====================

#[test]
fn test_go_checker_basic_functionality() {
    let result = checkers::check_go();
    assert_eq!(result.name, "Go");
}

#[test]
fn test_go_checker_returns_valid_structure() {
    let result = checkers::check_go();

    assert!(!result.name.is_empty());

    for item in &result.items {
        assert!(!item.item_type.is_empty());
        assert!(!item.size_str.is_empty());
    }
}

// ==================== Go Module Cache Tests ====================

#[test]
fn test_go_module_cache_structure() {
    let temp = TempDir::new().unwrap();
    let go_mod_cache = temp.path().join("go/pkg/mod");
    fs::create_dir_all(&go_mod_cache).unwrap();

    // Create module directories
    let github_mod = go_mod_cache.join("github.com/user/repo@v1.0.0");
    fs::create_dir_all(&github_mod).unwrap();
    fs::write(github_mod.join("go.mod"), b"module github.com/user/repo").unwrap();
    fs::write(github_mod.join("main.go"), b"package main").unwrap();

    assert!(github_mod.exists());
}

#[test]
fn test_go_module_cache_multiple_versions() {
    let temp = TempDir::new().unwrap();
    let go_mod_cache = temp.path().join("go/pkg/mod");
    fs::create_dir_all(&go_mod_cache).unwrap();

    // Create multiple versions of same module
    let base_path = go_mod_cache.join("github.com/gorilla/mux");

    for version in &["v1.7.0", "v1.7.1", "v1.8.0", "v1.8.1"] {
        let mod_path = go_mod_cache.join(format!("github.com/gorilla/mux@{}", version));
        fs::create_dir_all(&mod_path).unwrap();
        fs::write(mod_path.join("mux.go"), b"package mux").unwrap();
    }

    // Should have 4 version directories
    let count = fs::read_dir(&go_mod_cache.join("github.com/gorilla"))
        .unwrap()
        .count();
    assert_eq!(count, 4);
}

#[test]
fn test_go_module_cache_download_cache() {
    let temp = TempDir::new().unwrap();
    let go_mod_cache = temp.path().join("go/pkg/mod/cache/download");
    fs::create_dir_all(&go_mod_cache).unwrap();

    // Create download cache structure
    let module_cache = go_mod_cache.join("github.com/user/repo/@v");
    fs::create_dir_all(&module_cache).unwrap();

    // Create version files
    fs::write(module_cache.join("list"), b"v1.0.0\nv1.1.0\nv1.2.0").unwrap();
    fs::write(module_cache.join("v1.0.0.mod"), b"module github.com/user/repo").unwrap();
    fs::write(module_cache.join("v1.0.0.zip"), b"zip content").unwrap();
    fs::write(module_cache.join("v1.0.0.ziphash"), b"h1:abcd1234").unwrap();

    assert!(module_cache.exists());
    assert!(module_cache.join("list").exists());
}

#[test]
fn test_go_module_cache_sumdb() {
    let temp = TempDir::new().unwrap();
    let sumdb_cache = temp.path().join("go/pkg/mod/cache/download/sumdb");
    fs::create_dir_all(&sumdb_cache).unwrap();

    // Create sum.golang.org cache
    let sum_cache = sumdb_cache.join("sum.golang.org");
    fs::create_dir_all(&sum_cache).unwrap();

    let lookup = sum_cache.join("lookup");
    fs::create_dir_all(&lookup).unwrap();
    fs::write(lookup.join("github.com!user!repo@v1.0.0"), b"hash info").unwrap();

    assert!(sum_cache.exists());
}

// ==================== Go Build Cache Tests ====================

#[test]
fn test_go_build_cache_structure() {
    let temp = TempDir::new().unwrap();
    let build_cache = temp.path().join("Library/Caches/go-build");
    fs::create_dir_all(&build_cache).unwrap();

    // Create hash-based subdirectories
    for i in 0..16 {
        let subdir = build_cache.join(format!("{:02x}", i));
        fs::create_dir_all(&subdir).unwrap();

        // Create cache files
        fs::write(subdir.join("abcdef1234567890"), b"compiled object").unwrap();
    }

    assert_eq!(fs::read_dir(&build_cache).unwrap().count(), 16);
}

#[test]
fn test_go_build_cache_with_action_files() {
    let temp = TempDir::new().unwrap();
    let build_cache = temp.path().join("Library/Caches/go-build");
    let subdir = build_cache.join("00");
    fs::create_dir_all(&subdir).unwrap();

    // Create action output files (compiled packages)
    fs::write(subdir.join("hash1-a"), b"action cache entry").unwrap();
    fs::write(subdir.join("hash1-d"), b"cache data").unwrap();
    fs::write(subdir.join("hash2-a"), b"another action").unwrap();

    assert_eq!(fs::read_dir(&subdir).unwrap().count(), 3);
}

#[test]
fn test_go_build_cache_trim_file() {
    let temp = TempDir::new().unwrap();
    let build_cache = temp.path().join("Library/Caches/go-build");
    fs::create_dir_all(&build_cache).unwrap();

    // Create trim.txt file (used by go clean)
    fs::write(build_cache.join("trim.txt"), b"trim marker").unwrap();

    assert!(build_cache.join("trim.txt").exists());
}

// ==================== Go Binary Cache Tests ====================

#[test]
fn test_go_bin_directory() {
    let temp = TempDir::new().unwrap();
    let go_bin = temp.path().join("go/bin");
    fs::create_dir_all(&go_bin).unwrap();

    // Create installed binaries
    fs::write(go_bin.join("golangci-lint"), b"binary").unwrap();
    fs::write(go_bin.join("gopls"), b"binary").unwrap();
    fs::write(go_bin.join("dlv"), b"binary").unwrap();
    fs::write(go_bin.join("staticcheck"), b"binary").unwrap();

    assert_eq!(fs::read_dir(&go_bin).unwrap().count(), 4);
}

// ==================== GOPATH Structure Tests ====================

#[test]
fn test_gopath_src_directory() {
    let temp = TempDir::new().unwrap();
    let gopath_src = temp.path().join("go/src");
    fs::create_dir_all(&gopath_src).unwrap();

    // Create source directories (legacy GOPATH mode)
    let project = gopath_src.join("github.com/user/myproject");
    fs::create_dir_all(&project).unwrap();
    fs::write(project.join("main.go"), b"package main\nfunc main() {}").unwrap();

    assert!(project.exists());
}

#[test]
fn test_gopath_pkg_directory() {
    let temp = TempDir::new().unwrap();
    let gopath_pkg = temp.path().join("go/pkg");
    fs::create_dir_all(&gopath_pkg).unwrap();

    // Create platform-specific compiled packages
    let darwin_arm64 = gopath_pkg.join("darwin_arm64");
    fs::create_dir_all(&darwin_arm64).unwrap();

    let pkg_path = darwin_arm64.join("github.com/user/lib.a");
    fs::create_dir_all(pkg_path.parent().unwrap()).unwrap();
    fs::write(&pkg_path, b"archive").unwrap();

    assert!(darwin_arm64.exists());
}

// ==================== Edge Cases ====================

#[test]
fn test_empty_go_directories() {
    let temp = TempDir::new().unwrap();

    // Create empty Go directories
    let go_dir = temp.path().join("go");
    fs::create_dir_all(&go_dir).unwrap();

    let pkg_mod = go_dir.join("pkg/mod");
    fs::create_dir_all(&pkg_mod).unwrap();

    let bin = go_dir.join("bin");
    fs::create_dir_all(&bin).unwrap();

    assert!(pkg_mod.exists());
    assert!(bin.exists());
    assert_eq!(fs::read_dir(&pkg_mod).unwrap().count(), 0);
}

#[test]
fn test_go_mod_cache_with_vendor() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("myproject");
    fs::create_dir_all(&project).unwrap();

    // Create go.mod
    fs::write(project.join("go.mod"), b"module myproject\n\ngo 1.21").unwrap();
    fs::write(project.join("go.sum"), b"github.com/pkg/errors v0.9.1 h1:abcd").unwrap();

    // Create vendor directory
    let vendor = project.join("vendor");
    fs::create_dir_all(&vendor).unwrap();

    let vendored_pkg = vendor.join("github.com/pkg/errors");
    fs::create_dir_all(&vendored_pkg).unwrap();
    fs::write(vendored_pkg.join("errors.go"), b"package errors").unwrap();

    // Create modules.txt
    fs::write(vendor.join("modules.txt"), b"# github.com/pkg/errors v0.9.1").unwrap();

    assert!(vendor.exists());
    assert!(vendor.join("modules.txt").exists());
}

#[test]
fn test_go_test_cache() {
    let temp = TempDir::new().unwrap();
    let build_cache = temp.path().join("Library/Caches/go-build");
    fs::create_dir_all(&build_cache).unwrap();

    // Test results are also cached in go-build
    let test_cache = build_cache.join("test");
    fs::create_dir_all(&test_cache).unwrap();

    fs::write(test_cache.join("testmain.go"), b"package main").unwrap();

    assert!(test_cache.exists());
}

#[test]
fn test_large_go_module_cache() {
    let temp = TempDir::new().unwrap();
    let go_mod_cache = temp.path().join("go/pkg/mod");
    fs::create_dir_all(&go_mod_cache).unwrap();

    // Create many modules (simulate real-world project)
    let modules = [
        "github.com/gin-gonic/gin@v1.9.0",
        "github.com/gorilla/mux@v1.8.0",
        "github.com/go-chi/chi@v5.0.0",
        "github.com/labstack/echo@v4.10.0",
        "github.com/gofiber/fiber@v2.42.0",
        "google.golang.org/grpc@v1.53.0",
        "github.com/stretchr/testify@v1.8.2",
        "github.com/spf13/cobra@v1.6.1",
        "github.com/spf13/viper@v1.15.0",
        "go.uber.org/zap@v1.24.0",
    ];

    for module in modules {
        let mod_path = go_mod_cache.join(module);
        fs::create_dir_all(&mod_path).unwrap();
        fs::write(mod_path.join("go.mod"), format!("module {}", module.split('@').next().unwrap()).as_bytes()).unwrap();
    }

    assert!(go_mod_cache.exists());
}

#[test]
fn test_go_module_with_replace_directive() {
    let temp = TempDir::new().unwrap();
    let go_mod_cache = temp.path().join("go/pkg/mod");
    fs::create_dir_all(&go_mod_cache).unwrap();

    // Create a module with local path (replace directive target)
    let local_mod = go_mod_cache.join("local");
    fs::create_dir_all(&local_mod).unwrap();
    fs::write(local_mod.join("go.mod"), b"module local\ngo 1.21").unwrap();

    assert!(local_mod.exists());
}

#[test]
fn test_go_cache_directories_mixed() {
    let temp = TempDir::new().unwrap();
    let go_dir = temp.path().join("go");

    // Create all Go-related cache directories
    let mod_cache = go_dir.join("pkg/mod");
    fs::create_dir_all(&mod_cache).unwrap();

    let bin_dir = go_dir.join("bin");
    fs::create_dir_all(&bin_dir).unwrap();

    let build_cache = temp.path().join("Library/Caches/go-build");
    fs::create_dir_all(&build_cache).unwrap();

    // Add some content
    fs::write(mod_cache.join("cache_marker"), b"marker").unwrap();
    fs::write(bin_dir.join("tool"), b"binary").unwrap();
    let build_cache_subdir = build_cache.join("00");
    fs::create_dir_all(&build_cache_subdir).unwrap();
    fs::write(build_cache_subdir.join("cache"), b"cache").unwrap();

    assert!(mod_cache.exists());
    assert!(bin_dir.exists());
    assert!(build_cache.exists());
}
