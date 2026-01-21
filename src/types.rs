use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a single item that can be cleaned
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupItem {
    pub item_type: String,
    pub path: Option<PathBuf>,
    pub size: u64,
    pub size_str: String,
    pub safe_to_delete: bool,
    pub warning: Option<String>,
    pub cleanup_command: Option<String>,
    pub details: Option<Vec<ItemDetail>>,
}

impl CleanupItem {
    pub fn new(item_type: &str, size: u64, size_str: &str) -> Self {
        Self {
            item_type: item_type.to_string(),
            path: None,
            size,
            size_str: size_str.to_string(),
            safe_to_delete: false,
            warning: None,
            cleanup_command: None,
            details: None,
        }
    }

    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    pub fn with_safe_to_delete(mut self, safe: bool) -> Self {
        self.safe_to_delete = safe;
        self
    }

    pub fn with_warning(mut self, warning: &str) -> Self {
        self.warning = Some(warning.to_string());
        self
    }

    pub fn with_cleanup_command(mut self, cmd: &str) -> Self {
        self.cleanup_command = Some(cmd.to_string());
        self
    }

    pub fn with_details(mut self, details: Vec<ItemDetail>) -> Self {
        self.details = Some(details);
        self
    }
}

/// Detail for items with multiple sub-components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDetail {
    pub name: String,
    pub path: Option<PathBuf>,
    pub size: u64,
    pub size_str: String,
    pub extra_info: Option<String>,
}

impl ItemDetail {
    pub fn new(name: &str, size: u64, size_str: &str) -> Self {
        Self {
            name: name.to_string(),
            path: None,
            size,
            size_str: size_str.to_string(),
            extra_info: None,
        }
    }

    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    pub fn with_extra_info(mut self, info: &str) -> Self {
        self.extra_info = Some(info.to_string());
        self
    }
}

/// Result of checking a storage category
#[derive(Debug, Clone)]
pub struct CheckResult {
    #[allow(dead_code)]
    pub name: String,
    pub status: Option<String>,
    pub items: Vec<CleanupItem>,
    pub total_size: u64,
    pub extra_data: ExtraData,
}

impl CheckResult {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            status: None,
            items: Vec::new(),
            total_size: 0,
            extra_data: ExtraData::default(),
        }
    }

    pub fn add_item(&mut self, item: CleanupItem) {
        self.total_size += item.size;
        self.items.push(item);
    }
}

/// Extra data specific to certain check types
#[derive(Debug, Clone, Default)]
pub struct ExtraData {
    // Docker
    pub dangling_images: Option<usize>,
    pub stopped_containers: Option<usize>,
    pub docker_summary: Option<String>,

    // Homebrew
    pub brew_prefix: Option<PathBuf>,
    pub old_versions: Option<Vec<OldVersionInfo>>,
    pub large_packages: Option<Vec<PackageInfo>>,
    pub global_npm_packages: Option<Vec<PackageInfo>>,
    pub potentially_unneeded: Option<Vec<UnneededPackage>>,
    pub leaf_packages: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct OldVersionInfo {
    pub package: String,
    pub old_count: usize,
    pub size: u64,
    pub size_str: String,
}

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub package: String,
    pub path: PathBuf,
    pub size: u64,
    pub size_str: String,
}

#[derive(Debug, Clone)]
pub struct UnneededPackage {
    pub package: String,
    pub reason: String,
    pub size: u64,
    pub size_str: String,
}

/// Cleanup action that can be executed
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CleanupAction {
    pub name: String,
    pub category: String,
    pub action_type: ActionType,
    pub warning: Option<String>,
    pub info: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ActionType {
    Command { cmd: String, args: Vec<String> },
    DeleteDirectory { path: PathBuf },
    ShellCommand { command: String },
}

impl CleanupAction {
    #[allow(dead_code)]
    pub fn command(name: &str, category: &str, cmd: &str, args: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            action_type: ActionType::Command {
                cmd: cmd.to_string(),
                args: args.into_iter().map(|s| s.to_string()).collect(),
            },
            warning: None,
            info: None,
        }
    }

    #[allow(dead_code)]
    pub fn delete_dir(name: &str, category: &str, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            action_type: ActionType::DeleteDirectory { path },
            warning: None,
            info: None,
        }
    }

    #[allow(dead_code)]
    pub fn shell(name: &str, category: &str, command: &str) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            action_type: ActionType::ShellCommand {
                command: command.to_string(),
            },
            warning: None,
            info: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_warning(mut self, warning: &str) -> Self {
        self.warning = Some(warning.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_info(mut self, info: &str) -> Self {
        self.info = Some(info.to_string());
        self
    }
}
