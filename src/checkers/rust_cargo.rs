use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir};
use walkdir::WalkDir;

pub fn check_rust() -> CheckResult {
    let mut result = CheckResult::new("Rust/Cargo");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    let cargo_home = home.join(".cargo");
    if !cargo_home.exists() {
        result.status = Some("Cargo not installed".to_string());
        return result;
    }

    // Cargo registry cache
    let registry_cache = cargo_home.join("registry/cache");
    if registry_cache.exists() {
        let size = get_dir_size(&registry_cache);
        if size > 0 {
            let item = CleanupItem::new("Cargo registry cache", size, &format_size(size))
                .with_path(registry_cache)
                .with_safe_to_delete(true);
            result.add_item(item);
        }
    }

    // Cargo git checkouts
    let git_db = cargo_home.join("git");
    if git_db.exists() {
        let size = get_dir_size(&git_db);
        if size > 0 {
            let item = CleanupItem::new("Cargo git checkouts", size, &format_size(size))
                .with_path(git_db)
                .with_safe_to_delete(true);
            result.add_item(item);
        }
    }

    // Find target directories in common project locations
    let search_paths = [
        home.join("Projects"),
        home.join("Developer"),
        home.join("Code"),
        home.join("Documents"),
    ];

    let mut target_dirs: Vec<(String, std::path::PathBuf, u64)> = Vec::new();

    for search_path in search_paths.iter().filter(|p| p.exists()) {
        for entry in WalkDir::new(search_path)
            .max_depth(5)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Check if this is a Rust project with a target directory
            if path.is_dir() && path.join("Cargo.toml").exists() {
                let target_path = path.join("target");
                if target_path.exists() {
                    let size = get_dir_size(&target_path);
                    if size > 100 * 1024 * 1024 {
                        // > 100MB
                        target_dirs.push((path.to_string_lossy().to_string(), target_path, size));
                    }
                }
            }
        }
    }

    // Sort by size descending and add each as a separate item
    // Add two items per target: one for full delete, one for cargo sweep
    target_dirs.sort_by(|a, b| b.2.cmp(&a.2));
    for (project_path, target_path, size) in target_dirs {
        // Option 1: Delete entire target directory (default)
        let delete_item = CleanupItem::new(
            &format!("target: {}", project_path),
            size,
            &format_size(size),
        )
        .with_path(target_path)
        .with_safe_to_delete(true);
        result.add_item(delete_item);

        // Option 2: Use cargo sweep to clean old artifacts only
        // Size is 0 to avoid double-counting in total (it's the same directory)
        let sweep_item = CleanupItem::new(
            &format!("target (sweep): {}", project_path),
            0,
            &format_size(size),
        )
        .with_safe_to_delete(true)
        .with_warning("Cleans artifacts older than 30 days, keeps recent builds")
        .with_cleanup_command(&format!("cargo sweep --time 30 {}", project_path));
        result.add_item(sweep_item);
    }

    result
}
