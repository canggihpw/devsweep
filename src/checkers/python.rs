use crate::types::{CheckResult, CleanupItem, ItemDetail};
use crate::utils::{format_size, get_dir_size, home_dir};
use std::fs;

pub fn check_python() -> CheckResult {
    let mut result = CheckResult::new("Python");

    let home = match home_dir() {
        Some(h) => h,
        None => return result,
    };

    // pip cache
    let pip_cache = home.join("Library/Caches/pip");
    if pip_cache.exists() {
        let size = get_dir_size(&pip_cache);
        if size > 0 {
            let item = CleanupItem::new("pip cache", size, &format_size(size))
                .with_path(pip_cache)
                .with_safe_to_delete(true);
            result.add_item(item);
        }
    }

    // pyenv versions
    let pyenv_path = home.join(".pyenv/versions");
    if pyenv_path.exists() {
        let size = get_dir_size(&pyenv_path);
        if size > 0 {
            let versions: Vec<String> = fs::read_dir(&pyenv_path)
                .ok()
                .map(|entries| {
                    entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().is_dir())
                        .map(|e| e.file_name().to_string_lossy().to_string())
                        .collect()
                })
                .unwrap_or_default();

            let details: Vec<ItemDetail> = versions
                .iter()
                .take(10)
                .map(|v| {
                    let v_path = pyenv_path.join(v);
                    let v_size = get_dir_size(&v_path);
                    ItemDetail::new(v, v_size, &format_size(v_size)).with_path(v_path)
                })
                .collect();

            let item = CleanupItem::new("pyenv versions", size, &format_size(size))
                .with_path(pyenv_path)
                .with_warning("Contains Python versions - review before removing")
                .with_details(details);
            result.add_item(item);
        }
    }

    // conda (check both miniconda3 and anaconda3)
    for conda_name in &["miniconda3", "anaconda3"] {
        let conda_path = home.join(conda_name);
        if conda_path.exists() {
            let size = get_dir_size(&conda_path);
            if size > 0 {
                let item = CleanupItem::new(
                    &format!("Conda installation ({})", conda_name),
                    size,
                    &format_size(size),
                )
                .with_path(conda_path)
                .with_warning("Full Conda installation - use 'conda clean --all' to clean caches");
                result.add_item(item);
                break; // Only report one conda installation
            }
        }
    }

    // virtualenvs
    let venv_path = home.join(".virtualenvs");
    if venv_path.exists() {
        let size = get_dir_size(&venv_path);
        if size > 0 {
            let envs: Vec<String> = fs::read_dir(&venv_path)
                .ok()
                .map(|entries| {
                    entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().is_dir())
                        .map(|e| e.file_name().to_string_lossy().to_string())
                        .collect()
                })
                .unwrap_or_default();

            let details: Vec<ItemDetail> = envs
                .iter()
                .take(10)
                .map(|v| {
                    let v_path = venv_path.join(v);
                    let v_size = get_dir_size(&v_path);
                    ItemDetail::new(v, v_size, &format_size(v_size)).with_path(v_path)
                })
                .collect();

            let item = CleanupItem::new("virtualenvs", size, &format_size(size))
                .with_path(venv_path)
                .with_warning("Contains virtual environments - review before removing")
                .with_details(details);
            result.add_item(item);
        }
    }

    result
}
