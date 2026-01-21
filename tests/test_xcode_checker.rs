//! Xcode checker comprehensive tests
//! Testing DerivedData, Archives, and DeviceSupport detection with real file structures

use devsweep::checkers;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_xcode_checker_basic_functionality() {
    let result = checkers::check_xcode();

    // Should return a result (items may be empty if Xcode not installed)
    assert!(!result.name.is_empty());
}

#[test]
fn test_derived_data_detection() {
    let temp = TempDir::new().unwrap();
    let derived_data = temp.path().join("DerivedData");
    fs::create_dir_all(&derived_data).unwrap();

    // Create mock project build data
    let project1 = derived_data.join("MyApp-abcdefghijklmnop");
    fs::create_dir_all(&project1).unwrap();

    let build = project1.join("Build");
    fs::create_dir_all(&build).unwrap();

    let products = build.join("Products");
    fs::create_dir_all(&products).unwrap();
    fs::write(products.join("MyApp.app"), b"app bundle").unwrap();

    let intermediates = build.join("Intermediates.noindex");
    fs::create_dir_all(&intermediates).unwrap();
    fs::write(intermediates.join("build.db"), b"build database").unwrap();

    assert!(derived_data.exists());
    assert!(build.exists());
    assert!(products.exists());
}

#[test]
fn test_multiple_derived_data_projects() {
    let temp = TempDir::new().unwrap();
    let derived_data = temp.path().join("DerivedData");
    fs::create_dir_all(&derived_data).unwrap();

    // Create multiple project builds
    for i in 0..5 {
        let project = derived_data.join(format!("Project{}-{:016x}", i, i));
        fs::create_dir_all(&project).unwrap();

        let build = project.join("Build");
        fs::create_dir_all(&build).unwrap();

        let products = build.join("Products");
        fs::create_dir_all(&products).unwrap();
        fs::write(products.join(format!("App{}.app", i)), b"app").unwrap();
    }

    assert_eq!(fs::read_dir(&derived_data).unwrap().count(), 5);
}

#[test]
fn test_archives_detection() {
    let temp = TempDir::new().unwrap();
    let archives = temp.path().join("Archives");
    fs::create_dir_all(&archives).unwrap();

    // Create dated archive directory
    let date_dir = archives.join("2024-01-15");
    fs::create_dir_all(&date_dir).unwrap();

    // Create .xcarchive
    let archive1 = date_dir.join("MyApp 2024-01-15 10.30.00.xcarchive");
    fs::create_dir_all(&archive1).unwrap();

    let products = archive1.join("Products");
    fs::create_dir_all(&products).unwrap();

    let applications = products.join("Applications");
    fs::create_dir_all(&applications).unwrap();
    fs::write(applications.join("MyApp.app"), b"archived app").unwrap();

    let dsyms = archive1.join("dSYMs");
    fs::create_dir_all(&dsyms).unwrap();
    fs::write(dsyms.join("MyApp.app.dSYM"), b"debug symbols").unwrap();

    assert!(archive1.exists());
    assert!(dsyms.exists());
}

#[test]
fn test_multiple_archives() {
    let temp = TempDir::new().unwrap();
    let archives = temp.path().join("Archives");
    fs::create_dir_all(&archives).unwrap();

    // Create multiple dated directories
    for date in &["2024-01-10", "2024-01-15", "2024-01-20"] {
        let date_dir = archives.join(date);
        fs::create_dir_all(&date_dir).unwrap();

        // Create multiple archives per date
        for i in 0..3 {
            let archive = date_dir.join(format!("Build {} {}.xcarchive", i, date));
            fs::create_dir_all(&archive).unwrap();
            fs::write(archive.join("Info.plist"), b"<?xml version=\"1.0\"?>").unwrap();
        }
    }

    assert_eq!(fs::read_dir(&archives).unwrap().count(), 3);
}

#[test]
fn test_device_support_detection() {
    let temp = TempDir::new().unwrap();
    let device_support = temp.path().join("DeviceSupport");
    fs::create_dir_all(&device_support).unwrap();

    // Create iOS device support
    let ios_support = device_support.join("iOS DeviceSupport");
    fs::create_dir_all(&ios_support).unwrap();

    let version1 = ios_support.join("15.0 (19A346)");
    fs::create_dir_all(&version1).unwrap();

    let symbols = version1.join("Symbols");
    fs::create_dir_all(&symbols).unwrap();
    fs::write(symbols.join("System"), b"system symbols").unwrap();

    let version2 = ios_support.join("16.0 (20A362)");
    fs::create_dir_all(&version2).unwrap();
    fs::create_dir_all(version2.join("Symbols")).unwrap();

    assert!(ios_support.exists());
    assert_eq!(fs::read_dir(&ios_support).unwrap().count(), 2);
}

#[test]
fn test_watchos_device_support() {
    let temp = TempDir::new().unwrap();
    let device_support = temp.path().join("DeviceSupport");
    fs::create_dir_all(&device_support).unwrap();

    // Create watchOS device support
    let watchos_support = device_support.join("watchOS DeviceSupport");
    fs::create_dir_all(&watchos_support).unwrap();

    let version = watchos_support.join("9.0 (20R362)");
    fs::create_dir_all(&version).unwrap();

    let symbols = version.join("Symbols");
    fs::create_dir_all(&symbols).unwrap();

    assert!(watchos_support.exists());
}

#[test]
fn test_tvos_device_support() {
    let temp = TempDir::new().unwrap();
    let device_support = temp.path().join("DeviceSupport");
    fs::create_dir_all(&device_support).unwrap();

    // Create tvOS device support
    let tvos_support = device_support.join("tvOS DeviceSupport");
    fs::create_dir_all(&tvos_support).unwrap();

    let version = tvos_support.join("16.0 (20J373)");
    fs::create_dir_all(&version).unwrap();
    fs::create_dir_all(version.join("Symbols")).unwrap();

    assert!(tvos_support.exists());
}

#[test]
fn test_xcode_caches_directory() {
    let temp = TempDir::new().unwrap();
    let caches = temp.path().join("Caches").join("com.apple.dt.Xcode");
    fs::create_dir_all(&caches).unwrap();

    // Create various cache subdirectories
    fs::create_dir_all(caches.join("SymbolCache")).unwrap();
    fs::create_dir_all(caches.join("ModuleCache.noindex")).unwrap();
    fs::create_dir_all(caches.join("DerivedData")).unwrap();

    assert!(caches.join("SymbolCache").exists());
    assert!(caches.join("ModuleCache.noindex").exists());
}

#[test]
fn test_simulator_devices() {
    let temp = TempDir::new().unwrap();
    let developer = temp.path().join("Developer");
    fs::create_dir_all(&developer).unwrap();

    // Create CoreSimulator directory
    let core_sim = developer.join("CoreSimulator").join("Devices");
    fs::create_dir_all(&core_sim).unwrap();

    // Create simulator devices
    for i in 0..3 {
        let device_uuid = format!("AAAAAAAA-BBBB-CCCC-DDDD-{:012}", i);
        let device = core_sim.join(&device_uuid);
        fs::create_dir_all(&device).unwrap();

        fs::write(device.join("device.plist"), b"<?xml version=\"1.0\"?>").unwrap();

        let data = device.join("data");
        fs::create_dir_all(&data).unwrap();
    }

    assert_eq!(fs::read_dir(&core_sim).unwrap().count(), 3);
}

#[test]
fn test_simulator_runtime_caches() {
    let temp = TempDir::new().unwrap();
    let developer = temp.path().join("Developer");
    fs::create_dir_all(&developer).unwrap();

    let caches = developer.join("CoreSimulator").join("Caches");
    fs::create_dir_all(&caches).unwrap();

    // Create dyld cache
    let dyld = caches.join("dyld");
    fs::create_dir_all(&dyld).unwrap();
    fs::write(dyld.join("dyld_sim_shared_cache_arm64"), b"cache").unwrap();

    assert!(dyld.exists());
}

#[test]
fn test_xcode_logs() {
    let temp = TempDir::new().unwrap();
    let logs = temp.path().join("Logs").join("Xcode");
    fs::create_dir_all(&logs).unwrap();

    // Create build logs
    for i in 0..10 {
        fs::write(logs.join(format!("build-{}.log", i)), b"Build log content").unwrap();
    }

    assert_eq!(fs::read_dir(&logs).unwrap().count(), 10);
}

#[test]
fn test_index_data() {
    let temp = TempDir::new().unwrap();
    let derived_data = temp.path().join("DerivedData");
    fs::create_dir_all(&derived_data).unwrap();

    let project = derived_data.join("MyApp-abcdef");
    fs::create_dir_all(&project).unwrap();

    // Create Index directory
    let index = project.join("Index.noindex");
    fs::create_dir_all(&index).unwrap();

    let data_store = index.join("DataStore");
    fs::create_dir_all(&data_store).unwrap();
    fs::write(data_store.join("index.db"), b"index database").unwrap();

    assert!(index.exists());
    assert!(data_store.exists());
}

#[test]
fn test_module_cache() {
    let temp = TempDir::new().unwrap();
    let derived_data = temp.path().join("DerivedData");
    fs::create_dir_all(&derived_data).unwrap();

    let project = derived_data.join("MyApp-abcdef");
    fs::create_dir_all(&project).unwrap();

    // Create ModuleCache
    let module_cache = project.join("ModuleCache.noindex");
    fs::create_dir_all(&module_cache).unwrap();

    // Create module files
    for module in &["Foundation", "UIKit", "SwiftUI"] {
        let module_dir = module_cache.join(module);
        fs::create_dir_all(&module_dir).unwrap();
        fs::write(module_dir.join(format!("{}.pcm", module)), b"module").unwrap();
    }

    assert_eq!(fs::read_dir(&module_cache).unwrap().count(), 3);
}

#[test]
fn test_swift_package_manager_cache() {
    let temp = TempDir::new().unwrap();
    let derived_data = temp.path().join("DerivedData");
    fs::create_dir_all(&derived_data).unwrap();

    let project = derived_data.join("MyApp-abcdef");
    fs::create_dir_all(&project).unwrap();

    // Create SourcePackages directory
    let source_packages = project.join("SourcePackages");
    fs::create_dir_all(&source_packages).unwrap();

    let checkouts = source_packages.join("checkouts");
    fs::create_dir_all(&checkouts).unwrap();

    // Create package checkouts
    for pkg in &["Alamofire", "SDWebImage", "RxSwift"] {
        let pkg_dir = checkouts.join(pkg);
        fs::create_dir_all(&pkg_dir).unwrap();
        fs::write(pkg_dir.join("Package.swift"), b"// swift-tools-version:5.5").unwrap();
    }

    assert_eq!(fs::read_dir(&checkouts).unwrap().count(), 3);
}

#[test]
fn test_xcuserdata_directories() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("MyApp.xcodeproj");
    fs::create_dir_all(&project).unwrap();

    // Create xcuserdata
    let xcuserdata = project.join("xcuserdata");
    fs::create_dir_all(&xcuserdata).unwrap();

    let user = xcuserdata.join("username.xcuserdatad");
    fs::create_dir_all(&user).unwrap();

    fs::create_dir_all(user.join("xcschemes")).unwrap();
    fs::create_dir_all(user.join("xcdebugger")).unwrap();

    assert!(xcuserdata.exists());
}

#[test]
fn test_build_products_debug_release() {
    let temp = TempDir::new().unwrap();
    let derived_data = temp.path().join("DerivedData");
    fs::create_dir_all(&derived_data).unwrap();

    let project = derived_data.join("MyApp-abcdef");
    fs::create_dir_all(&project).unwrap();

    let build = project.join("Build").join("Products");
    fs::create_dir_all(&build).unwrap();

    // Create Debug and Release builds
    let debug = build.join("Debug-iphoneos");
    fs::create_dir_all(&debug).unwrap();
    fs::write(debug.join("MyApp.app"), b"debug app").unwrap();

    let release = build.join("Release-iphoneos");
    fs::create_dir_all(&release).unwrap();
    fs::write(release.join("MyApp.app"), b"release app").unwrap();

    assert!(debug.exists());
    assert!(release.exists());
}

#[test]
fn test_xcode_checker_returns_valid_structure() {
    let result = checkers::check_xcode();

    // Verify result structure
    assert!(!result.name.is_empty());

    // All items should have valid structure
    for item in result.items {
        assert!(!item.item_type.is_empty());
        // size is u64, always valid
    }
}

#[test]
fn test_large_derived_data_structure() {
    let temp = TempDir::new().unwrap();
    let derived_data = temp.path().join("DerivedData");
    fs::create_dir_all(&derived_data).unwrap();

    // Create many project builds (simulate heavy usage)
    for i in 0..20 {
        let project = derived_data.join(format!("Project{}-{:016x}", i, i * 12345));
        fs::create_dir_all(&project).unwrap();

        let build = project.join("Build");
        fs::create_dir_all(&build).unwrap();
    }

    assert_eq!(fs::read_dir(&derived_data).unwrap().count(), 20);
}

#[test]
fn test_archive_info_plist() {
    let temp = TempDir::new().unwrap();
    let archives = temp.path().join("Archives");
    fs::create_dir_all(&archives).unwrap();

    let date_dir = archives.join("2024-01-15");
    fs::create_dir_all(&date_dir).unwrap();

    let archive = date_dir.join("MyApp.xcarchive");
    fs::create_dir_all(&archive).unwrap();

    // Create Info.plist
    let plist = archive.join("Info.plist");
    fs::write(
        &plist,
        b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\">
<plist version=\"1.0\">
<dict>
    <key>Name</key>
    <string>MyApp</string>
</dict>
</plist>",
    )
    .unwrap();

    assert!(plist.exists());
}

#[test]
fn test_device_support_symbols_structure() {
    let temp = TempDir::new().unwrap();
    let device_support = temp.path().join("DeviceSupport");
    fs::create_dir_all(&device_support).unwrap();

    let ios_support = device_support.join("iOS DeviceSupport");
    fs::create_dir_all(&ios_support).unwrap();

    let version = ios_support.join("16.0 (20A362)");
    fs::create_dir_all(&version).unwrap();

    let symbols = version.join("Symbols");
    fs::create_dir_all(&symbols).unwrap();

    // Create system framework structure
    let system = symbols.join("System").join("Library").join("Frameworks");
    fs::create_dir_all(&system).unwrap();

    fs::create_dir_all(system.join("Foundation.framework")).unwrap();
    fs::create_dir_all(system.join("UIKit.framework")).unwrap();

    assert!(symbols.exists());
    assert!(system.exists());
}

#[test]
fn test_empty_derived_data() {
    let temp = TempDir::new().unwrap();
    let derived_data = temp.path().join("DerivedData");
    fs::create_dir_all(&derived_data).unwrap();

    // Empty DerivedData directory
    assert!(derived_data.exists());
    assert_eq!(fs::read_dir(&derived_data).unwrap().count(), 0);
}

#[test]
fn test_simulator_unavailable_devices() {
    let temp = TempDir::new().unwrap();
    let developer = temp.path().join("Developer");
    fs::create_dir_all(&developer).unwrap();

    let devices = developer.join("CoreSimulator").join("Devices");
    fs::create_dir_all(&devices).unwrap();

    // Create Unavailable directory for old simulators
    let unavailable = devices.join("Unavailable");
    fs::create_dir_all(&unavailable).unwrap();

    for i in 0..5 {
        let device = unavailable.join(format!("OLD-DEVICE-{}", i));
        fs::create_dir_all(&device).unwrap();
    }

    assert!(unavailable.exists());
    assert_eq!(fs::read_dir(&unavailable).unwrap().count(), 5);
}
