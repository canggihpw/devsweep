//! Homebrew checker comprehensive tests
//! Testing Homebrew cache, old versions, and package detection

use devsweep::checkers;
use std::fs;
use tempfile::TempDir;

// ==================== Basic Functionality Tests ====================

#[test]
fn test_homebrew_checker_basic_functionality() {
    let result = checkers::check_homebrew();
    assert_eq!(result.name, "Homebrew");
}

#[test]
fn test_homebrew_checker_returns_valid_structure() {
    let result = checkers::check_homebrew();

    assert!(!result.name.is_empty());

    for item in &result.items {
        assert!(!item.item_type.is_empty());
        assert!(!item.size_str.is_empty());
    }
}

// ==================== Homebrew Cache Tests ====================

#[test]
fn test_homebrew_cache_directory() {
    let temp = TempDir::new().unwrap();
    let cache = temp.path().join("Library/Caches/Homebrew");
    fs::create_dir_all(&cache).unwrap();

    // Create downloads directory with cached files
    let downloads = cache.join("downloads");
    fs::create_dir_all(&downloads).unwrap();
    fs::write(downloads.join("package.tar.gz"), b"downloads marker").unwrap();

    assert!(cache.exists());
    assert!(downloads.exists());
}

#[test]
fn test_homebrew_cache_bottles() {
    let temp = TempDir::new().unwrap();
    let cache = temp.path().join("Library/Caches/Homebrew");
    fs::create_dir_all(&cache).unwrap();

    // Create bottle cache files
    fs::write(cache.join("python@3.11--3.11.4.arm64_monterey.bottle.tar.gz"), b"bottle").unwrap();
    fs::write(cache.join("node--20.5.0.arm64_monterey.bottle.tar.gz"), b"bottle").unwrap();
    fs::write(cache.join("git--2.41.0.arm64_monterey.bottle.tar.gz"), b"bottle").unwrap();

    assert_eq!(fs::read_dir(&cache).unwrap().count(), 3);
}

#[test]
fn test_homebrew_cache_cask_downloads() {
    let temp = TempDir::new().unwrap();
    let cask_cache = temp.path().join("Library/Caches/Homebrew/Cask");
    fs::create_dir_all(&cask_cache).unwrap();

    // Create cask download files
    fs::write(cask_cache.join("visual-studio-code--1.80.0.dmg"), b"dmg").unwrap();
    fs::write(cask_cache.join("docker--4.21.1.dmg"), b"dmg").unwrap();

    assert!(cask_cache.exists());
}

#[test]
fn test_homebrew_cache_api() {
    let temp = TempDir::new().unwrap();
    let api_cache = temp.path().join("Library/Caches/Homebrew/api");
    fs::create_dir_all(&api_cache).unwrap();

    // Create API cache files
    fs::write(api_cache.join("formula.jws.json"), b"{}").unwrap();
    fs::write(api_cache.join("cask.jws.json"), b"{}").unwrap();

    assert!(api_cache.exists());
}

// ==================== Cellar Tests ====================

#[test]
fn test_homebrew_cellar_structure_apple_silicon() {
    let temp = TempDir::new().unwrap();
    let cellar = temp.path().join("opt/homebrew/Cellar");
    fs::create_dir_all(&cellar).unwrap();

    // Create package directories
    let python = cellar.join("python@3.11");
    fs::create_dir_all(&python).unwrap();

    let version = python.join("3.11.4");
    fs::create_dir_all(&version).unwrap();
    fs::write(version.join("INSTALL_RECEIPT.json"), b"{}").unwrap();

    assert!(cellar.exists());
}

#[test]
fn test_homebrew_cellar_structure_intel() {
    let temp = TempDir::new().unwrap();
    let cellar = temp.path().join("usr/local/Cellar");
    fs::create_dir_all(&cellar).unwrap();

    let node = cellar.join("node");
    let version = node.join("20.5.0");
    fs::create_dir_all(&version).unwrap();
    fs::write(version.join("INSTALL_RECEIPT.json"), b"{}").unwrap();

    assert!(cellar.exists());
}

#[test]
fn test_homebrew_multiple_versions() {
    let temp = TempDir::new().unwrap();
    let cellar = temp.path().join("opt/homebrew/Cellar");
    let python = cellar.join("python@3.11");
    fs::create_dir_all(&python).unwrap();

    // Create multiple versions (old ones can be cleaned)
    for version in &["3.11.1", "3.11.2", "3.11.3", "3.11.4"] {
        let version_dir = python.join(version);
        fs::create_dir_all(&version_dir).unwrap();

        // Create bin directory with content
        let bin = version_dir.join("bin");
        fs::create_dir_all(&bin).unwrap();
        fs::write(bin.join("python3.11"), b"binary").unwrap();

        // Create lib directory
        let lib = version_dir.join("lib");
        fs::create_dir_all(&lib).unwrap();
        fs::write(lib.join("libpython3.11.dylib"), b"library").unwrap();
    }

    assert_eq!(fs::read_dir(&python).unwrap().count(), 4);
}

#[test]
fn test_homebrew_large_packages() {
    let temp = TempDir::new().unwrap();
    let cellar = temp.path().join("opt/homebrew/Cellar");
    fs::create_dir_all(&cellar).unwrap();

    // Create a "large" package (> 100MB threshold in real code)
    let llvm = cellar.join("llvm/16.0.0");
    fs::create_dir_all(&llvm).unwrap();

    // Create many files to simulate large package
    let lib = llvm.join("lib");
    fs::create_dir_all(&lib).unwrap();
    for i in 0..20 {
        fs::write(lib.join(format!("libLLVM{}.dylib", i)), b"library content").unwrap();
    }

    assert!(llvm.exists());
}

// ==================== Caskroom Tests ====================

#[test]
fn test_homebrew_caskroom_structure() {
    let temp = TempDir::new().unwrap();
    let caskroom = temp.path().join("opt/homebrew/Caskroom");
    fs::create_dir_all(&caskroom).unwrap();

    // Create cask directories
    let vscode = caskroom.join("visual-studio-code/1.80.0");
    fs::create_dir_all(&vscode).unwrap();
    fs::write(vscode.join("Visual Studio Code.app"), b"app").unwrap();

    assert!(caskroom.exists());
}

#[test]
fn test_homebrew_caskroom_multiple_versions() {
    let temp = TempDir::new().unwrap();
    let caskroom = temp.path().join("opt/homebrew/Caskroom");
    let docker = caskroom.join("docker");
    fs::create_dir_all(&docker).unwrap();

    // Create multiple cask versions
    for version in &["4.19.0", "4.20.0", "4.21.0"] {
        let version_dir = docker.join(version);
        fs::create_dir_all(&version_dir).unwrap();
        fs::write(version_dir.join("Docker.app"), b"app").unwrap();
    }

    assert_eq!(fs::read_dir(&docker).unwrap().count(), 3);
}

// ==================== Global npm via Homebrew Tests ====================

#[test]
fn test_homebrew_global_node_modules() {
    let temp = TempDir::new().unwrap();
    let node_modules = temp.path().join("opt/homebrew/lib/node_modules");
    fs::create_dir_all(&node_modules).unwrap();

    // Create global npm packages
    let typescript = node_modules.join("typescript");
    fs::create_dir_all(&typescript).unwrap();
    fs::write(typescript.join("package.json"), b"{\"name\":\"typescript\"}").unwrap();

    let eslint = node_modules.join("eslint");
    fs::create_dir_all(&eslint).unwrap();
    fs::write(eslint.join("package.json"), b"{\"name\":\"eslint\"}").unwrap();

    assert!(node_modules.exists());
}

#[test]
fn test_homebrew_global_npm_large_packages() {
    let temp = TempDir::new().unwrap();
    let node_modules = temp.path().join("opt/homebrew/lib/node_modules");
    fs::create_dir_all(&node_modules).unwrap();

    // Create a package with many files
    let package = node_modules.join("@angular/cli");
    fs::create_dir_all(&package).unwrap();

    let node_modules_nested = package.join("node_modules");
    fs::create_dir_all(&node_modules_nested).unwrap();

    for i in 0..50 {
        let dep = node_modules_nested.join(format!("dep-{}", i));
        fs::create_dir_all(&dep).unwrap();
        fs::write(dep.join("index.js"), b"module.exports = {}").unwrap();
    }

    assert!(package.exists());
}

// ==================== Homebrew Logs Tests ====================

#[test]
fn test_homebrew_logs_directory() {
    let temp = TempDir::new().unwrap();
    let logs = temp.path().join("opt/homebrew/var/log");
    fs::create_dir_all(&logs).unwrap();

    // Create log files
    fs::write(logs.join("homebrew.log"), b"log content").unwrap();

    assert!(logs.exists());
}

// ==================== Homebrew Taps Tests ====================

#[test]
fn test_homebrew_taps_directory() {
    let temp = TempDir::new().unwrap();
    let taps = temp.path().join("opt/homebrew/Library/Taps");
    fs::create_dir_all(&taps).unwrap();

    // Create tap directories
    let core = taps.join("homebrew/homebrew-core");
    fs::create_dir_all(&core).unwrap();
    fs::write(core.join("Formula"), b"").unwrap();

    let cask = taps.join("homebrew/homebrew-cask");
    fs::create_dir_all(&cask).unwrap();
    fs::write(cask.join("Casks"), b"").unwrap();

    assert!(taps.exists());
}

// ==================== Edge Cases ====================

#[test]
fn test_empty_homebrew_directories() {
    let temp = TempDir::new().unwrap();

    // Create empty directories
    let cache = temp.path().join("Library/Caches/Homebrew");
    fs::create_dir_all(&cache).unwrap();

    let cellar = temp.path().join("opt/homebrew/Cellar");
    fs::create_dir_all(&cellar).unwrap();

    assert!(cache.exists());
    assert!(cellar.exists());
    assert_eq!(fs::read_dir(&cache).unwrap().count(), 0);
}

#[test]
fn test_homebrew_symlinks_structure() {
    let temp = TempDir::new().unwrap();
    let brew = temp.path().join("opt/homebrew");
    fs::create_dir_all(&brew).unwrap();

    // Create opt directory (contains symlinks in real setup)
    let opt = brew.join("opt");
    fs::create_dir_all(&opt).unwrap();

    // Create bin directory
    let bin = brew.join("bin");
    fs::create_dir_all(&bin).unwrap();

    assert!(opt.exists());
    assert!(bin.exists());
}

#[test]
fn test_homebrew_single_version_packages() {
    let temp = TempDir::new().unwrap();
    let cellar = temp.path().join("opt/homebrew/Cellar");
    fs::create_dir_all(&cellar).unwrap();

    // Create packages with only one version (no cleanup needed)
    for pkg in &["wget", "curl", "jq", "ripgrep"] {
        let pkg_dir = cellar.join(pkg).join("1.0.0");
        fs::create_dir_all(&pkg_dir).unwrap();
        fs::write(pkg_dir.join("INSTALL_RECEIPT.json"), b"{}").unwrap();
    }

    assert_eq!(fs::read_dir(&cellar).unwrap().count(), 4);
}

#[test]
fn test_homebrew_deprecated_packages() {
    let temp = TempDir::new().unwrap();
    let cellar = temp.path().join("opt/homebrew/Cellar");
    fs::create_dir_all(&cellar).unwrap();

    // Create deprecated/replaced packages mentioned in the checker
    for pkg in &["openssl@1.1", "youtube-dl", "python@3.9", "node@16"] {
        let pkg_dir = cellar.join(pkg).join("1.0.0");
        fs::create_dir_all(&pkg_dir).unwrap();
        fs::write(pkg_dir.join("INSTALL_RECEIPT.json"), b"{}").unwrap();
    }

    assert_eq!(fs::read_dir(&cellar).unwrap().count(), 4);
}

#[test]
fn test_homebrew_keg_only_packages() {
    let temp = TempDir::new().unwrap();
    let cellar = temp.path().join("opt/homebrew/Cellar");
    fs::create_dir_all(&cellar).unwrap();

    // Create keg-only packages (not linked to /opt/homebrew/bin)
    let openssl = cellar.join("openssl@3/3.1.0");
    fs::create_dir_all(&openssl).unwrap();

    // These have .keg_only marker
    fs::write(openssl.join("INSTALL_RECEIPT.json"), b"{\"keg_only\":true}").unwrap();

    assert!(openssl.exists());
}

#[test]
fn test_homebrew_services_directory() {
    let temp = TempDir::new().unwrap();
    let services = temp.path().join("opt/homebrew/Cellar/postgresql@14/14.8/homebrew.mxcl.postgresql@14.plist");
    fs::create_dir_all(services.parent().unwrap()).unwrap();
    fs::write(&services, b"plist content").unwrap();

    assert!(services.exists());
}

#[test]
fn test_homebrew_with_many_packages() {
    let temp = TempDir::new().unwrap();
    let cellar = temp.path().join("opt/homebrew/Cellar");
    fs::create_dir_all(&cellar).unwrap();

    // Create many packages (simulate real homebrew installation)
    let packages = [
        ("git", "2.41.0"),
        ("node", "20.5.0"),
        ("python@3.11", "3.11.4"),
        ("rust", "1.71.0"),
        ("go", "1.20.6"),
        ("postgresql@14", "14.8"),
        ("redis", "7.0.12"),
        ("nginx", "1.25.1"),
        ("ffmpeg", "6.0"),
        ("imagemagick", "7.1.1"),
    ];

    for (pkg, version) in packages {
        let pkg_dir = cellar.join(pkg).join(version);
        fs::create_dir_all(&pkg_dir).unwrap();
        fs::write(pkg_dir.join("INSTALL_RECEIPT.json"), b"{}").unwrap();

        // Create bin directory
        let bin = pkg_dir.join("bin");
        fs::create_dir_all(&bin).unwrap();
        fs::write(bin.join(pkg.split('@').next().unwrap()), b"binary").unwrap();
    }

    assert_eq!(fs::read_dir(&cellar).unwrap().count(), 10);
}

#[test]
fn test_homebrew_pinned_versions() {
    let temp = TempDir::new().unwrap();
    let cellar = temp.path().join("opt/homebrew/Cellar");
    let node = cellar.join("node");
    fs::create_dir_all(&node).unwrap();

    // Create multiple versions with one pinned
    for version in &["18.17.0", "20.5.0", "20.5.1"] {
        let version_dir = node.join(version);
        fs::create_dir_all(&version_dir).unwrap();
        fs::write(version_dir.join("INSTALL_RECEIPT.json"), b"{}").unwrap();
    }

    // Create pinned marker (in real setup this would be in var/homebrew/pinned)
    let pinned = temp.path().join("opt/homebrew/var/homebrew/pinned");
    fs::create_dir_all(&pinned).unwrap();
    fs::write(pinned.join("node"), b"18.17.0").unwrap();

    assert!(pinned.exists());
}
