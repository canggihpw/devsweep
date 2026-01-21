use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir};
use std::fs;
use walkdir::WalkDir;

pub fn check_general_caches() -> CheckResult {
    use rayon::prelude::*;

    let mut result = CheckResult::new("General Caches");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    // User caches
    let cache_dir = home.join("Library/Caches");
    if cache_dir.exists() {
        if let Ok(entries) = fs::read_dir(&cache_dir) {
            let cache_entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();

            // Calculate sizes in parallel
            let mut large_caches: Vec<(String, std::path::PathBuf, u64)> = cache_entries
                .par_iter()
                .filter_map(|entry| {
                    let path = entry.path();
                    if path.is_dir() {
                        let size = get_dir_size(&path);
                        if size > 100 * 1024 * 1024 {
                            // > 100MB
                            let name = entry.file_name().to_string_lossy().to_string();
                            Some((name, path, size))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            // Sort by size descending and add each as a separate item
            large_caches.sort_by(|a, b| b.2.cmp(&a.2));
            for (name, path, size) in large_caches {
                let item = CleanupItem::new(&format!("cache: {}", name), size, &format_size(size))
                    .with_path(path)
                    .with_warning("App may need to rebuild cache");
                result.add_item(item);
            }
        }
    }

    // User logs
    let logs_dir = home.join("Library/Logs");
    if logs_dir.exists() {
        let size = get_dir_size(&logs_dir);
        if size > 50 * 1024 * 1024 {
            // > 50MB
            let item = CleanupItem::new("User Logs", size, &format_size(size))
                .with_path(logs_dir)
                .with_safe_to_delete(true);
            result.add_item(item);
        }
    }

    result
}

pub fn check_trash() -> CheckResult {
    let mut result = CheckResult::new("Trash");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    let trash_path = home.join(".Trash");

    println!("DEBUG: Checking trash at: {:?}", trash_path);
    println!("DEBUG: Trash exists: {}", trash_path.exists());

    if !trash_path.exists() {
        println!("DEBUG: Trash path does not exist");
        return result;
    }

    // Try to read directory entries and count items
    let (item_count, total_size) = match fs::read_dir(&trash_path) {
        Ok(entries) => {
            let mut count = 0;
            let mut size = 0u64;

            for entry in entries {
                match entry {
                    Ok(e) => {
                        count += 1;
                        let path = e.path();
                        println!("DEBUG: Found trash item: {:?}", path.file_name());

                        // Calculate size for this entry
                        if let Ok(metadata) = e.metadata() {
                            if metadata.is_file() {
                                size += metadata.len();
                            } else if metadata.is_dir() {
                                // For directories, walk recursively
                                size += get_dir_size(&path);
                            }
                        }
                    }
                    Err(e) => {
                        println!("DEBUG: Error reading entry: {}", e);
                    }
                }
            }

            println!(
                "DEBUG: Trash contains {} items, total size: {} bytes ({})",
                count,
                size,
                format_size(size)
            );
            (count, size)
        }
        Err(e) => {
            println!("DEBUG: Error reading trash directory: {}", e);
            println!(
                "DEBUG: This may be a permission issue. Try running the app with appropriate permissions."
            );

            // Fallback: try to get metadata of the trash directory itself
            if let Ok(metadata) = fs::metadata(&trash_path) {
                println!(
                    "DEBUG: Trash directory metadata - is_dir: {}, readonly: {}",
                    metadata.is_dir(),
                    metadata.permissions().readonly()
                );
            }

            return result;
        }
    };

    if item_count > 0 && total_size > 0 {
        let item = CleanupItem::new(
            &format!("Empty Trash ({} items)", item_count),
            total_size,
            &format_size(total_size),
        )
        .with_path(trash_path)
        .with_safe_to_delete(true);
        result.add_item(item);
        println!("DEBUG: Added trash item to results");
    } else if item_count > 0 {
        println!(
            "DEBUG: Trash has {} items but size calculated as 0 - may be permission issue",
            item_count
        );
    } else {
        println!("DEBUG: Trash is empty");
    }

    result
}

pub fn check_node_modules() -> CheckResult {
    use rayon::prelude::*;
    use std::sync::{Arc, Mutex};

    let mut result = CheckResult::new("node_modules in Projects");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    let search_paths = [
        home.join("Projects"),
        home.join("Developer"),
        home.join("Code"),
        home.join("Documents"),
        home.join("Desktop"),
    ];

    let node_modules_found: Arc<Mutex<Vec<(String, std::path::PathBuf, u64)>>> =
        Arc::new(Mutex::new(Vec::new()));

    let skip_dirs = [
        ".git",
        "venv",
        ".venv",
        "__pycache__",
        "target",
        "build",
        "dist",
        "node_modules",
    ];

    // Process search paths in parallel
    search_paths
        .par_iter()
        .filter(|p| p.exists())
        .for_each(|search_path| {
            let candidates: Vec<_> = WalkDir::new(search_path)
                .max_depth(6)
                .into_iter()
                .filter_entry(|e| {
                    let name = e.file_name().to_string_lossy();
                    !skip_dirs.contains(&name.as_ref()) || name == "node_modules"
                })
                .filter_map(|e| e.ok())
                .filter(|entry| {
                    let path = entry.path();
                    // Only collect node_modules directories with package.json parent
                    path.is_dir()
                        && path
                            .file_name()
                            .map(|n| n == "node_modules")
                            .unwrap_or(false)
                        && path
                            .parent()
                            .map(|p| p.join("package.json").exists())
                            .unwrap_or(false)
                })
                .collect();

            // Calculate sizes in parallel
            let found: Vec<_> = candidates
                .par_iter()
                .filter_map(|entry| {
                    let path = entry.path();
                    let size = get_dir_size(path);

                    // Only include if > 50MB
                    if size > 50 * 1024 * 1024 {
                        let parent = path.parent()?;
                        Some((
                            parent.to_string_lossy().to_string(),
                            path.to_path_buf(),
                            size,
                        ))
                    } else {
                        None
                    }
                })
                .collect();

            // Update shared state
            if !found.is_empty() {
                node_modules_found.lock().unwrap().extend(found);
            }
        });

    // Extract results from Arc<Mutex>
    let mut node_modules_found = match Arc::try_unwrap(node_modules_found) {
        Ok(mutex) => mutex.into_inner().unwrap(),
        Err(arc) => arc.lock().unwrap().clone(),
    };

    // Sort by size descending and add each as a separate item
    node_modules_found.sort_by(|a, b| b.2.cmp(&a.2));
    for (project_path, node_modules_path, size) in node_modules_found {
        let item = CleanupItem::new(
            &format!("node_modules: {}", project_path),
            size,
            &format_size(size),
        )
        .with_path(node_modules_path)
        .with_warning("Run 'npm install' to restore");
        result.add_item(item);
    }

    result
}
