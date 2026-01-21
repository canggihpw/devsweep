use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir};

pub fn check_ide_caches() -> CheckResult {
    let mut result = CheckResult::new("IDE Caches");

    if let Some(home) = home_dir() {
        // VSCode Caches
        let vscode_caches = [
            ("Library/Caches/com.microsoft.VSCode", "VSCode Cache"),
            ("Library/Caches/com.microsoft.VSCode.ShipIt", "VSCode Update Cache"),
            ("Library/Application Support/Code/Cache", "VSCode App Cache"),
            ("Library/Application Support/Code/CachedData", "VSCode Cached Data"),
            ("Library/Application Support/Code/User/workspaceStorage", "VSCode Workspace Storage"),
        ];

        for (path_str, label) in vscode_caches {
            let path = home.join(path_str);
            if path.exists() {
                let size = get_dir_size(&path);
                if size > 0 {
                    result.add_item(
                        CleanupItem::new(label, size, &format_size(size))
                            .with_path(path)
                            .with_safe_to_delete(true)
                    );
                }
            }
        }

        // JetBrains (IntelliJ, PyCharm, WebStorm, etc.)
        // Modern layout: ~/Library/Caches/JetBrains/<Product><Version>
        let jetbrains_root = home.join("Library/Caches/JetBrains");
        if jetbrains_root.exists() {
             if let Ok(entries) = std::fs::read_dir(&jetbrains_root) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let name = path.file_name().unwrap_or_default().to_string_lossy();
                        let size = get_dir_size(&path);
                        if size > 0 {
                             result.add_item(
                                CleanupItem::new(&format!("{} Cache", name), size, &format_size(size))
                                    .with_path(path)
                                    .with_safe_to_delete(true)
                            );
                        }
                    }
                }
             }
        }

        // Android Studio (often under ~/Library/Caches/Google/AndroidStudio*)
        let google_caches = home.join("Library/Caches/Google");
        if google_caches.exists() {
             if let Ok(entries) = std::fs::read_dir(&google_caches) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if name.starts_with("AndroidStudio") {
                        let size = get_dir_size(&path);
                        if size > 0 {
                             result.add_item(
                                CleanupItem::new(&format!("{} Cache", name), size, &format_size(size))
                                    .with_path(path)
                                    .with_safe_to_delete(true)
                            );
                        }
                    }
                }
             }
        }
    }

    result
}
