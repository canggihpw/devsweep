use crate::app::state::{
    CacheTTLSetting, CategoryItem, CleanupItemData, DevSweep, QuarantineItemData,
    QuarantineRecordData, SizeFilter, SuperCategoryItem, SuperCategoryType,
};
use crate::custom_paths::CustomPathsConfig;
use crate::ui::sidebar::Tab;
use crate::update_checker;
use crate::utils;
use gpui::*;
use std::path::PathBuf;

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

    /// Increase TTL for a category
    pub fn increase_ttl(&mut self, category: &str, _cx: &mut ViewContext<Self>) {
        // Find current TTL
        if let Some(setting) = self
            .cache_ttls
            .iter()
            .find(|s| s.category.as_ref() == category)
        {
            let current_minutes = setting.ttl_minutes;
            // Define step increments: 0 -> 1 -> 5 -> 10 -> 30 -> 60 -> 120 -> 360 -> 720 -> 1440
            let new_minutes = match current_minutes {
                0 => 1,
                1 => 5,
                2..=5 => 10,
                6..=10 => 30,
                11..=30 => 60,
                31..=60 => 120,
                61..=120 => 360,
                121..=360 => 720,
                361..=720 => 1440,
                _ => current_minutes, // Max reached
            };

            let new_seconds = (new_minutes * 60) as u64;
            let mut backend = self.backend.lock().unwrap();
            backend.set_cache_ttl(category, new_seconds);
            drop(backend);
            self.refresh_cache_ttls();
        }
    }

    /// Decrease TTL for a category
    pub fn decrease_ttl(&mut self, category: &str, _cx: &mut ViewContext<Self>) {
        // Find current TTL
        if let Some(setting) = self
            .cache_ttls
            .iter()
            .find(|s| s.category.as_ref() == category)
        {
            let current_minutes = setting.ttl_minutes;
            // Define step decrements: 1440 -> 720 -> 360 -> 120 -> 60 -> 30 -> 10 -> 5 -> 1 -> 0
            let new_minutes = match current_minutes {
                1..=1 => 0,
                2..=5 => 1,
                6..=10 => 5,
                11..=30 => 10,
                31..=60 => 30,
                61..=120 => 60,
                121..=360 => 120,
                361..=720 => 360,
                721..=1440 => 720,
                _ if current_minutes > 1440 => 1440,
                _ => 0, // Already at min
            };

            let new_seconds = (new_minutes * 60) as u64;
            let mut backend = self.backend.lock().unwrap();
            backend.set_cache_ttl(category, new_seconds);
            drop(backend);
            self.refresh_cache_ttls();
        }
    }

    pub fn set_tab(&mut self, tab: Tab) {
        self.active_tab = tab;
        if tab == Tab::Quarantine {
            self.refresh_quarantine();
        } else if tab == Tab::Settings {
            self.refresh_cache_ttls();
        } else if tab == Tab::Trends {
            self.refresh_trends_data();
        }
        // Note: Ports tab auto-refresh is handled in the render method
        // to avoid needing ViewContext here
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
            // Run blocking scan in a separate thread to keep UI responsive
            let result = std::thread::spawn(move || {
                let mut backend = backend.lock().unwrap();
                let cats = backend.scan_with_cache(use_cache);
                let total = backend.get_total_reclaimable();
                (cats, total)
            })
            .join()
            .unwrap_or_else(|_| (Vec::new(), 0));

            let categories = result.0;
            let total = result.1;

            // Update UI on main thread
            let _ = cx.update(|cx| {
                let _ = this.update(cx, |this, cx| {
                    this.category_data = categories.clone();

                    // Convert to UI models with super category assignment
                    this.categories = categories
                        .iter()
                        .map(|c| CategoryItem {
                            name: c.name.clone().into(),
                            size: c.size.clone().into(),
                            total_size: c.total_size,
                            item_count: c.item_count,
                            checked: false,
                            expanded: false,
                            super_category: SuperCategoryType::from_category_name(&c.name),
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

                    // Build super categories (only non-empty ones)
                    this.build_super_categories();

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

        // Update super category checked states
        self.update_super_category_states();
    }

    /// Build super categories from the current categories (filters out empty ones)
    pub fn build_super_categories(&mut self) {
        self.super_categories.clear();

        for super_type in SuperCategoryType::all() {
            // Find all category indices that belong to this super category
            let category_indices: Vec<usize> = self
                .categories
                .iter()
                .enumerate()
                .filter(|(_, cat)| cat.super_category == super_type && cat.item_count > 0)
                .map(|(idx, _)| idx)
                .collect();

            // Skip if no non-empty categories in this super category
            if category_indices.is_empty() {
                continue;
            }

            // Calculate totals
            let total_size: u64 = category_indices
                .iter()
                .map(|&idx| self.categories[idx].total_size)
                .sum();
            let item_count: i32 = category_indices
                .iter()
                .map(|&idx| self.categories[idx].item_count)
                .sum();

            // Check if all categories in this super category are checked
            let all_checked = category_indices
                .iter()
                .all(|&idx| self.categories[idx].checked);

            self.super_categories.push(SuperCategoryItem {
                super_type,
                name: super_type.name().into(),
                icon: super_type.icon().into(),
                total_size,
                size_str: utils::format_size(total_size).into(),
                item_count,
                category_count: category_indices.len() as i32,
                checked: all_checked,
                expanded: false,
                category_indices,
            });
        }

        // Apply size filter to build filtered views
        self.apply_size_filter();
    }

    /// Update super category checked states based on their child categories
    fn update_super_category_states(&mut self) {
        for super_cat in &mut self.super_categories {
            let all_checked = super_cat
                .category_indices
                .iter()
                .all(|&idx| self.categories.get(idx).map(|c| c.checked).unwrap_or(false));
            super_cat.checked = all_checked && !super_cat.category_indices.is_empty();
        }
    }

    /// Toggle a super category's expansion state
    pub fn toggle_super_category_expand(&mut self, super_idx: usize, _cx: &mut ViewContext<Self>) {
        if super_idx < self.super_categories.len() {
            self.super_categories[super_idx].expanded = !self.super_categories[super_idx].expanded;
        }
    }

    /// Toggle all categories within a super category
    pub fn toggle_super_category(&mut self, super_idx: usize, cx: &mut ViewContext<Self>) {
        if super_idx >= self.super_categories.len() {
            return;
        }

        let checked = !self.super_categories[super_idx].checked;
        self.super_categories[super_idx].checked = checked;

        // Toggle all child categories
        let category_indices = self.super_categories[super_idx].category_indices.clone();
        for cat_idx in category_indices {
            if cat_idx < self.categories.len() {
                // Only toggle if the state is different
                if self.categories[cat_idx].checked != checked {
                    self.toggle_category(cat_idx, cx);
                }
            }
        }
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
            // Run blocking cleanup in a separate thread to keep UI responsive
            let result = std::thread::spawn(move || {
                let mut backend = backend.lock().unwrap();
                backend.execute_cleanup_with_history(&items_to_clean, true)
            })
            .join()
            .unwrap_or_else(|_| Err("Cleanup thread panicked".to_string()));

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
            // Run blocking undo in a separate thread to keep UI responsive
            let result = std::thread::spawn(move || {
                let mut backend = backend.lock().unwrap();
                backend.undo_cleanup(&record_id)
            })
            .join()
            .unwrap_or_else(|_| Err("Undo thread panicked".to_string()));

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
            // Run blocking delete in a separate thread to keep UI responsive
            let result = std::thread::spawn(move || {
                let mut backend = backend.lock().unwrap();
                backend.delete_quarantine_item(&record_id, item_index)
            })
            .join()
            .unwrap_or_else(|_| Err("Delete thread panicked".to_string()));

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
            // Run blocking clear in a separate thread to keep UI responsive
            let result = std::thread::spawn(move || {
                let mut backend = backend.lock().unwrap();
                backend.clear_all_quarantine()
            })
            .join()
            .unwrap_or_else(|_| Err("Clear thread panicked".to_string()));

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

    /// Check for updates from GitHub releases
    pub fn check_for_updates(&mut self, cx: &mut ViewContext<Self>) {
        // Don't check if already checking
        if self.is_checking_update {
            return;
        }

        self.is_checking_update = true;
        self.update_error = None;
        cx.notify();

        cx.spawn(|this, mut cx| async move {
            // Run blocking HTTP call in a separate thread
            let result = std::thread::spawn(update_checker::fetch_latest_release)
                .join()
                .unwrap_or_else(|_| {
                    Err(update_checker::UpdateError::NetworkError(
                        "Thread panic".to_string(),
                    ))
                });

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |this, cx| {
                    this.is_checking_update = false;
                    this.update_check_completed = true;

                    match result {
                        Ok(info) => {
                            let current = update_checker::current_version();
                            if update_checker::is_update_available(current, &info.version) {
                                this.update_info = Some(info);
                            } else {
                                this.update_info = None; // Already up to date
                            }
                            this.update_error = None;
                        }
                        Err(e) => {
                            this.update_info = None;
                            this.update_error = Some(e.to_string());
                        }
                    }
                    cx.notify();
                });
            });
        })
        .detach();
    }

    /// Open the GitHub release page in the default browser
    pub fn open_release_page(&self) {
        if let Some(ref info) = self.update_info {
            let _ = std::process::Command::new("open")
                .arg(&info.release_url)
                .spawn();
        }
    }

    /// Open the download URL in the default browser
    pub fn download_update(&self) {
        if let Some(ref info) = self.update_info {
            if let Some(ref url) = info.download_url {
                let _ = std::process::Command::new("open").arg(url).spawn();
            } else {
                // Fallback to release page if no DMG available
                self.open_release_page();
            }
        }
    }

    // ==================== Custom Paths Actions ====================

    /// Refresh custom paths from config
    pub fn refresh_custom_paths(&mut self) {
        self.custom_paths = CustomPathsConfig::load().paths;
    }

    /// Add a new custom path
    pub fn add_custom_path(&mut self, cx: &mut ViewContext<Self>) {
        let path_str = self.new_custom_path_input.trim();
        let label = self.new_custom_path_label.trim();

        if path_str.is_empty() {
            self.status_text = "Please enter a path".into();
            cx.notify();
            return;
        }

        let path = PathBuf::from(path_str);

        // Use path name as label if not provided
        let label = if label.is_empty() {
            path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Custom Path".to_string())
        } else {
            label.to_string()
        };

        let mut config = CustomPathsConfig::load();
        match config.add_path(path, label) {
            Ok(_) => {
                self.status_text = "Custom path added".into();
                self.new_custom_path_input.clear();
                self.new_custom_path_label.clear();
                self.refresh_custom_paths();
            }
            Err(e) => {
                self.status_text = format!("Error: {}", e).into();
            }
        }
        cx.notify();
    }

    /// Remove a custom path by index
    pub fn remove_custom_path(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        let mut config = CustomPathsConfig::load();
        match config.remove_path(index) {
            Ok(_) => {
                self.status_text = "Custom path removed".into();
                self.refresh_custom_paths();
            }
            Err(e) => {
                self.status_text = format!("Error: {}", e).into();
            }
        }
        cx.notify();
    }

    /// Toggle a custom path's enabled status
    pub fn toggle_custom_path(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        let mut config = CustomPathsConfig::load();
        match config.toggle_path(index) {
            Ok(_) => {
                self.refresh_custom_paths();
            }
            Err(e) => {
                self.status_text = format!("Error: {}", e).into();
            }
        }
        cx.notify();
    }

    /// Open folder picker dialog and add selected path
    pub fn browse_for_custom_path(&mut self, cx: &mut ViewContext<Self>) {
        // Use macOS open dialog via shell command
        cx.spawn(|this, mut cx| async move {
            let output = std::process::Command::new("osascript")
                .args([
                    "-e",
                    r#"POSIX path of (choose folder with prompt "Select a folder to scan")"#,
                ])
                .output();

            if let Ok(output) = output {
                if output.status.success() {
                    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();

                    let _ = cx.update(|cx| {
                        let _ = this.update(cx, |this, cx| {
                            this.new_custom_path_input = path;
                            cx.notify();
                        });
                    });
                }
            }
        })
        .detach();
    }

    /// Open text input dialog for manually entering a path
    pub fn type_custom_path(&mut self, cx: &mut ViewContext<Self>) {
        // Use macOS text input dialog via osascript
        cx.spawn(|this, mut cx| async move {
            let output = std::process::Command::new("osascript")
                .args([
                    "-e",
                    r#"text returned of (display dialog "Enter the full path to the folder:" default answer "~/Projects" with title "Add Custom Path")"#,
                ])
                .output();

            if let Ok(output) = output {
                if output.status.success() {
                    let mut path = String::from_utf8_lossy(&output.stdout).trim().to_string();

                    // Expand ~ to home directory
                    if path.starts_with("~/") {
                        if let Ok(home) = std::env::var("HOME") {
                            path = path.replacen("~", &home, 1);
                        }
                    }

                    let _ = cx.update(|cx| {
                        let _ = this.update(cx, |this, cx| {
                            this.new_custom_path_input = path;
                            cx.notify();
                        });
                    });
                }
            }
        })
        .detach();
    }

    // ==================== Dropdown Actions ====================

    /// Toggle the size filter dropdown visibility
    pub fn toggle_size_filter_dropdown(&mut self, _cx: &mut ViewContext<Self>) {
        self.size_filter_dropdown_open = !self.size_filter_dropdown_open;
        // Close other dropdowns
        self.scan_dropdown_open = false;
    }

    /// Toggle the scan dropdown visibility
    pub fn toggle_scan_dropdown(&mut self, _cx: &mut ViewContext<Self>) {
        self.scan_dropdown_open = !self.scan_dropdown_open;
        // Close other dropdowns
        self.size_filter_dropdown_open = false;
    }

    // ==================== Size Filter Actions ====================

    /// Close the size filter dropdown
    pub fn close_size_filter_dropdown(&mut self, _cx: &mut ViewContext<Self>) {
        self.size_filter_dropdown_open = false;
    }

    /// Set the size filter and apply it to the current results
    pub fn set_size_filter(&mut self, filter: SizeFilter, _cx: &mut ViewContext<Self>) {
        self.size_filter = filter;
        self.size_filter_dropdown_open = false;
        self.apply_size_filter();
    }

    /// Apply the current size filter to all_items and rebuild filtered views
    pub fn apply_size_filter(&mut self) {
        let threshold = self.size_filter.threshold_bytes();

        // Filter items based on size threshold
        self.filtered_items = self
            .all_items
            .iter()
            .filter(|item| item.size >= threshold)
            .cloned()
            .collect();

        // Rebuild filtered super categories
        self.build_filtered_super_categories();
    }

    /// Build filtered super categories based on the current size filter
    fn build_filtered_super_categories(&mut self) {
        self.filtered_super_categories.clear();

        let threshold = self.size_filter.threshold_bytes();

        for super_type in SuperCategoryType::all() {
            // Find all category indices that belong to this super category
            // and have items that pass the filter
            let category_indices: Vec<usize> = self
                .categories
                .iter()
                .enumerate()
                .filter(|(idx, cat)| {
                    cat.super_category == super_type && {
                        // Check if any items in this category pass the filter
                        self.all_items
                            .iter()
                            .any(|item| item.category_index == *idx && item.size >= threshold)
                    }
                })
                .map(|(idx, _)| idx)
                .collect();

            // Skip if no categories have items passing the filter
            if category_indices.is_empty() {
                continue;
            }

            // Calculate totals for filtered items only
            let mut total_size: u64 = 0;
            let mut item_count: i32 = 0;

            for &cat_idx in &category_indices {
                for item in &self.all_items {
                    if item.category_index == cat_idx && item.size >= threshold {
                        total_size += item.size;
                        item_count += 1;
                    }
                }
            }

            // Check if all categories in this super category are checked
            let all_checked = category_indices
                .iter()
                .all(|&idx| self.categories[idx].checked);

            self.filtered_super_categories.push(SuperCategoryItem {
                super_type,
                name: super_type.name().into(),
                icon: super_type.icon().into(),
                total_size,
                size_str: utils::format_size(total_size).into(),
                item_count,
                category_count: category_indices.len() as i32,
                checked: all_checked,
                expanded: self
                    .super_categories
                    .iter()
                    .find(|sc| sc.super_type == super_type)
                    .map(|sc| sc.expanded)
                    .unwrap_or(false),
                category_indices,
            });
        }
    }

    /// Get the filtered item count for a category
    pub fn get_filtered_item_count(&self, cat_idx: usize) -> i32 {
        let threshold = self.size_filter.threshold_bytes();
        self.all_items
            .iter()
            .filter(|item| item.category_index == cat_idx && item.size >= threshold)
            .count() as i32
    }

    /// Get the filtered total size for a category
    pub fn get_filtered_category_size(&self, cat_idx: usize) -> u64 {
        let threshold = self.size_filter.threshold_bytes();
        self.all_items
            .iter()
            .filter(|item| item.category_index == cat_idx && item.size >= threshold)
            .map(|item| item.size)
            .sum()
    }

    // ==================== Port Manager Actions ====================

    /// Scan for all listening ports
    pub fn scan_ports(&mut self, cx: &mut ViewContext<Self>) {
        if self.is_scanning_ports {
            return;
        }

        self.is_scanning_ports = true;
        self.port_status = "Scanning ports...".to_string();
        cx.notify();

        cx.spawn(|this, mut cx| async move {
            // Run port scanning in a separate thread
            let processes = std::thread::spawn(crate::port_manager::get_listening_ports)
                .join()
                .unwrap_or_default();

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |this, cx| {
                    this.is_scanning_ports = false;
                    this.port_processes = processes;
                    let count = this.port_processes.len();
                    this.port_status = format!("Found {} listening ports", count);
                    cx.notify();
                });
            });
        })
        .detach();
    }

    /// Set the port filter string
    pub fn set_port_filter(&mut self, filter: String, cx: &mut ViewContext<Self>) {
        self.port_filter = filter;
        cx.notify();
    }

    /// Kill a process using a port
    pub fn kill_port_process(
        &mut self,
        pid: u32,
        port: u16,
        force: bool,
        cx: &mut ViewContext<Self>,
    ) {
        if self.is_killing_process {
            return;
        }

        self.is_killing_process = true;
        self.port_status = format!("Killing process {} on port {}...", pid, port);
        cx.notify();

        cx.spawn(|this, mut cx| async move {
            // Run kill in a separate thread
            let result = std::thread::spawn(move || {
                if force {
                    crate::port_manager::force_kill_process(pid)
                } else {
                    crate::port_manager::kill_process(pid)
                }
            })
            .join()
            .unwrap_or_else(|_| crate::port_manager::KillResult {
                pid,
                port,
                success: false,
                message: "Thread panicked".to_string(),
            });

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |this, cx| {
                    this.is_killing_process = false;

                    if result.success {
                        this.port_status = format!("✓ {}", result.message);
                        // Refresh the port list after successful kill
                        this.scan_ports(cx);
                    } else {
                        this.port_status = format!("✗ {}", result.message);
                        cx.notify();
                    }
                });
            });
        })
        .detach();
    }

    /// Refresh ports (called when switching to Ports tab)
    pub fn refresh_ports(&mut self, cx: &mut ViewContext<Self>) {
        // Auto-scan ports when navigating to the tab
        if self.port_processes.is_empty() {
            self.scan_ports(cx);
        }
    }
}
