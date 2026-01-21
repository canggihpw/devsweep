use crate::app::state::{
    CacheTTLSetting, CategoryItem, CleanupItemData, DevSweep, QuarantineItemData,
    QuarantineRecordData,
};
use crate::ui::sidebar::Tab;
use crate::utils;
use gpui::*;

impl DevSweep {
    pub fn refresh_quarantine(&mut self) {
        let backend = self.backend.lock().unwrap();
        let records = backend.get_quarantine_records();

        self.quarantine_records = records
            .iter()
            .map(|r| {
                use std::time::SystemTime;
                let timestamp_str = r
                    .timestamp
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .ok()
                    .map(|d| {
                        let secs = d.as_secs();
                        let datetime = chrono::DateTime::<chrono::Local>::from(
                            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(secs),
                        );
                        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                    })
                    .unwrap_or_else(|| "Unknown".to_string());

                QuarantineRecordData {
                    id: r.id.clone().into(),
                    timestamp: timestamp_str.into(),
                    total_size: utils::format_size(r.total_size).into(),
                    item_count: r.items.len() as i32,
                    success_count: r.success_count as i32,
                    error_count: r.error_count as i32,
                    can_undo: r.can_undo,
                    expanded: false,
                }
            })
            .collect();

        // Collect all items
        self.quarantine_items = records
            .iter()
            .flat_map(|record| {
                record.items.iter().enumerate().map(|(idx, item)| {
                    let error_msg = item.error_message.clone().unwrap_or_default();
                    QuarantineItemData {
                        item_type: item.item_type.clone().into(),
                        original_path: item.original_path.display().to_string().into(),
                        size_str: utils::format_size(item.size).into(),
                        success: item.success,
                        error_message: error_msg.into(),
                        can_restore: item.can_restore(),
                        deleted_permanently: item.deleted_permanently,
                        record_id: record.id.clone().into(),
                        item_index: idx,
                        quarantine_path: item.quarantine_path.clone(),
                    }
                })
            })
            .collect();

        let stats = backend.get_quarantine_stats();
        self.quarantine_total_size = utils::format_size(stats.quarantine_size).into();
        self.quarantine_total_items = stats.total_items_cleaned as i32;
    }

    pub fn refresh_cache_ttls(&mut self) {
        let backend = self.backend.lock().unwrap();
        let ttls = backend.get_all_cache_ttls();
        self.cache_ttls = ttls
            .iter()
            .map(|(cat, ttl_sec)| CacheTTLSetting {
                category: cat.clone().into(),
                ttl_minutes: (*ttl_sec / 60) as i32,
            })
            .collect();
        self.cache_ttls.sort_by(|a, b| a.category.cmp(&b.category));
    }

    pub fn set_tab(&mut self, tab: Tab) {
        self.active_tab = tab;
        if tab == Tab::Quarantine {
            self.refresh_quarantine();
        } else if tab == Tab::Settings {
            self.refresh_cache_ttls();
        }
    }

    pub fn start_scan(&mut self, use_cache: bool, cx: &mut ViewContext<Self>) {
        // Prevent multiple concurrent scans
        if self.is_scanning || self.is_cleaning {
            return;
        }

        self.is_scanning = true;
        self.status_text = if use_cache {
            "Scanning (using cache)...".into()
        } else {
            "Full scan in progress...".into()
        };
        self.selected_items.clear();
        self.selected_items_count = 0;
        self.selected_items_size = "0 B".into();

        // Notify immediately to update UI and show disabled state
        cx.notify();

        let backend = self.backend.clone();

        cx.spawn(|this, mut cx| async move {
            // Run scan in background
            let result = {
                let mut backend = backend.lock().unwrap();
                let cats = backend.scan_with_cache(use_cache);
                let total = backend.get_total_reclaimable();
                (cats, total)
            };

            let categories = result.0;
            let total = result.1;

            // Update UI on main thread
            let _ = cx.update(|cx| {
                let _ = this.update(cx, |this, cx| {
                    this.category_data = categories.clone();

                    // Convert to UI models
                    this.categories = categories
                        .iter()
                        .map(|c| CategoryItem {
                            name: c.name.clone().into(),
                            size: c.size.clone().into(),
                            total_size: c.total_size,
                            item_count: c.item_count,
                            checked: false,
                            expanded: false,
                        })
                        .collect();

                    this.all_items.clear();
                    for (cat_idx, cat) in categories.iter().enumerate() {
                        for item in &cat.items {
                            let path_str = item
                                .path
                                .as_ref()
                                .map(|p| p.display().to_string())
                                .unwrap_or_default();
                            let warning = item.warning.clone().unwrap_or_default();

                            this.all_items.push(CleanupItemData {
                                item_type: item.item_type.clone().into(),
                                path: path_str.into(),
                                size_str: item.size_str.clone().into(),
                                size: item.size,
                                safe_to_delete: item.safe_to_delete,
                                warning: warning.into(),
                                has_warning: item.warning.is_some(),
                                selected: false,
                                category_index: cat_idx,
                            });
                        }
                    }

                    this.total_reclaimable = utils::format_size(total).into();
                    this.is_scanning = false;

                    if total > 0 {
                        this.status_text =
                            format!("Found {} that can be cleaned", utils::format_size(total))
                                .into();
                    } else {
                        this.status_text = "No cleanable files found".into();
                    }

                    this.update_storage_info();
                    cx.notify();
                });
            });
        })
        .detach();
    }

    pub fn toggle_category(&mut self, index: usize, _cx: &mut ViewContext<Self>) {
        if index >= self.categories.len() {
            return;
        }

        let checked = !self.categories[index].checked;
        self.categories[index].checked = checked;

        // Update all items in this category
        if let Some(cat_data) = self.category_data.get(index) {
            if checked {
                // Add all items from this category to selected
                for item in &cat_data.items {
                    if !self
                        .selected_items
                        .iter()
                        .any(|si| si.item_type == item.item_type && si.path == item.path)
                    {
                        self.selected_items.push(item.clone());
                    }
                }
            } else {
                // Remove all items from this category
                self.selected_items.retain(|si| {
                    !cat_data
                        .items
                        .iter()
                        .any(|ci| ci.item_type == si.item_type && ci.path == si.path)
                });
            }

            // Update item checkboxes
            for item in &mut self.all_items {
                if item.category_index == index {
                    item.selected = checked;
                }
            }
        }

        self.update_selection_counts();
    }

    pub fn toggle_category_expand(&mut self, index: usize, _cx: &mut ViewContext<Self>) {
        if index < self.categories.len() {
            self.categories[index].expanded = !self.categories[index].expanded;
        }
    }

    pub fn toggle_item(&mut self, index: usize, _cx: &mut ViewContext<Self>) {
        if index >= self.all_items.len() {
            return;
        }

        let item_data = &mut self.all_items[index];
        item_data.selected = !item_data.selected;
        let selected = item_data.selected;
        let cat_idx = item_data.category_index;
        let item_type = item_data.item_type.to_string();
        let item_path = item_data.path.to_string();

        // Find the backend item
        if let Some(cat) = self.category_data.get(cat_idx) {
            if let Some(backend_item) = cat.items.iter().find(|bi| {
                bi.item_type == item_type
                    && bi.path.as_ref().map(|p| p.display().to_string()).as_deref()
                        == Some(&item_path)
            }) {
                if selected {
                    if !self.selected_items.iter().any(|si| {
                        si.item_type == backend_item.item_type && si.path == backend_item.path
                    }) {
                        self.selected_items.push(backend_item.clone());
                    }
                } else {
                    self.selected_items.retain(|si| {
                        !(si.item_type == backend_item.item_type && si.path == backend_item.path)
                    });
                }
            }

            // Update category checkbox
            let all_selected = cat.items.iter().all(|ci| {
                self.selected_items
                    .iter()
                    .any(|si| si.item_type == ci.item_type && si.path == ci.path)
            });
            self.categories[cat_idx].checked = all_selected && !cat.items.is_empty();
        }

        self.update_selection_counts();
    }

    pub fn select_all(&mut self, _cx: &mut ViewContext<Self>) {
        for cat in &mut self.categories {
            cat.checked = true;
        }
        for item in &mut self.all_items {
            item.selected = true;
        }
        self.selected_items.clear();
        for cat in &self.category_data {
            self.selected_items.extend(cat.items.clone());
        }
        self.update_selection_counts();
    }

    pub fn deselect_all(&mut self, _cx: &mut ViewContext<Self>) {
        for cat in &mut self.categories {
            cat.checked = false;
        }
        for item in &mut self.all_items {
            item.selected = false;
        }
        self.selected_items.clear();
        self.update_selection_counts();
    }

    pub fn update_selection_counts(&mut self) {
        let total_size: u64 = self.selected_items.iter().map(|si| si.size).sum();
        self.selected_items_count = self.selected_items.len() as i32;
        self.selected_items_size = utils::format_size(total_size).into();
    }

    pub fn execute_cleanup(&mut self, cx: &mut ViewContext<Self>) {
        if self.selected_items.is_empty() {
            self.status_text = "No items selected for cleanup".into();
            cx.notify();
            return;
        }

        self.is_cleaning = true;
        self.status_text = format!("Deleting {} items...", self.selected_items.len()).into();
        cx.notify();

        let backend = self.backend.clone();
        let items_to_clean = self.selected_items.clone();

        cx.spawn(|this, mut cx| async move {
            let result = {
                let mut backend = backend.lock().unwrap();
                backend.execute_cleanup_with_history(&items_to_clean, true)
            };

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |this, cx| {
                    this.is_cleaning = false;

                    match result {
                        Ok(msg) => {
                            this.status_text = format!("✓ {}", msg).into();
                        }
                        Err(e) => {
                            this.status_text = format!("⚠ {}", e).into();
                        }
                    }

                    // Trigger rescan
                    this.start_scan(true, cx);
                });
            });
        })
        .detach();
    }

    pub fn toggle_quarantine_record_expand(&mut self, index: usize, _cx: &mut ViewContext<Self>) {
        if index < self.quarantine_records.len() {
            self.quarantine_records[index].expanded = !self.quarantine_records[index].expanded;
        }
    }

    pub fn undo_record(&mut self, record_id: String, cx: &mut ViewContext<Self>) {
        self.is_cleaning = true;
        self.status_text = "Undoing cleanup...".into();
        cx.notify();

        let backend = self.backend.clone();

        cx.spawn(|this, mut cx| async move {
            let result = {
                let mut backend = backend.lock().unwrap();
                backend.undo_cleanup(&record_id)
            };

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |this, cx| {
                    this.is_cleaning = false;

                    match result {
                        Ok(msg) => {
                            this.status_text = format!("✓ {}", msg).into();
                        }
                        Err(e) => {
                            this.status_text = format!("✗ {}", e).into();
                        }
                    }

                    this.refresh_quarantine();
                    this.update_storage_info();
                    cx.notify();
                });
            });
        })
        .detach();
    }

    pub fn delete_quarantine_item(
        &mut self,
        record_id: String,
        item_index: usize,
        cx: &mut ViewContext<Self>,
    ) {
        self.is_cleaning = true;
        self.status_text = "Deleting item...".into();
        cx.notify();

        let backend = self.backend.clone();

        cx.spawn(|this, mut cx| async move {
            let result = {
                let mut backend = backend.lock().unwrap();
                backend.delete_quarantine_item(&record_id, item_index)
            };

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |this, cx| {
                    this.is_cleaning = false;

                    match result {
                        Ok(msg) => {
                            this.status_text = format!("✓ {}", msg).into();
                        }
                        Err(e) => {
                            this.status_text = format!("✗ {}", e).into();
                        }
                    }

                    this.refresh_quarantine();
                    this.update_storage_info();
                    cx.notify();
                });
            });
        })
        .detach();
    }

    pub fn clear_all_quarantine(&mut self, cx: &mut ViewContext<Self>) {
        self.is_cleaning = true;
        self.status_text = "Clearing quarantine...".into();
        cx.notify();

        let backend = self.backend.clone();

        cx.spawn(|this, mut cx| async move {
            let result = {
                let mut backend = backend.lock().unwrap();
                backend.clear_all_quarantine()
            };

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |this, cx| {
                    this.is_cleaning = false;

                    match result {
                        Ok(msg) => {
                            this.status_text = format!("✓ {}", msg).into();
                        }
                        Err(e) => {
                            this.status_text = format!("✗ {}", e).into();
                        }
                    }

                    this.refresh_quarantine();
                    this.update_storage_info();
                    cx.notify();
                });
            });
        })
        .detach();
    }

    pub fn reset_cache_defaults(&mut self, _cx: &mut ViewContext<Self>) {
        let mut backend = self.backend.lock().unwrap();
        backend.reset_cache_config();
        drop(backend);
        self.refresh_cache_ttls();
    }
}
