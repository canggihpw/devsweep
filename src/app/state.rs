use crate::backend::{CategoryData, StorageBackend};
use crate::custom_paths::{CustomPath, CustomPathsConfig};
use crate::trends::{CategoryTrendData, TrendData, TrendTimeRange};
use crate::types;
use crate::ui::sidebar::Tab;
use crate::ui::ThemeMode;
use crate::update_checker::UpdateInfo;
use crate::utils;
use gpui::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Size filter threshold options
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SizeFilter {
    /// Show all items (no filter)
    #[default]
    All,
    /// Show items > 1 MB
    Above1MB,
    /// Show items > 10 MB
    Above10MB,
    /// Show items > 100 MB
    Above100MB,
    /// Show items > 500 MB
    Above500MB,
    /// Show items > 1 GB
    Above1GB,
}

impl SizeFilter {
    /// Get the threshold in bytes for this filter
    pub fn threshold_bytes(&self) -> u64 {
        match self {
            Self::All => 0,
            Self::Above1MB => 1024 * 1024,
            Self::Above10MB => 10 * 1024 * 1024,
            Self::Above100MB => 100 * 1024 * 1024,
            Self::Above500MB => 500 * 1024 * 1024,
            Self::Above1GB => 1024 * 1024 * 1024,
        }
    }

    /// Get display label for this filter
    pub fn label(&self) -> &'static str {
        match self {
            Self::All => "All sizes",
            Self::Above1MB => "> 1 MB",
            Self::Above10MB => "> 10 MB",
            Self::Above100MB => "> 100 MB",
            Self::Above500MB => "> 500 MB",
            Self::Above1GB => "> 1 GB",
        }
    }

    /// Get all filter options in order
    pub fn all_options() -> Vec<Self> {
        vec![
            Self::All,
            Self::Above1MB,
            Self::Above10MB,
            Self::Above100MB,
            Self::Above500MB,
            Self::Above1GB,
        ]
    }

    /// Check if an item with the given size passes this filter
    pub fn passes(&self, size: u64) -> bool {
        size >= self.threshold_bytes()
    }
}

// Super category definitions
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SuperCategoryType {
    DevelopmentTools,
    PackageManagers,
    ProjectFiles,
    SystemAndBrowsers,
    Trash,
}

impl SuperCategoryType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::DevelopmentTools => "Development Tools",
            Self::PackageManagers => "Package Managers",
            Self::ProjectFiles => "Project Files",
            Self::SystemAndBrowsers => "System & Browsers",
            Self::Trash => "Trash",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::DevelopmentTools => "🛠",
            Self::PackageManagers => "📦",
            Self::ProjectFiles => "📁",
            Self::SystemAndBrowsers => "🌐",
            Self::Trash => "🗑",
        }
    }

    /// Map a category name to its super category
    pub fn from_category_name(name: &str) -> Self {
        match name {
            // Development Tools
            "Docker" | "Homebrew" | "Xcode" | "IDE Caches" => Self::DevelopmentTools,
            // Package Managers (match actual checker names)
            "Node.js Package Managers" | "Python" | "Rust/Cargo" | "Go" | "Java Build Tools" => {
                Self::PackageManagers
            }
            // Project Files
            "node_modules in Projects" | "Git Repositories" | "Custom Paths" => Self::ProjectFiles,
            // System & Browsers (match actual checker names)
            "System Logs & Crash Reports"
            | "Browser Caches"
            | "Shell Caches"
            | "Database Caches"
            | "General Caches" => Self::SystemAndBrowsers,
            // Trash
            "Trash" => Self::Trash,
            // Default to System & Browsers for unknown categories
            _ => Self::SystemAndBrowsers,
        }
    }

    /// Get all super category types in display order
    pub fn all() -> Vec<Self> {
        vec![
            Self::DevelopmentTools,
            Self::PackageManagers,
            Self::ProjectFiles,
            Self::SystemAndBrowsers,
            Self::Trash,
        ]
    }
}

#[derive(Clone)]
pub struct SuperCategoryItem {
    pub super_type: SuperCategoryType,
    pub name: SharedString,
    pub icon: SharedString,
    pub total_size: u64,
    pub size_str: SharedString,
    pub item_count: i32,
    pub category_count: i32,
    pub checked: bool,
    pub expanded: bool,
    /// Indices into the categories vec for categories in this super category
    pub category_indices: Vec<usize>,
}

// UI Data structures
#[derive(Clone)]
pub struct CategoryItem {
    pub name: SharedString,
    pub size: SharedString,
    #[allow(dead_code)]
    pub total_size: u64,
    pub item_count: i32,
    pub checked: bool,
    pub expanded: bool,
    /// Which super category this belongs to
    pub super_category: SuperCategoryType,
}

#[derive(Clone)]
pub struct CleanupItemData {
    pub item_type: SharedString,
    pub path: SharedString,
    pub size_str: SharedString,
    #[allow(dead_code)]
    pub size: u64,
    pub safe_to_delete: bool,
    #[allow(dead_code)]
    pub warning: SharedString,
    pub has_warning: bool,
    pub selected: bool,
    pub category_index: usize,
}

#[derive(Clone)]
pub struct QuarantineRecordData {
    pub id: SharedString,
    pub timestamp: SharedString,
    pub total_size: SharedString,
    pub item_count: i32,
    pub success_count: i32,
    pub error_count: i32,
    pub can_undo: bool,
    pub expanded: bool,
}

#[derive(Clone)]
pub struct QuarantineItemData {
    pub item_type: SharedString,
    pub original_path: SharedString,
    pub size_str: SharedString,
    pub success: bool,
    pub error_message: SharedString,
    #[allow(dead_code)]
    pub can_restore: bool,
    pub deleted_permanently: bool,
    pub record_id: SharedString,
    pub item_index: usize,
    pub quarantine_path: Option<PathBuf>,
}

#[derive(Clone)]
pub struct CacheTTLSetting {
    pub category: SharedString,
    pub ttl_minutes: i32,
}

// Main application state
pub struct DevSweep {
    pub backend: Arc<Mutex<StorageBackend>>,
    pub active_tab: Tab,
    pub theme_mode: ThemeMode,
    pub is_scanning: bool,
    pub is_cleaning: bool,
    pub status_text: SharedString,
    pub storage_available: SharedString,
    pub storage_total: u64,
    pub storage_used_fraction: f32,
    pub total_reclaimable: SharedString,
    pub selected_items_count: i32,
    pub selected_items_size: SharedString,
    pub super_categories: Vec<SuperCategoryItem>,
    pub categories: Vec<CategoryItem>,
    pub all_items: Vec<CleanupItemData>,
    pub category_data: Vec<CategoryData>,
    pub selected_items: Vec<types::CleanupItem>,
    pub quarantine_records: Vec<QuarantineRecordData>,
    pub quarantine_items: Vec<QuarantineItemData>,
    pub quarantine_total_size: SharedString,
    pub quarantine_total_items: i32,
    pub cache_ttls: Vec<CacheTTLSetting>,
    // Update checker state
    pub is_checking_update: bool,
    pub update_info: Option<UpdateInfo>,
    pub update_error: Option<String>,
    pub update_check_completed: bool,
    // Custom paths state
    pub custom_paths: Vec<CustomPath>,
    pub new_custom_path_input: String,
    pub new_custom_path_label: String,
    // Size filter state
    pub size_filter: SizeFilter,
    pub size_filter_dropdown_open: bool,
    /// Filtered items (items that pass the current size filter)
    pub filtered_items: Vec<CleanupItemData>,
    /// Filtered super categories (rebuilt when filter changes)
    pub filtered_super_categories: Vec<SuperCategoryItem>,
    // Trends state
    pub trend_time_range: TrendTimeRange,
    pub trend_data: Option<TrendData>,
    pub category_trends: Vec<CategoryTrendData>,
    pub has_trend_data: bool,
    pub trend_snapshot_count: usize,
}

impl Default for DevSweep {
    fn default() -> Self {
        Self::new()
    }
}

impl DevSweep {
    pub fn new() -> Self {
        let backend = Arc::new(Mutex::new(StorageBackend::new()));

        // Load initial cache TTLs
        let ttls = backend.lock().unwrap().get_all_cache_ttls();
        let mut cache_ttls: Vec<CacheTTLSetting> = ttls
            .iter()
            .map(|(cat, ttl_sec)| CacheTTLSetting {
                category: cat.clone().into(),
                ttl_minutes: (*ttl_sec / 60) as i32,
            })
            .collect();
        cache_ttls.sort_by(|a, b| a.category.cmp(&b.category));

        // Get initial storage info
        let (storage_available, storage_total, storage_used_fraction) =
            if let Ok(stat) = fs2::statvfs("/") {
                let available = stat.available_space();
                let total = stat.total_space();
                let used_fraction = if total > 0 {
                    1.0 - (available as f32 / total as f32)
                } else {
                    0.0
                };
                (utils::format_size(available).into(), total, used_fraction)
            } else {
                ("Unknown".into(), 0, 0.0)
            };

        Self {
            backend,
            active_tab: Tab::Scan,
            theme_mode: ThemeMode::default(),
            is_scanning: false,
            is_cleaning: false,
            status_text: "Click 'Scan' to analyze your storage".into(),
            storage_available,
            storage_total,
            storage_used_fraction,
            total_reclaimable: "0 B".into(),
            selected_items_count: 0,
            selected_items_size: "0 B".into(),
            super_categories: Vec::new(),
            categories: Vec::new(),
            all_items: Vec::new(),
            category_data: Vec::new(),
            selected_items: Vec::new(),
            quarantine_records: Vec::new(),
            quarantine_items: Vec::new(),
            quarantine_total_size: "0 B".into(),
            quarantine_total_items: 0,
            cache_ttls,
            // Update checker state
            is_checking_update: false,
            update_info: None,
            update_error: None,
            update_check_completed: false,
            // Custom paths state
            custom_paths: CustomPathsConfig::load().paths,
            new_custom_path_input: String::new(),
            new_custom_path_label: String::new(),
            // Size filter state
            size_filter: SizeFilter::default(),
            size_filter_dropdown_open: false,
            filtered_items: Vec::new(),
            filtered_super_categories: Vec::new(),
            // Trends state
            trend_time_range: TrendTimeRange::default(),
            trend_data: None,
            category_trends: Vec::new(),
            has_trend_data: false,
            trend_snapshot_count: 0,
        }
    }

    pub fn update_storage_info(&mut self) {
        if let Ok(stat) = fs2::statvfs("/") {
            let available = stat.available_space();
            let total = stat.total_space();
            self.storage_available = utils::format_size(available).into();
            self.storage_total = total;
            self.storage_used_fraction = if total > 0 {
                1.0 - (available as f32 / total as f32)
            } else {
                0.0
            };
        }
    }
}
