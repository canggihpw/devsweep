use crate::types::{CheckResult, CleanupItem};
use crate::utils::{format_size, get_dir_size, home_dir};

pub fn check_shell_caches() -> CheckResult {
    let mut result = CheckResult::new("Shell Caches");

    if let Some(home) = home_dir() {
        // Zsh
        // Oh My Zsh Cache
        let omz_cache = home.join(".oh-my-zsh/cache");
        if omz_cache.exists() {
            let size = get_dir_size(&omz_cache);
            if size > 0 {
                result.add_item(
                    CleanupItem::new("Oh My Zsh Cache", size, &format_size(size))
                        .with_path(omz_cache)
                        .with_safe_to_delete(true),
                );
            }
        }

        // Zsh completion dumps (safe to regenerate)
        if let Ok(entries) = std::fs::read_dir(&home) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with(".zcompdump") {
                        let size = get_dir_size(&path);
                        if size > 0 {
                            result.add_item(
                                CleanupItem::new(
                                    &format!("Zsh Completion Dump ({})", name),
                                    size,
                                    &format_size(size),
                                )
                                .with_path(path)
                                .with_safe_to_delete(true),
                            );
                        }
                    }
                }
            }
        }

        // Zsh sessions/history (optional, might be sensitive)
        let zsh_sessions = home.join(".zsh_sessions");
        if zsh_sessions.exists() {
            let size = get_dir_size(&zsh_sessions);
            if size > 0 {
                result.add_item(
                    CleanupItem::new("Zsh Sessions", size, &format_size(size))
                        .with_path(zsh_sessions)
                        .with_safe_to_delete(true),
                );
            }
        }

        // Bash
        // Bash sessions/history often in .bash_sessions on macOS
        let bash_sessions = home.join(".bash_sessions");
        if bash_sessions.exists() {
            let size = get_dir_size(&bash_sessions);
            if size > 0 {
                result.add_item(
                    CleanupItem::new("Bash Sessions", size, &format_size(size))
                        .with_path(bash_sessions)
                        .with_safe_to_delete(true),
                );
            }
        }

        // Fish Shell
        // Fish cache is typically in ~/.config/fish/cache or ~/.cache/fish
        let fish_cache_config = home.join(".config/fish/cache");
        if fish_cache_config.exists() {
            let size = get_dir_size(&fish_cache_config);
            if size > 0 {
                result.add_item(
                    CleanupItem::new("Fish Shell Cache (.config)", size, &format_size(size))
                        .with_path(fish_cache_config)
                        .with_safe_to_delete(true),
                );
            }
        }

        let fish_cache_local = home.join(".cache/fish");
        if fish_cache_local.exists() {
            let size = get_dir_size(&fish_cache_local);
            if size > 0 {
                result.add_item(
                    CleanupItem::new("Fish Shell Cache (.cache)", size, &format_size(size))
                        .with_path(fish_cache_local)
                        .with_safe_to_delete(true),
                );
            }
        }

        // Starship Prompt Cache (common cross-shell prompt)
        let starship_cache = home.join(".cache/starship");
        if starship_cache.exists() {
            let size = get_dir_size(&starship_cache);
            if size > 0 {
                result.add_item(
                    CleanupItem::new("Starship Prompt Cache", size, &format_size(size))
                        .with_path(starship_cache)
                        .with_safe_to_delete(true),
                );
            }
        }
    }

    result
}
