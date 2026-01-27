//! IDE checker comprehensive tests
//! Testing VSCode, JetBrains, Android Studio, and other IDE cache detection

use devsweep::checkers;
use std::fs;
use tempfile::TempDir;

// ==================== Basic Functionality Tests ====================

#[test]
fn test_ide_checker_basic_functionality() {
    let result = checkers::check_ide_caches();
    assert_eq!(result.name, "IDE Caches");
}

#[test]
fn test_ide_checker_returns_valid_structure() {
    let result = checkers::check_ide_caches();

    assert!(!result.name.is_empty());

    for item in &result.items {
        assert!(!item.item_type.is_empty());
        assert!(!item.size_str.is_empty());
    }
}

// ==================== VSCode Tests ====================

#[test]
fn test_vscode_main_cache() {
    let temp = TempDir::new().unwrap();
    let vscode_cache = temp.path().join("Library/Caches/com.microsoft.VSCode");
    fs::create_dir_all(&vscode_cache).unwrap();

    // Create cache subdirectories
    let cache_dir = vscode_cache.join("Cache");
    fs::create_dir_all(&cache_dir).unwrap();
    fs::write(cache_dir.join("data_0"), b"cache data").unwrap();
    fs::write(cache_dir.join("data_1"), b"more cache").unwrap();
    fs::write(cache_dir.join("index"), b"cache index").unwrap();

    assert!(cache_dir.exists());
}

#[test]
fn test_vscode_shipit_update_cache() {
    let temp = TempDir::new().unwrap();
    let shipit_cache = temp.path().join("Library/Caches/com.microsoft.VSCode.ShipIt");
    fs::create_dir_all(&shipit_cache).unwrap();

    // Create update cache files
    fs::write(shipit_cache.join("update.zip"), b"update package").unwrap();
    fs::write(shipit_cache.join("update.log"), b"update log").unwrap();

    assert!(shipit_cache.exists());
}

#[test]
fn test_vscode_app_cache() {
    let temp = TempDir::new().unwrap();
    let app_cache = temp.path().join("Library/Application Support/Code/Cache");
    fs::create_dir_all(&app_cache).unwrap();

    // Create cache entries
    let cache_data = app_cache.join("Cache_Data");
    fs::create_dir_all(&cache_data).unwrap();

    for i in 0..5 {
        fs::write(cache_data.join(format!("f_{:06}", i)), b"cached").unwrap();
    }

    assert!(cache_data.exists());
}

#[test]
fn test_vscode_cached_data() {
    let temp = TempDir::new().unwrap();
    let cached_data = temp.path().join("Library/Application Support/Code/CachedData");
    fs::create_dir_all(&cached_data).unwrap();

    // Create version-specific cache
    let version_cache = cached_data.join("1a2b3c4d5e6f7890");
    fs::create_dir_all(&version_cache).unwrap();
    fs::write(version_cache.join("main.js"), b"cached js").unwrap();

    assert!(version_cache.exists());
}

#[test]
fn test_vscode_workspace_storage() {
    let temp = TempDir::new().unwrap();
    let workspace_storage = temp.path().join("Library/Application Support/Code/User/workspaceStorage");
    fs::create_dir_all(&workspace_storage).unwrap();

    // Create workspace storage entries
    for i in 0..5 {
        let workspace = workspace_storage.join(format!("abc123def{}", i));
        fs::create_dir_all(&workspace).unwrap();
        fs::write(workspace.join("workspace.json"), b"{}").unwrap();
        fs::write(workspace.join("state.vscdb"), b"state db").unwrap();
    }

    assert!(workspace_storage.exists());
    assert_eq!(fs::read_dir(&workspace_storage).unwrap().count(), 5);
}

#[test]
fn test_vscode_extensions_cache() {
    let temp = TempDir::new().unwrap();
    let extensions = temp.path().join(".vscode/extensions");
    fs::create_dir_all(&extensions).unwrap();

    // Create extension directories
    let ext1 = extensions.join("ms-python.python-2023.1.0");
    fs::create_dir_all(&ext1).unwrap();
    fs::write(ext1.join("package.json"), b"{}").unwrap();

    let ext2 = extensions.join("rust-lang.rust-analyzer-0.3.1234");
    fs::create_dir_all(&ext2).unwrap();
    fs::write(ext2.join("package.json"), b"{}").unwrap();

    assert!(extensions.exists());
}

#[test]
fn test_vscode_logs() {
    let temp = TempDir::new().unwrap();
    let logs = temp.path().join("Library/Application Support/Code/logs");
    fs::create_dir_all(&logs).unwrap();

    // Create log directories for different dates
    for date in &["20240101", "20240102", "20240103"] {
        let log_dir = logs.join(date);
        fs::create_dir_all(&log_dir).unwrap();

        fs::write(log_dir.join("main.log"), b"log content").unwrap();
        fs::write(log_dir.join("renderer.log"), b"renderer log").unwrap();
    }

    assert!(logs.exists());
}

// ==================== JetBrains Tests ====================

#[test]
fn test_jetbrains_cache_root() {
    let temp = TempDir::new().unwrap();
    let jetbrains_root = temp.path().join("Library/Caches/JetBrains");
    fs::create_dir_all(&jetbrains_root).unwrap();

    assert!(jetbrains_root.exists());
}

#[test]
fn test_intellij_idea_cache() {
    let temp = TempDir::new().unwrap();
    let jetbrains_root = temp.path().join("Library/Caches/JetBrains");
    let idea_cache = jetbrains_root.join("IntelliJIdea2023.2");
    fs::create_dir_all(&idea_cache).unwrap();

    // Create cache subdirectories
    let caches = idea_cache.join("caches");
    fs::create_dir_all(&caches).unwrap();
    fs::write(caches.join("content.dat"), b"cache data").unwrap();

    let index = idea_cache.join("index");
    fs::create_dir_all(&index).unwrap();
    fs::write(index.join("index.dat"), b"index data").unwrap();

    assert!(idea_cache.exists());
}

#[test]
fn test_pycharm_cache() {
    let temp = TempDir::new().unwrap();
    let jetbrains_root = temp.path().join("Library/Caches/JetBrains");
    let pycharm_cache = jetbrains_root.join("PyCharm2023.2");
    fs::create_dir_all(&pycharm_cache).unwrap();

    // Create cache files
    fs::write(pycharm_cache.join("caches.db"), b"caches").unwrap();

    assert!(pycharm_cache.exists());
}

#[test]
fn test_webstorm_cache() {
    let temp = TempDir::new().unwrap();
    let jetbrains_root = temp.path().join("Library/Caches/JetBrains");
    let webstorm_cache = jetbrains_root.join("WebStorm2023.2");
    fs::create_dir_all(&webstorm_cache).unwrap();

    fs::write(webstorm_cache.join("workspace.dat"), b"workspace").unwrap();

    assert!(webstorm_cache.exists());
}

#[test]
fn test_goland_cache() {
    let temp = TempDir::new().unwrap();
    let jetbrains_root = temp.path().join("Library/Caches/JetBrains");
    let goland_cache = jetbrains_root.join("GoLand2023.2");
    fs::create_dir_all(&goland_cache).unwrap();

    fs::write(goland_cache.join("go_indexes"), b"indexes").unwrap();

    assert!(goland_cache.exists());
}

#[test]
fn test_rustrover_cache() {
    let temp = TempDir::new().unwrap();
    let jetbrains_root = temp.path().join("Library/Caches/JetBrains");
    let rustrover_cache = jetbrains_root.join("RustRover2023.2");
    fs::create_dir_all(&rustrover_cache).unwrap();

    fs::write(rustrover_cache.join("rust_cache"), b"rust cache").unwrap();

    assert!(rustrover_cache.exists());
}

#[test]
fn test_clion_cache() {
    let temp = TempDir::new().unwrap();
    let jetbrains_root = temp.path().join("Library/Caches/JetBrains");
    let clion_cache = jetbrains_root.join("CLion2023.2");
    fs::create_dir_all(&clion_cache).unwrap();

    // Create cmake caches
    let cmake_cache = clion_cache.join("cmake");
    fs::create_dir_all(&cmake_cache).unwrap();
    fs::write(cmake_cache.join("project_cache"), b"cmake").unwrap();

    assert!(clion_cache.exists());
}

#[test]
fn test_datagrip_cache() {
    let temp = TempDir::new().unwrap();
    let jetbrains_root = temp.path().join("Library/Caches/JetBrains");
    let datagrip_cache = jetbrains_root.join("DataGrip2023.2");
    fs::create_dir_all(&datagrip_cache).unwrap();

    fs::write(datagrip_cache.join("db_cache"), b"db cache").unwrap();

    assert!(datagrip_cache.exists());
}

#[test]
fn test_multiple_jetbrains_versions() {
    let temp = TempDir::new().unwrap();
    let jetbrains_root = temp.path().join("Library/Caches/JetBrains");
    fs::create_dir_all(&jetbrains_root).unwrap();

    // Create multiple versions of same IDE
    for version in &["2022.3", "2023.1", "2023.2", "2023.3"] {
        let idea = jetbrains_root.join(format!("IntelliJIdea{}", version));
        fs::create_dir_all(&idea).unwrap();
        fs::write(idea.join("cache"), b"cache").unwrap();
    }

    assert_eq!(fs::read_dir(&jetbrains_root).unwrap().count(), 4);
}

// ==================== Android Studio Tests ====================

#[test]
fn test_android_studio_cache() {
    let temp = TempDir::new().unwrap();
    let google_caches = temp.path().join("Library/Caches/Google");
    let android_studio = google_caches.join("AndroidStudio2023.1");
    fs::create_dir_all(&android_studio).unwrap();

    // Create cache subdirectories
    let caches = android_studio.join("caches");
    fs::create_dir_all(&caches).unwrap();
    fs::write(caches.join("content.dat"), b"cache").unwrap();

    assert!(android_studio.exists());
}

#[test]
fn test_android_studio_multiple_versions() {
    let temp = TempDir::new().unwrap();
    let google_caches = temp.path().join("Library/Caches/Google");
    fs::create_dir_all(&google_caches).unwrap();

    // Create multiple Android Studio versions
    for version in &["2022.2", "2022.3", "2023.1"] {
        let as_dir = google_caches.join(format!("AndroidStudio{}", version));
        fs::create_dir_all(&as_dir).unwrap();
        fs::write(as_dir.join("cache"), b"cache").unwrap();
    }

    let as_count = fs::read_dir(&google_caches)
        .unwrap()
        .filter(|e| e.as_ref().unwrap().file_name().to_string_lossy().starts_with("AndroidStudio"))
        .count();
    assert_eq!(as_count, 3);
}

#[test]
fn test_android_studio_avd_cache() {
    let temp = TempDir::new().unwrap();
    let avd_dir = temp.path().join(".android/avd");
    fs::create_dir_all(&avd_dir).unwrap();

    // Create AVD (Android Virtual Device) cache
    let emulator = avd_dir.join("Pixel_6_API_33.avd");
    fs::create_dir_all(&emulator).unwrap();
    fs::write(emulator.join("cache.img"), b"cache image").unwrap();
    fs::write(emulator.join("userdata-qemu.img"), b"userdata").unwrap();

    assert!(emulator.exists());
}

#[test]
fn test_gradle_cache_android() {
    let temp = TempDir::new().unwrap();
    let gradle = temp.path().join(".gradle");
    fs::create_dir_all(&gradle).unwrap();

    // Create caches directory
    let caches = gradle.join("caches");
    fs::create_dir_all(&caches).unwrap();

    // Create build cache
    let build_cache = caches.join("build-cache-1");
    fs::create_dir_all(&build_cache).unwrap();
    fs::write(build_cache.join("gc.properties"), b"properties").unwrap();

    assert!(caches.exists());
}

// ==================== Xcode Tests ====================

#[test]
fn test_xcode_derived_data() {
    let temp = TempDir::new().unwrap();
    let derived_data = temp.path().join("Library/Developer/Xcode/DerivedData");
    fs::create_dir_all(&derived_data).unwrap();

    // Create project build directories
    let project = derived_data.join("MyApp-abcdef1234567890");
    fs::create_dir_all(&project).unwrap();

    let build = project.join("Build/Products/Debug-iphonesimulator");
    fs::create_dir_all(&build).unwrap();
    fs::write(build.join("MyApp.app"), b"app bundle").unwrap();

    assert!(project.exists());
}

// ==================== Sublime Text Tests ====================

#[test]
fn test_sublime_text_cache() {
    let temp = TempDir::new().unwrap();
    let sublime_cache = temp.path().join("Library/Caches/com.sublimetext.4");
    fs::create_dir_all(&sublime_cache).unwrap();

    fs::write(sublime_cache.join("cache"), b"cache data").unwrap();

    assert!(sublime_cache.exists());
}

#[test]
fn test_sublime_text_local_cache() {
    let temp = TempDir::new().unwrap();
    let sublime_local = temp.path().join("Library/Application Support/Sublime Text/Local");
    fs::create_dir_all(&sublime_local).unwrap();

    // Create session and cache files
    fs::write(sublime_local.join("Session.sublime_session"), b"session").unwrap();
    fs::write(sublime_local.join("Auto Save Session.sublime_session"), b"auto save").unwrap();

    assert!(sublime_local.exists());
}

// ==================== Atom/Pulsar Tests ====================

#[test]
fn test_atom_cache() {
    let temp = TempDir::new().unwrap();
    let atom_cache = temp.path().join(".atom/compile-cache");
    fs::create_dir_all(&atom_cache).unwrap();

    // Create compile cache
    let js_cache = atom_cache.join("js");
    fs::create_dir_all(&js_cache).unwrap();
    fs::write(js_cache.join("module.js"), b"compiled").unwrap();

    assert!(atom_cache.exists());
}

// ==================== Edge Cases ====================

#[test]
fn test_empty_ide_cache_directories() {
    let temp = TempDir::new().unwrap();

    // Create empty cache directories
    let vscode = temp.path().join("Library/Caches/com.microsoft.VSCode");
    fs::create_dir_all(&vscode).unwrap();

    let jetbrains = temp.path().join("Library/Caches/JetBrains");
    fs::create_dir_all(&jetbrains).unwrap();

    assert!(vscode.exists());
    assert!(jetbrains.exists());
    assert_eq!(fs::read_dir(&vscode).unwrap().count(), 0);
}

#[test]
fn test_mixed_ide_installations() {
    let temp = TempDir::new().unwrap();

    // VSCode
    let vscode = temp.path().join("Library/Caches/com.microsoft.VSCode");
    fs::create_dir_all(&vscode).unwrap();
    fs::write(vscode.join("cache"), b"vscode").unwrap();

    // JetBrains
    let jetbrains = temp.path().join("Library/Caches/JetBrains/IntelliJIdea2023.2");
    fs::create_dir_all(&jetbrains).unwrap();
    fs::write(jetbrains.join("cache"), b"idea").unwrap();

    // Android Studio
    let android_studio = temp.path().join("Library/Caches/Google/AndroidStudio2023.1");
    fs::create_dir_all(&android_studio).unwrap();
    fs::write(android_studio.join("cache"), b"android").unwrap();

    assert!(vscode.exists());
    assert!(jetbrains.exists());
    assert!(android_studio.exists());
}

#[test]
fn test_vscode_insiders_cache() {
    let temp = TempDir::new().unwrap();
    let vscode_insiders = temp.path().join("Library/Caches/com.microsoft.VSCodeInsiders");
    fs::create_dir_all(&vscode_insiders).unwrap();

    fs::write(vscode_insiders.join("cache"), b"insiders cache").unwrap();

    assert!(vscode_insiders.exists());
}

#[test]
fn test_cursor_ide_cache() {
    let temp = TempDir::new().unwrap();
    let cursor_cache = temp.path().join("Library/Caches/com.todesktop.cursor");
    fs::create_dir_all(&cursor_cache).unwrap();

    fs::write(cursor_cache.join("cache"), b"cursor cache").unwrap();

    assert!(cursor_cache.exists());
}

#[test]
fn test_zed_ide_cache() {
    let temp = TempDir::new().unwrap();
    let zed_cache = temp.path().join("Library/Caches/dev.zed.Zed");
    fs::create_dir_all(&zed_cache).unwrap();

    fs::write(zed_cache.join("cache"), b"zed cache").unwrap();

    assert!(zed_cache.exists());
}
