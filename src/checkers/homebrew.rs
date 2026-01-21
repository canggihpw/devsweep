use crate::types::{
    CheckResult, CleanupItem, ItemDetail, OldVersionInfo, PackageInfo, UnneededPackage,
};
use crate::utils::{format_size, get_dir_size, home_dir, run_command, sort_versions};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn check_homebrew() -> CheckResult {
    let mut result = CheckResult::new("Homebrew");

    // Check if Homebrew is installed
    let brew_version = run_command("brew", &["--version"]);
    if brew_version.is_none() {
        result.status = Some("Homebrew not installed".to_string());
        return result;
    }

    result.status = Some("installed".to_string());

    // Get Homebrew prefix
    let brew_prefix = run_command("brew", &["--prefix"])
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            if PathBuf::from("/opt/homebrew").exists() {
                PathBuf::from("/opt/homebrew")
            } else {
                PathBuf::from("/usr/local")
            }
        });
    result.extra_data.brew_prefix = Some(brew_prefix.clone());

    // Check Homebrew cache
    if let Some(home) = home_dir() {
        let cache_path = home.join("Library/Caches/Homebrew");
        if cache_path.exists() {
            let cache_size = get_dir_size(&cache_path);
            if cache_size > 0 {
                let item = CleanupItem::new("Homebrew Cache", cache_size, &format_size(cache_size))
                    .with_path(cache_path)
                    .with_safe_to_delete(true);
                result.add_item(item);
            }
        }
    }

    // Check for old versions in Cellar
    let cellar_path = brew_prefix.join("Cellar");
    if cellar_path.exists() {
        if let Ok(packages) = fs::read_dir(&cellar_path) {
            let package_entries: Vec<_> = packages.filter_map(|e| e.ok()).collect();

            // Process packages in parallel
            let mut old_versions_info: Vec<OldVersionInfo> = package_entries
                .par_iter()
                .filter_map(|pkg_entry| {
                    let pkg_path = pkg_entry.path();
                    if !pkg_path.is_dir() {
                        return None;
                    }

                    let pkg_name = pkg_entry.file_name().to_string_lossy().to_string();

                    let versions_entries = fs::read_dir(&pkg_path).ok()?;
                    let mut versions: Vec<String> = versions_entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().is_dir())
                        .map(|e| e.file_name().to_string_lossy().to_string())
                        .collect();

                    if versions.len() > 1 {
                        sort_versions(&mut versions);
                        let _latest = versions.pop().unwrap();
                        let old_versions = versions;

                        // Calculate size of old versions in parallel
                        let old_size: u64 = old_versions
                            .par_iter()
                            .map(|v| get_dir_size(pkg_path.join(v)))
                            .sum();

                        if old_size > 1024 * 1024 {
                            // > 1MB
                            Some(OldVersionInfo {
                                package: pkg_name,
                                old_count: old_versions.len(),
                                size: old_size,
                                size_str: format_size(old_size),
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            let total_old_size: u64 = old_versions_info.iter().map(|v| v.size).sum();

            if !old_versions_info.is_empty() {
                old_versions_info.sort_by(|a, b| b.size.cmp(&a.size));

                let details: Vec<ItemDetail> = old_versions_info
                    .iter()
                    .take(15)
                    .map(|v| {
                        ItemDetail::new(&v.package, v.size, &v.size_str)
                            .with_extra_info(&format!("{} old version(s)", v.old_count))
                    })
                    .collect();

                let item = CleanupItem::new(
                    "Old Package Versions",
                    total_old_size,
                    &format_size(total_old_size),
                )
                .with_safe_to_delete(true)
                .with_cleanup_command("brew cleanup")
                .with_details(details);
                result.add_item(item);
                result.extra_data.old_versions = Some(old_versions_info);
            }
        }

        // Check for large packages
        let mut large_packages: Vec<PackageInfo> = Vec::new();
        if let Ok(packages) = fs::read_dir(&cellar_path) {
            for pkg_entry in packages.filter_map(|e| e.ok()) {
                let pkg_path = pkg_entry.path();
                if pkg_path.is_dir() {
                    let size = get_dir_size(&pkg_path);
                    if size > 100 * 1024 * 1024 {
                        // > 100MB
                        large_packages.push(PackageInfo {
                            package: pkg_entry.file_name().to_string_lossy().to_string(),
                            path: pkg_path,
                            size,
                            size_str: format_size(size),
                        });
                    }
                }
            }
        }
        if !large_packages.is_empty() {
            large_packages.sort_by(|a, b| b.size.cmp(&a.size));
            result.extra_data.large_packages = Some(large_packages);
        }
    }

    // Check global node_modules in Homebrew lib
    let brew_node_modules = brew_prefix.join("lib/node_modules");
    if brew_node_modules.exists() {
        let mut global_npm_packages: Vec<PackageInfo> = Vec::new();
        let mut total_npm_size = 0u64;

        if let Ok(packages) = fs::read_dir(&brew_node_modules) {
            for pkg_entry in packages.filter_map(|e| e.ok()) {
                let pkg_path = pkg_entry.path();
                if pkg_path.is_dir() {
                    let size = get_dir_size(&pkg_path);
                    if size > 10 * 1024 * 1024 {
                        // > 10MB
                        global_npm_packages.push(PackageInfo {
                            package: pkg_entry.file_name().to_string_lossy().to_string(),
                            path: pkg_path,
                            size,
                            size_str: format_size(size),
                        });
                        total_npm_size += size;
                    }
                }
            }
        }

        if !global_npm_packages.is_empty() {
            global_npm_packages.sort_by(|a, b| b.size.cmp(&a.size));

            let details: Vec<ItemDetail> = global_npm_packages
                .iter()
                .take(10)
                .map(|p| ItemDetail::new(&p.package, p.size, &p.size_str).with_path(p.path.clone()))
                .collect();

            let item = CleanupItem::new(
                "Global npm Packages (via Homebrew)",
                total_npm_size,
                &format_size(total_npm_size),
            )
            .with_path(brew_node_modules)
            .with_warning("Review before removing - use 'npm uninstall -g <package>'")
            .with_details(details);
            result.add_item(item);
            result.extra_data.global_npm_packages = Some(global_npm_packages);
        }
    }

    // Check for potentially unneeded packages
    if let Some(leaves) = run_command("brew", &["leaves"]) {
        let leaf_list: Vec<String> = leaves.lines().map(|s| s.to_string()).collect();
        result.extra_data.leaf_packages = Some(leaf_list.clone());

        let deprecated_or_replaced: HashMap<&str, &str> = [
            (
                "openssl@1.1",
                "Deprecated - openssl@3 is the current version",
            ),
            ("youtube-dl", "Unmaintained - yt-dlp is the active fork"),
            (
                "python@3.9",
                "Old Python version - consider if still needed",
            ),
            (
                "python@3.10",
                "Older Python version - check if projects need it",
            ),
            ("node@16", "Old Node.js LTS - consider upgrading"),
            ("node@18", "Older Node.js LTS - check if still needed"),
        ]
        .into_iter()
        .collect();

        let mut potentially_unneeded: Vec<UnneededPackage> = Vec::new();
        let mut total_unneeded_size = 0u64;

        for pkg in &leaf_list {
            if let Some(reason) = deprecated_or_replaced.get(pkg.as_str()) {
                let pkg_path = cellar_path.join(pkg);
                let size = if pkg_path.exists() {
                    get_dir_size(&pkg_path)
                } else {
                    0
                };
                potentially_unneeded.push(UnneededPackage {
                    package: pkg.clone(),
                    reason: reason.to_string(),
                    size,
                    size_str: format_size(size),
                });
                total_unneeded_size += size;
            }
        }

        if !potentially_unneeded.is_empty() && total_unneeded_size > 0 {
            let details: Vec<ItemDetail> = potentially_unneeded
                .iter()
                .map(|p| {
                    ItemDetail::new(&p.package, p.size, &p.size_str).with_extra_info(&p.reason)
                })
                .collect();

            let item = CleanupItem::new(
                "Potentially Unneeded Packages",
                total_unneeded_size,
                &format_size(total_unneeded_size),
            )
            .with_warning("Review before removing - these are leaf packages that may be deprecated or replaced")
            .with_details(details);
            result.add_item(item);
            result.extra_data.potentially_unneeded = Some(potentially_unneeded);
        }
    }

    result
}
