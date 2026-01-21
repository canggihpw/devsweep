use crate::backend::{CategoryData, StorageBackend};
use crate::types;
use crate::ui::sidebar::Tab;
use crate::ui::ThemeMode;
use crate::utils;
use gpui::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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
    pub total_reclaimable: SharedString,
    pub selected_items_count: i32,
    pub selected_items_size: SharedString,
    pub categories: Vec<CategoryItem>,
    pub all_items: Vec<CleanupItemData>,
    pub category_data: Vec<CategoryData>,
    pub selected_items: Vec<types::CleanupItem>,
    pub quarantine_records: Vec<QuarantineRecordData>,
    pub quarantine_items: Vec<QuarantineItemData>,
    pub quarantine_total_size: SharedString,
    pub quarantine_total_items: i32,
    pub cache_ttls: Vec<CacheTTLSetting>,
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
        let storage_available = if let Ok(stat) = fs2::statvfs("/") {
            utils::format_size(stat.available_space()).into()
        } else {
            "Unknown".into()
        };

        Self {
            backend,
            active_tab: Tab::Scan,
            theme_mode: ThemeMode::default(),
            is_scanning: false,
            is_cleaning: false,
            status_text: "Click 'Scan' to analyze your storage".into(),
            storage_available,
            total_reclaimable: "0 B".into(),
            selected_items_count: 0,
            selected_items_size: "0 B".into(),
            categories: Vec::new(),
            all_items: Vec::new(),
            category_data: Vec::new(),
            selected_items: Vec::new(),
            quarantine_records: Vec::new(),
            quarantine_items: Vec::new(),
            quarantine_total_size: "0 B".into(),
            quarantine_total_items: 0,
            cache_ttls,
        }
    }

    pub fn update_storage_info(&mut self) {
        if let Ok(stat) = fs2::statvfs("/") {
            self.storage_available = utils::format_size(stat.available_space()).into();
        }
    }
}
