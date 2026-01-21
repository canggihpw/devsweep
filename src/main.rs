mod backend;
mod cache_settings;
mod checkers;
mod cleanup_history;
mod scan_cache;
mod types;
mod ui;
mod utils;

use backend::{CategoryData, StorageBackend};
use gpui::prelude::FluentBuilder;
use gpui::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use ui::sidebar::Tab;
use ui::{Theme, ThemeMode};

// UI Data structures
#[derive(Clone)]
struct CategoryItem {
    name: SharedString,
    size: SharedString,
    #[allow(dead_code)]
    total_size: u64,
    item_count: i32,
    checked: bool,
    expanded: bool,
}

#[derive(Clone)]
struct CleanupItemData {
    item_type: SharedString,
    path: SharedString,
    size_str: SharedString,
    #[allow(dead_code)]
    size: u64,
    safe_to_delete: bool,
    #[allow(dead_code)]
    warning: SharedString,
    has_warning: bool,
    selected: bool,
    category_index: usize,
}

#[derive(Clone)]
struct QuarantineRecordData {
    id: SharedString,
    timestamp: SharedString,
    total_size: SharedString,
    item_count: i32,
    success_count: i32,
    error_count: i32,
    can_undo: bool,
    expanded: bool,
}

#[derive(Clone)]
struct QuarantineItemData {
    item_type: SharedString,
    original_path: SharedString,
    size_str: SharedString,
    success: bool,
    error_message: SharedString,
    #[allow(dead_code)]
    can_restore: bool,
    deleted_permanently: bool,
    record_id: SharedString,
    item_index: usize,
    quarantine_path: Option<PathBuf>,
}

#[derive(Clone)]
struct CacheTTLSetting {
    category: SharedString,
    ttl_minutes: i32,
}

// Main application view
struct DevCleaner {
    backend: Arc<Mutex<StorageBackend>>,
    active_tab: Tab,
    theme_mode: ThemeMode,
    is_scanning: bool,
    is_cleaning: bool,
    status_text: SharedString,
    storage_available: SharedString,
    total_reclaimable: SharedString,
    selected_items_count: i32,
    selected_items_size: SharedString,
    categories: Vec<CategoryItem>,
    all_items: Vec<CleanupItemData>,
    category_data: Vec<CategoryData>,
    selected_items: Vec<types::CleanupItem>,
    quarantine_records: Vec<QuarantineRecordData>,
    quarantine_items: Vec<QuarantineItemData>,
    quarantine_total_size: SharedString,
    quarantine_total_items: i32,
    cache_ttls: Vec<CacheTTLSetting>,
}

impl DevCleaner {
    fn new() -> Self {
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

    fn update_storage_info(&mut self) {
        if let Ok(stat) = fs2::statvfs("/") {
            self.storage_available = utils::format_size(stat.available_space()).into();
        }
    }

    fn refresh_quarantine(&mut self) {
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
                    .and_then(|d| {
                        let secs = d.as_secs();
                        let datetime = chrono::DateTime::<chrono::Local>::from(
                            SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(secs),
                        );
                        Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
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

    fn refresh_cache_ttls(&mut self) {
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

    fn set_tab(&mut self, tab: Tab) {
        self.active_tab = tab;
        if tab == Tab::Quarantine {
            self.refresh_quarantine();
        } else if tab == Tab::Settings {
            self.refresh_cache_ttls();
        }
    }

    fn start_scan(&mut self, use_cache: bool, cx: &mut ViewContext<Self>) {
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
            let result: (Vec<CategoryData>, u64) = {
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

    fn toggle_category(&mut self, index: usize, _cx: &mut ViewContext<Self>) {
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

    fn toggle_category_expand(&mut self, index: usize, _cx: &mut ViewContext<Self>) {
        if index < self.categories.len() {
            self.categories[index].expanded = !self.categories[index].expanded;
        }
    }

    fn toggle_item(&mut self, index: usize, _cx: &mut ViewContext<Self>) {
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

    fn select_all(&mut self, _cx: &mut ViewContext<Self>) {
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

    fn deselect_all(&mut self, _cx: &mut ViewContext<Self>) {
        for cat in &mut self.categories {
            cat.checked = false;
        }
        for item in &mut self.all_items {
            item.selected = false;
        }
        self.selected_items.clear();
        self.update_selection_counts();
    }

    fn update_selection_counts(&mut self) {
        let total_size: u64 = self.selected_items.iter().map(|si| si.size).sum();
        self.selected_items_count = self.selected_items.len() as i32;
        self.selected_items_size = utils::format_size(total_size).into();
    }

    fn execute_cleanup(&mut self, cx: &mut ViewContext<Self>) {
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

    fn toggle_quarantine_record_expand(&mut self, index: usize, _cx: &mut ViewContext<Self>) {
        if index < self.quarantine_records.len() {
            self.quarantine_records[index].expanded = !self.quarantine_records[index].expanded;
        }
    }

    fn undo_record(&mut self, record_id: String, cx: &mut ViewContext<Self>) {
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

    fn delete_quarantine_item(
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

    fn clear_all_quarantine(&mut self, cx: &mut ViewContext<Self>) {
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

    fn reset_cache_defaults(&mut self, _cx: &mut ViewContext<Self>) {
        let mut backend = self.backend.lock().unwrap();
        backend.reset_cache_config();
        drop(backend);
        self.refresh_cache_ttls();
    }
}

impl Render for DevCleaner {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let active_tab = self.active_tab;

        div()
            .flex()
            .w_full()
            .h_full()
            .bg(Theme::base(self.theme_mode))
            // Sidebar
            .child(self.render_sidebar(cx))
            // Main content area
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .overflow_hidden()
                    .child(match active_tab {
                        Tab::Scan => self.render_scan_tab(cx),
                        Tab::Quarantine => self.render_quarantine_tab(cx),
                        Tab::Settings => self.render_settings_tab(cx),
                        Tab::About => self.render_about_tab(),
                    }),
            )
    }
}

impl DevCleaner {
    fn render_sidebar(&mut self, cx: &mut ViewContext<Self>) -> Div {
        let active_tab = self.active_tab;
        let storage_available = self.storage_available.clone();

        div()
            .w(px(200.0))
            .h_full()
            .bg(Theme::mantle(self.theme_mode))
            .border_r_1()
            .border_color(Theme::surface0(self.theme_mode))
            .flex()
            .flex_col()
            // Logo and title
            .child(
                div()
                    .w_full()
                    .p_4()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .border_b_1()
                    .border_color(Theme::surface0(self.theme_mode))
                    .child(svg().path(self.theme_mode.icon_path()).size(px(48.0)))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(Theme::text(self.theme_mode))
                            .child("DevSweep"),
                    ),
            )
            // Navigation items
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .p_2()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(self.sidebar_item(Tab::Scan, active_tab == Tab::Scan, cx))
                    .child(self.sidebar_item(Tab::Quarantine, active_tab == Tab::Quarantine, cx))
                    .child(self.sidebar_item(Tab::Settings, active_tab == Tab::Settings, cx))
                    .child(self.sidebar_item(Tab::About, active_tab == Tab::About, cx)),
            )
            // Theme toggle and storage info at bottom
            .child(
                div()
                    .w_full()
                    .p_4()
                    .border_t_1()
                    .border_color(Theme::surface0(self.theme_mode))
                    .flex()
                    .flex_col()
                    .gap_3()
                    // Theme toggle
                    .child(
                        div()
                            .id("theme-toggle")
                            .w_full()
                            .px_3()
                            .py_2()
                            .flex()
                            .items_center()
                            .justify_between()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(Theme::surface0(self.theme_mode))
                            .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
                            .active(|style| style.bg(Theme::surface2(self.theme_mode)).opacity(0.9))
                            .on_click(cx.listener(|this, _event, cx| {
                                this.theme_mode = this.theme_mode.toggle();
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_sm().child(if self.theme_mode.is_dark() {
                                        "🌙"
                                    } else {
                                        "☀️"
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::text(self.theme_mode))
                                            .child(if self.theme_mode.is_dark() {
                                                "Dark Mode"
                                            } else {
                                                "Light Mode"
                                            }),
                                    ),
                            ),
                    )
                    // Storage info
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(Theme::subtext0(self.theme_mode))
                                    .child("Available Storage"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(Theme::green(self.theme_mode))
                                    .child(storage_available),
                            ),
                    ),
            )
    }

    fn sidebar_item(
        &self,
        tab: Tab,
        is_active: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        div()
            .id(SharedString::from(format!("tab-{:?}", tab)))
            .w_full()
            .px_3()
            .py_2()
            .flex()
            .items_center()
            .gap_3()
            .rounded_md()
            .cursor_pointer()
            .bg(if is_active {
                Theme::surface0(self.theme_mode)
            } else {
                Theme::transparent()
            })
            .hover(|style| {
                if is_active {
                    style
                } else {
                    style.bg(Theme::surface0(self.theme_mode))
                }
            })
            .active(|style| style.opacity(0.7))
            .on_click(cx.listener(move |this, _event, cx| {
                this.set_tab(tab);
                cx.notify();
            }))
            .child(div().text_lg().child(tab.icon()))
            .child(
                div()
                    .text_sm()
                    .text_color(if is_active {
                        Theme::text(self.theme_mode)
                    } else {
                        Theme::subtext0(self.theme_mode)
                    })
                    .font_weight(if is_active {
                        FontWeight::SEMIBOLD
                    } else {
                        FontWeight::NORMAL
                    })
                    .child(tab.label()),
            )
    }

    fn render_scan_tab(&mut self, cx: &mut ViewContext<Self>) -> Div {
        let is_scanning = self.is_scanning;
        let is_cleaning = self.is_cleaning;
        let status_text = self.status_text.clone();
        let total_reclaimable = self.total_reclaimable.clone();
        let selected_count = self.selected_items_count;
        let selected_size = self.selected_items_size.clone();
        let categories = self.categories.clone();
        let items = self.all_items.clone();

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .bg(Theme::base(self.theme_mode))
            // Header
            .child(
                div()
                    .w_full()
                    .p_4()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(Theme::surface0(self.theme_mode))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(Theme::text(self.theme_mode))
                                    .child("Scan & Clean"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::subtext0(self.theme_mode))
                                    .child(status_text),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .when(is_scanning || is_cleaning, |d| {
                                d.child(
                                    div()
                                        .px_4()
                                        .py_2()
                                        .bg(Theme::surface1(self.theme_mode))
                                        .rounded_md()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(Theme::subtext0(self.theme_mode))
                                                .child(if is_scanning {
                                                    "Scanning..."
                                                } else {
                                                    "Cleaning..."
                                                }),
                                        ),
                                )
                            })
                            .when(!is_scanning && !is_cleaning, |d| {
                                d.child(
                                    div()
                                        .id("scan-btn")
                                        .px_4()
                                        .py_2()
                                        .bg(Theme::blue(self.theme_mode))
                                        .rounded_md()
                                        .cursor_pointer()
                                        .hover(|style| style.bg(Theme::sapphire(self.theme_mode)))
                                        .active(|style| {
                                            style
                                                .bg(Theme::blue_active(self.theme_mode))
                                                .opacity(0.9)
                                        })
                                        .on_click(cx.listener(|this, _event, cx| {
                                            if !this.is_scanning && !this.is_cleaning {
                                                this.start_scan(true, cx);
                                            }
                                        }))
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(Theme::crust(self.theme_mode))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Scan"),
                                        ),
                                )
                                .child(
                                    div()
                                        .id("force-rescan-btn")
                                        .px_2()
                                        .py_2()
                                        .cursor_pointer()
                                        .hover(|style| style.bg(Theme::surface0(self.theme_mode)))
                                        .active(|style| {
                                            style.bg(Theme::surface1(self.theme_mode)).opacity(0.9)
                                        })
                                        .on_click(cx.listener(|this, _event, cx| {
                                            if !this.is_scanning && !this.is_cleaning {
                                                this.start_scan(false, cx);
                                            }
                                        }))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(Theme::subtext0(self.theme_mode))
                                                .child("Full Rescan"),
                                        ),
                                )
                            }),
                    ),
            )
            // Stats bar
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_3()
                    .bg(Theme::mantle(self.theme_mode))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_6()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::subtext0(self.theme_mode))
                                            .child("Total Reclaimable:"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(Theme::peach(self.theme_mode))
                                            .child(total_reclaimable),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::subtext0(self.theme_mode))
                                            .child("Selected:"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(Theme::blue(self.theme_mode))
                                            .child(format!(
                                                "{} items ({})",
                                                selected_count, selected_size
                                            )),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .id("select-all-btn")
                                    .px_4()
                                    .py_2()
                                    .bg(Theme::surface0(self.theme_mode))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
                                    .active(|style| {
                                        style.bg(Theme::surface2(self.theme_mode)).opacity(0.9)
                                    })
                                    .on_click(cx.listener(|this, _event, cx| {
                                        this.select_all(cx);
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::text(self.theme_mode))
                                            .child("Select All"),
                                    ),
                            )
                            .child(
                                div()
                                    .id("deselect-all-btn")
                                    .px_4()
                                    .py_2()
                                    .bg(Theme::surface0(self.theme_mode))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
                                    .active(|style| {
                                        style.bg(Theme::surface2(self.theme_mode)).opacity(0.9)
                                    })
                                    .on_click(cx.listener(|this, _event, cx| {
                                        this.deselect_all(cx);
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::text(self.theme_mode))
                                            .child("Deselect All"),
                                    ),
                            )
                            .when(selected_count > 0 && !is_cleaning, |d| {
                                d.child(
                                    div()
                                        .id("clean-btn")
                                        .px_4()
                                        .py_2()
                                        .bg(Theme::red(self.theme_mode))
                                        .rounded_md()
                                        .cursor_pointer()
                                        .hover(|style| style.bg(Theme::red_hover(self.theme_mode)))
                                        .active(|style| {
                                            style
                                                .bg(Theme::red_active(self.theme_mode))
                                                .opacity(0.9)
                                        })
                                        .on_click(cx.listener(|this, _event, cx| {
                                            this.execute_cleanup(cx);
                                        }))
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(Theme::crust(self.theme_mode))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Clean Selected"),
                                        ),
                                )
                            })
                            .when(selected_count == 0 || is_cleaning, |d| {
                                d.child(
                                    div()
                                        .px_4()
                                        .py_2()
                                        .bg(Theme::surface1(self.theme_mode))
                                        .rounded_md()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(Theme::overlay0(self.theme_mode))
                                                .child("Clean Selected"),
                                        ),
                                )
                            }),
                    ),
            )
            // Categories list
            .child(
                div()
                    .id("scan-content")
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .child(if categories.is_empty() {
                        self.empty_state("Click 'Scan' to analyze your storage")
                    } else {
                        div().w_full().flex().flex_col().children(
                            categories.iter().enumerate().map(|(cat_idx, category)| {
                                let cat_items: Vec<_> = items
                                    .iter()
                                    .filter(|item| item.category_index == cat_idx)
                                    .cloned()
                                    .collect();

                                self.render_category_section(
                                    category.clone(),
                                    cat_items,
                                    cat_idx,
                                    cx,
                                )
                            }),
                        )
                    }),
            )
    }

    fn empty_state(&self, message: &str) -> Div {
        div()
            .w_full()
            .py_8()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2()
            .child(div().text_2xl().child("📭"))
            .child(
                div()
                    .text_sm()
                    .text_color(Theme::subtext0(self.theme_mode))
                    .child(message.to_string()),
            )
    }

    fn render_category_section(
        &self,
        category: CategoryItem,
        items: Vec<CleanupItemData>,
        cat_idx: usize,
        cx: &mut ViewContext<Self>,
    ) -> Div {
        let expanded = category.expanded;

        div()
            .w_full()
            .flex()
            .flex_col()
            .border_b_1()
            .border_color(Theme::surface0(self.theme_mode))
            // Category header
            .child(
                div()
                    .id(SharedString::from(format!("cat-header-{}", cat_idx)))
                    .w_full()
                    .px_4()
                    .py_3()
                    .flex()
                    .items_center()
                    .gap_3()
                    .bg(Theme::surface0(self.theme_mode))
                    .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
                    .active(|style| style.bg(Theme::surface2(self.theme_mode)).opacity(0.9))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _event, cx| {
                        this.toggle_category_expand(cat_idx, cx);
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::subtext0(self.theme_mode))
                            .child(if expanded { "▼" } else { "▶" }),
                    )
                    // Checkbox
                    .child(
                        div()
                            .id(SharedString::from(format!("cat-check-{}", cat_idx)))
                            .w_4()
                            .h_4()
                            .rounded_sm()
                            .border_1()
                            .border_color(if category.checked {
                                Theme::blue(self.theme_mode)
                            } else {
                                Theme::surface2(self.theme_mode)
                            })
                            .bg(if category.checked {
                                Theme::blue(self.theme_mode)
                            } else {
                                Theme::transparent()
                            })
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .active(|style| style.opacity(0.7))
                            .on_click(cx.listener(move |this, _event, cx| {
                                this.toggle_category(cat_idx, cx);
                                cx.notify();
                            }))
                            .when(category.checked, |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(Theme::crust(self.theme_mode))
                                        .child("✓"),
                                )
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(Theme::text(self.theme_mode))
                            .child(category.name.clone()),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(Theme::surface1(self.theme_mode))
                            .rounded_sm()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(Theme::subtext0(self.theme_mode))
                                    .child(format!("{} items", category.item_count)),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(Theme::peach(self.theme_mode))
                            .child(category.size.clone()),
                    ),
            )
            // Items (if expanded)
            .when(expanded, |d| {
                d.child(
                    div()
                        .w_full()
                        .flex()
                        .flex_col()
                        .bg(Theme::base(self.theme_mode))
                        .children(items.iter().enumerate().map(|(_item_idx, item)| {
                            let global_idx = self
                                .all_items
                                .iter()
                                .position(|i| {
                                    i.item_type == item.item_type
                                        && i.path == item.path
                                        && i.category_index == cat_idx
                                })
                                .unwrap_or(0);
                            self.render_cleanup_item(item.clone(), global_idx, cx)
                        })),
                )
            })
    }

    fn render_cleanup_item(
        &self,
        item: CleanupItemData,
        global_idx: usize,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let selected = item.selected;
        let has_warning = item.has_warning;
        let safe_to_delete = item.safe_to_delete;
        let path_empty = item.path.is_empty();

        div()
            .id(SharedString::from(format!("item-{}", global_idx)))
            .w_full()
            .px_4()
            .pl_12()
            .py_2()
            .flex()
            .items_center()
            .gap_3()
            .hover(|style| style.bg(Theme::surface0(self.theme_mode)))
            .active(|style| style.bg(Theme::surface1(self.theme_mode)).opacity(0.9))
            .cursor_pointer()
            .border_b_1()
            .border_color(Theme::border_subtle(self.theme_mode))
            .on_click(cx.listener(move |this, _event, cx| {
                this.toggle_item(global_idx, cx);
                cx.notify();
            }))
            // Checkbox
            .child(
                div()
                    .w_4()
                    .h_4()
                    .rounded_sm()
                    .border_1()
                    .border_color(if selected {
                        Theme::blue(self.theme_mode)
                    } else {
                        Theme::surface2(self.theme_mode)
                    })
                    .bg(if selected {
                        Theme::blue(self.theme_mode)
                    } else {
                        Theme::transparent()
                    })
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(selected, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(Theme::crust(self.theme_mode))
                                .child("✓"),
                        )
                    }),
            )
            // Item info
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::text(self.theme_mode))
                            .child(item.item_type.clone()),
                    )
                    .when(!path_empty, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(Theme::overlay0(self.theme_mode))
                                .child(item.path.clone()),
                        )
                    }),
            )
            // Warning indicator
            .when(has_warning, |d| {
                d.child(
                    div()
                        .px_2()
                        .py_1()
                        .bg(Theme::yellow(self.theme_mode))
                        .rounded_sm()
                        .child(
                            div()
                                .text_xs()
                                .text_color(Theme::crust(self.theme_mode))
                                .child("⚠"),
                        ),
                )
            })
            // Safe badge
            .when(safe_to_delete, |d| {
                d.child(
                    div()
                        .px_2()
                        .py_1()
                        .bg(Theme::green(self.theme_mode))
                        .rounded_sm()
                        .child(
                            div()
                                .text_xs()
                                .text_color(Theme::crust(self.theme_mode))
                                .child("Safe"),
                        ),
                )
            })
            // Size
            .child(
                div()
                    .text_sm()
                    .text_color(Theme::subtext1(self.theme_mode))
                    .child(item.size_str.clone()),
            )
    }

    fn render_quarantine_tab(&mut self, cx: &mut ViewContext<Self>) -> Div {
        let is_cleaning = self.is_cleaning;
        let status_text = self.status_text.clone();
        let total_size = self.quarantine_total_size.clone();
        let total_items = self.quarantine_total_items;
        let records = self.quarantine_records.clone();
        let items = self.quarantine_items.clone();
        let records_empty = records.is_empty();

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .bg(Theme::base(self.theme_mode))
            // Header
            .child(
                div()
                    .w_full()
                    .p_4()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(Theme::surface0(self.theme_mode))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(div().text_xl().font_weight(FontWeight::BOLD).text_color(Theme::text(self.theme_mode)).child("Quarantine"))
                            .child(div().text_sm().text_color(Theme::subtext0(self.theme_mode)).child(status_text))
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .id("refresh-quarantine-btn")
                                    .px_4()
                                    .py_2()
                                    .bg(Theme::surface0(self.theme_mode))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
                                    .active(|style| style.bg(Theme::surface2(self.theme_mode)).opacity(0.9))
                                    .on_click(cx.listener(|this, _event, cx| {
                                        this.refresh_quarantine();
                                        cx.notify();
                                    }))
                                    .child(div().text_sm().text_color(Theme::text(self.theme_mode)).child("Refresh"))
                            )
                            .child(
                                div()
                                    .id("open-finder-btn")
                                    .px_4()
                                    .py_2()
                                    .bg(Theme::surface0(self.theme_mode))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
                                    .active(|style| style.bg(Theme::surface2(self.theme_mode)).opacity(0.9))
                                    .on_click(cx.listener(|_this, _event, _cx| {
                                        if let Some(cache_dir) = dirs::cache_dir() {
                                            let quarantine_path = cache_dir.join("development-cleaner").join("quarantine");
                                            let _ = std::fs::create_dir_all(&quarantine_path);
                                            let _ = std::process::Command::new("open").arg(quarantine_path).spawn();
                                        }
                                    }))
                                    .child(div().text_sm().text_color(Theme::text(self.theme_mode)).child("Open in Finder"))
                            )
                            .when(!records_empty && !is_cleaning, |d| {
                                d.child(
                                    div()
                                        .id("clear-all-btn")
                                        .px_4()
                                        .py_2()
                                        .bg(Theme::red(self.theme_mode))
                                        .rounded_md()
                                        .cursor_pointer()
                                        .hover(|style| style.bg(Theme::red_hover(self.theme_mode)))
                                        .active(|style| style.bg(Theme::red_active(self.theme_mode)).opacity(0.9))
                                        .on_click(cx.listener(|this, _event, cx| {
                                            this.clear_all_quarantine(cx);
                                        }))
                                        .child(div().text_sm().text_color(Theme::crust(self.theme_mode)).font_weight(FontWeight::SEMIBOLD).child("Delete All"))
                                )
                            })
                            .when(records_empty || is_cleaning, |d| {
                                d.child(
                                    div()
                                        .px_4()
                                        .py_2()
                                        .bg(Theme::surface1(self.theme_mode))
                                        .rounded_md()
                                        .child(div().text_sm().text_color(Theme::overlay0(self.theme_mode)).child("Delete All"))
                                )
                            })
                    )
            )
            // Stats bar
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_3()
                    .bg(Theme::mantle(self.theme_mode))
                    .flex()
                    .items_center()
                    .gap_6()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_sm().text_color(Theme::subtext0(self.theme_mode)).child("Quarantine Size:"))
                            .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(Theme::peach(self.theme_mode)).child(total_size))
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_sm().text_color(Theme::subtext0(self.theme_mode)).child("Total Items:"))
                            .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(Theme::blue(self.theme_mode)).child(format!("{}", total_items)))
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_sm().text_color(Theme::subtext0(self.theme_mode)).child("Records:"))
                            .child(div().text_sm().font_weight(FontWeight::BOLD).text_color(Theme::lavender(self.theme_mode)).child(format!("{}", records.len())))
                    )
            )
            // Info banner
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_2()
                    .bg(Theme::blue_tint(self.theme_mode))
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().text_sm().child("ℹ️"))
                    .child(div().text_sm().text_color(Theme::blue(self.theme_mode)).child("Deleted files are quarantined for undo support. They are automatically cleaned when exceeding 10GB."))
            )
            // Records list
            .child(
                div()
                    .id("quarantine-content")
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .child(
                        if records_empty {
                            self.empty_state("No cleanup history yet")
                        } else {
                            div()
                                .w_full()
                                .flex()
                                .flex_col()
                                .children(
                                    records.iter().enumerate().map(|(record_idx, record)| {
                                        let record_items: Vec<_> = items.iter()
                                            .filter(|item| item.record_id == record.id)
                                            .cloned()
                                            .collect();

                                        self.render_quarantine_record(record.clone(), record_items, record_idx, cx)
                                    })
                                )
                        }
                    )
            )
    }

    fn render_quarantine_record(
        &self,
        record: QuarantineRecordData,
        items: Vec<QuarantineItemData>,
        record_idx: usize,
        cx: &mut ViewContext<Self>,
    ) -> Div {
        let record_id = record.id.to_string();
        let expanded = record.expanded;
        let can_undo = record.can_undo;
        let has_errors = record.error_count > 0;

        div()
            .w_full()
            .flex()
            .flex_col()
            .border_b_1()
            .border_color(Theme::surface0(self.theme_mode))
            // Record header
            .child(
                div()
                    .id(SharedString::from(format!("qr-header-{}", record_idx)))
                    .w_full()
                    .px_4()
                    .py_3()
                    .flex()
                    .items_center()
                    .gap_3()
                    .bg(Theme::surface0(self.theme_mode))
                    .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
                    .active(|style| style.bg(Theme::surface2(self.theme_mode)).opacity(0.9))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _event, cx| {
                        this.toggle_quarantine_record_expand(record_idx, cx);
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::subtext0(self.theme_mode))
                            .child(if expanded { "▼" } else { "▶" }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(Theme::text(self.theme_mode))
                                    .child(record.timestamp.clone()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(Theme::green(self.theme_mode))
                                            .child(format!("✓ {}", record.success_count)),
                                    )
                                    .when(has_errors, |d| {
                                        d.child(
                                            div()
                                                .text_xs()
                                                .text_color(Theme::red(self.theme_mode))
                                                .child(format!("✗ {}", record.error_count)),
                                        )
                                    }),
                            ),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(Theme::surface1(self.theme_mode))
                            .rounded_sm()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(Theme::subtext0(self.theme_mode))
                                    .child(format!("{} items", record.item_count)),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(Theme::peach(self.theme_mode))
                            .child(record.total_size.clone()),
                    )
                    .when(can_undo, |d| {
                        let record_id_clone = record_id.clone();
                        d.child(
                            div()
                                .id(SharedString::from(format!("undo-btn-{}", record_idx)))
                                .px_3()
                                .py_1()
                                .bg(Theme::blue(self.theme_mode))
                                .rounded_md()
                                .cursor_pointer()
                                .hover(|style| style.bg(Theme::sapphire(self.theme_mode)))
                                .active(|style| {
                                    style.bg(Theme::blue_active(self.theme_mode)).opacity(0.9)
                                })
                                .on_click(cx.listener(move |this, _event, cx| {
                                    this.undo_record(record_id_clone.clone(), cx);
                                }))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(Theme::crust(self.theme_mode))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Undo All"),
                                ),
                        )
                    }),
            )
            // Items (if expanded)
            .when(expanded, |d| {
                d.child(
                    div()
                        .w_full()
                        .flex()
                        .flex_col()
                        .bg(Theme::base(self.theme_mode))
                        .children(
                            items
                                .iter()
                                .map(|item| self.render_quarantine_item(item.clone(), cx)),
                        ),
                )
            })
    }

    fn render_quarantine_item(&self, item: QuarantineItemData, cx: &mut ViewContext<Self>) -> Div {
        let success = item.success;
        let has_error = !item.error_message.is_empty();
        let deleted_permanently = item.deleted_permanently;
        let can_delete = !deleted_permanently && item.quarantine_path.is_some();
        let record_id = item.record_id.clone();
        let item_index = item.item_index;

        div()
            .w_full()
            .px_4()
            .pl_12()
            .py_2()
            .flex()
            .items_center()
            .gap_3()
            .hover(|style| style.bg(Theme::surface0(self.theme_mode)))
            .border_b_1()
            .border_color(Theme::border_subtle(self.theme_mode))
            .child(
                div()
                    .text_sm()
                    .child(if success { "✓" } else { "✗" })
                    .text_color(if success {
                        Theme::green(self.theme_mode)
                    } else {
                        Theme::red(self.theme_mode)
                    }),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::text(self.theme_mode))
                            .child(item.item_type.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::overlay0(self.theme_mode))
                            .child(item.original_path.clone()),
                    )
                    .when(has_error, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(Theme::red(self.theme_mode))
                                .child(item.error_message.clone()),
                        )
                    }),
            )
            .when(deleted_permanently, |d| {
                d.child(
                    div()
                        .px_2()
                        .py_1()
                        .bg(Theme::surface1(self.theme_mode))
                        .rounded_sm()
                        .child(
                            div()
                                .text_xs()
                                .text_color(Theme::subtext0(self.theme_mode))
                                .child("Permanent"),
                        ),
                )
            })
            .child(
                div()
                    .text_sm()
                    .text_color(Theme::subtext1(self.theme_mode))
                    .child(item.size_str.clone()),
            )
            .when(can_delete, |d| {
                d.child(
                    div()
                        .id(SharedString::from(format!(
                            "delete-item-{}-{}",
                            record_id, item_index
                        )))
                        .px_3()
                        .py_1()
                        .bg(Theme::red(self.theme_mode))
                        .rounded_md()
                        .cursor_pointer()
                        .hover(|style| style.bg(Theme::red_hover(self.theme_mode)))
                        .active(|style| style.bg(Theme::red_active(self.theme_mode)).opacity(0.9))
                        .on_click(cx.listener(move |this, _event, cx| {
                            this.delete_quarantine_item(record_id.to_string(), item_index, cx);
                        }))
                        .child(
                            div()
                                .text_xs()
                                .text_color(Theme::crust(self.theme_mode))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("Delete"),
                        ),
                )
            })
    }

    fn render_settings_tab(&mut self, cx: &mut ViewContext<Self>) -> Div {
        let cache_ttls = self.cache_ttls.clone();

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .bg(Theme::base(self.theme_mode))
            // Header
            .child(
                div()
                    .w_full()
                    .p_4()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(Theme::surface0(self.theme_mode))
                    .child(div().text_xl().font_weight(FontWeight::BOLD).text_color(Theme::text(self.theme_mode)).child("Settings"))
                    .child(
                        div()
                            .id("reset-defaults-btn")
                            .px_4()
                            .py_2()
                            .bg(Theme::surface0(self.theme_mode))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
                            .active(|style| style.bg(Theme::surface2(self.theme_mode)).opacity(0.9))
                            .on_click(cx.listener(|this, _event, cx| {
                                this.reset_cache_defaults(cx);
                                cx.notify();
                            }))
                            .child(div().text_sm().text_color(Theme::text(self.theme_mode)).child("Reset to Defaults"))
                    )
            )
            // Content
            .child(
                div()
                    .id("settings-content")
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .p_4()
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_col()
                            .gap_6()
                            // Cache TTL Section
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .child(div().text_lg().font_weight(FontWeight::SEMIBOLD).text_color(Theme::text(self.theme_mode)).child("Cache TTL Settings"))
                                            .child(div().text_sm().text_color(Theme::subtext0(self.theme_mode)).child("Configure how long scan results are cached for each category."))
                                    )
                                    // TTL list
                                    .child(
                                        div()
                                            .w_full()
                                            .bg(Theme::surface0(self.theme_mode))
                                            .rounded_lg()
                                            .border_1()
                                            .border_color(Theme::surface1(self.theme_mode))
                                            .flex()
                                            .flex_col()
                                            .children(cache_ttls.iter().map(|ttl| {
                                                self.render_ttl_setting(ttl.clone())
                                            }))
                                    )
                            )
                            // Info section
                            .child(
                                div()
                                    .w_full()
                                    .p_4()
                                    .bg(Theme::blue_tint(self.theme_mode))
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(Theme::blue_border(self.theme_mode))
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(div().text_base().child("💡"))
                                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(Theme::blue(self.theme_mode)).child("Cache TTL Explained"))
                                    )
                                    .child(div().text_sm().text_color(Theme::subtext1(self.theme_mode)).child("TTL determines how long cached results remain valid. A value of 0 means always rescan."))
                            )
                    )
            )
    }

    fn render_ttl_setting(&self, setting: CacheTTLSetting) -> Div {
        let ttl_display = if setting.ttl_minutes == 0 {
            "Never".to_string()
        } else if setting.ttl_minutes < 60 {
            format!("{} min", setting.ttl_minutes)
        } else {
            format!("{} hr", setting.ttl_minutes / 60)
        };

        div()
            .w_full()
            .px_4()
            .py_3()
            .flex()
            .items_center()
            .justify_between()
            .border_b_1()
            .border_color(Theme::surface1(self.theme_mode))
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(Theme::text(self.theme_mode))
                    .child(setting.category.clone()),
            )
            .child(
                div()
                    .w(px(100.0))
                    .px_3()
                    .py_1()
                    .bg(Theme::base(self.theme_mode))
                    .rounded_sm()
                    .border_1()
                    .border_color(Theme::surface1(self.theme_mode))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::text(self.theme_mode))
                            .child(ttl_display),
                    ),
            )
    }

    fn render_about_tab(&self) -> Div {
        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(Theme::base(self.theme_mode))
            .gap_6()
            // Logo and title
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_4()
                    .child(
                        svg()
                            .path(self.theme_mode.icon_path())
                            .size(px(80.0))
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(Theme::text(self.theme_mode))
                            .child("DevSweep")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::subtext0(self.theme_mode))
                            .child("Version 0.1.0")
                    )
            )
            // Description
            .child(
                div()
                    .max_w(px(500.0))
                    .flex()
                    .justify_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::subtext1(self.theme_mode))
                            .child("A powerful tool for cleaning up development-related caches, temporary files, and unused data on your Mac.")
                    )
            )
            // Features
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .p_4()
                    .bg(Theme::surface0(self.theme_mode))
                    .rounded_lg()
                    .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(Theme::text(self.theme_mode)).child("Features"))
                    .child(self.feature_item("🔍", "Scan 16+ categories of development caches"))
                    .child(self.feature_item("🛡️", "Safe deletion with quarantine support"))
                    .child(self.feature_item("↩️", "Undo cleanup operations"))
                    .child(self.feature_item("⚡", "Incremental scanning with smart caching"))
                    .child(self.feature_item("⚙️", "Customizable cache TTL settings"))
            )
            // Built with
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(div().text_xs().text_color(Theme::overlay0(self.theme_mode)).child("Built with"))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(self.tech_badge("🦀", "Rust"))
                            .child(self.tech_badge("⚡", "GPUI"))
                    )
            )
    }

    fn feature_item(&self, icon: &str, text: &str) -> Div {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(div().text_sm().child(icon.to_string()))
            .child(
                div()
                    .text_sm()
                    .text_color(Theme::subtext1(self.theme_mode))
                    .child(text.to_string()),
            )
    }

    fn tech_badge(&self, icon: &str, name: &str) -> Div {
        div()
            .flex()
            .items_center()
            .gap_1()
            .px_3()
            .py_1()
            .bg(Theme::surface0(self.theme_mode))
            .rounded_md()
            .child(div().text_sm().child(icon.to_string()))
            .child(
                div()
                    .text_sm()
                    .text_color(Theme::text(self.theme_mode))
                    .child(name.to_string()),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(1000.0), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("DevSweep".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| DevCleaner::new()),
        )
        .unwrap();
    });
}
