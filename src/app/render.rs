use crate::app::state::{
    CacheTTLSetting, CategoryItem, CleanupItemData, DevSweep, QuarantineItemData,
    QuarantineRecordData,
};
use crate::ui::sidebar::Tab;
use crate::ui::Theme;
use gpui::prelude::FluentBuilder;
use gpui::*;

impl Render for DevSweep {
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

impl DevSweep {
    pub fn render_sidebar(&mut self, cx: &mut ViewContext<Self>) -> Div {
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

    pub fn sidebar_item(
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

    pub fn render_scan_tab(&mut self, cx: &mut ViewContext<Self>) -> Div {
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

    pub fn empty_state(&self, message: &str) -> Div {
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

    pub fn render_category_section(
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

    pub fn render_cleanup_item(
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

    pub fn render_quarantine_tab(&mut self, cx: &mut ViewContext<Self>) -> Div {
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
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(Theme::text(self.theme_mode))
                                    .child("Quarantine"),
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
                            .child(
                                div()
                                    .id("refresh-quarantine-btn")
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
                                        this.refresh_quarantine();
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::text(self.theme_mode))
                                            .child("Refresh"),
                                    ),
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
                                    .active(|style| {
                                        style.bg(Theme::surface2(self.theme_mode)).opacity(0.9)
                                    })
                                    .on_click(cx.listener(|_this, _event, _cx| {
                                        if let Some(cache_dir) = dirs::cache_dir() {
                                            let quarantine_path = cache_dir
                                                .join("development-cleaner")
                                                .join("quarantine");
                                            let _ = std::fs::create_dir_all(&quarantine_path);
                                            let _ = std::process::Command::new("open")
                                                .arg(quarantine_path)
                                                .spawn();
                                        }
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::text(self.theme_mode))
                                            .child("Open in Finder"),
                                    ),
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
                                        .active(|style| {
                                            style
                                                .bg(Theme::red_active(self.theme_mode))
                                                .opacity(0.9)
                                        })
                                        .on_click(cx.listener(|this, _event, cx| {
                                            this.clear_all_quarantine(cx);
                                        }))
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(Theme::crust(self.theme_mode))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Delete All"),
                                        ),
                                )
                            })
                            .when(records_empty || is_cleaning, |d| {
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
                                                .child("Delete All"),
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
                                    .child("Quarantine Size:"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(Theme::peach(self.theme_mode))
                                    .child(total_size),
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
                                    .child("Total Items:"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(Theme::blue(self.theme_mode))
                                    .child(format!("{}", total_items)),
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
                                    .child("Records:"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(Theme::lavender(self.theme_mode))
                                    .child(format!("{}", records.len())),
                            ),
                    ),
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
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::blue(self.theme_mode))
                            .child("Deleted files are quarantined for undo support. They are automatically cleaned when exceeding 10GB."),
                    ),
            )
            // Records list
            .child(
                div()
                    .id("quarantine-content")
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .child(if records_empty {
                        self.empty_state("No cleanup history yet")
                    } else {
                        div().w_full().flex().flex_col().children(
                            records.iter().enumerate().map(|(record_idx, record)| {
                                let record_items: Vec<_> = items
                                    .iter()
                                    .filter(|item| item.record_id == record.id)
                                    .cloned()
                                    .collect();

                                self.render_quarantine_record(
                                    record.clone(),
                                    record_items,
                                    record_idx,
                                    cx,
                                )
                            }),
                        )
                    }),
            )
    }

    pub fn render_quarantine_record(
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

    pub fn render_quarantine_item(
        &self,
        item: QuarantineItemData,
        cx: &mut ViewContext<Self>,
    ) -> Div {
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

    pub fn render_settings_tab(&mut self, cx: &mut ViewContext<Self>) -> Div {
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
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(Theme::text(self.theme_mode))
                            .child("Settings"),
                    )
                    .child(
                        div()
                            .id("reset-defaults-btn")
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
                                this.reset_cache_defaults(cx);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::text(self.theme_mode))
                                    .child("Reset to Defaults"),
                            ),
                    ),
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
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(Theme::text(self.theme_mode))
                                                    .child("Cache TTL Settings"),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(Theme::subtext0(self.theme_mode))
                                                    .child("Configure how long scan results are cached for each category."),
                                            ),
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
                                            .children(
                                                cache_ttls
                                                    .iter()
                                                    .map(|ttl| self.render_ttl_setting(ttl.clone())),
                                            ),
                                    ),
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
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(Theme::blue(self.theme_mode))
                                                    .child("Cache TTL Explained"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::subtext1(self.theme_mode))
                                            .child("TTL determines how long cached results remain valid. A value of 0 means always rescan."),
                                    ),
                            ),
                    ),
            )
    }

    pub fn render_ttl_setting(&self, setting: CacheTTLSetting) -> Div {
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

    pub fn render_about_tab(&self) -> Div {
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
                    .child(svg().path(self.theme_mode.icon_path()).size(px(80.0)))
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(Theme::text(self.theme_mode))
                            .child("DevSweep"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::subtext0(self.theme_mode))
                            .child("Version 0.1.0"),
                    ),
            )
            // Description
            .child(
                div().max_w(px(500.0)).flex().justify_center().child(
                    div()
                        .text_sm()
                        .text_color(Theme::subtext1(self.theme_mode))
                        .child("A powerful tool for cleaning up development-related caches, temporary files, and unused data on your Mac."),
                ),
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
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(Theme::text(self.theme_mode))
                            .child("Features"),
                    )
                    .child(self.feature_item("🔍", "Scan 16+ categories of development caches"))
                    .child(self.feature_item("🛡️", "Safe deletion with quarantine support"))
                    .child(self.feature_item("↩️", "Undo cleanup operations"))
                    .child(self.feature_item("⚡", "Incremental scanning with smart caching"))
                    .child(self.feature_item("⚙️", "Customizable cache TTL settings")),
            )
            // Built with
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::overlay0(self.theme_mode))
                            .child("Built with"),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(self.tech_badge("🦀", "Rust"))
                            .child(self.tech_badge("⚡", "GPUI")),
                    ),
            )
    }

    pub fn feature_item(&self, icon: &str, text: &str) -> Div {
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

    pub fn tech_badge(&self, icon: &str, name: &str) -> Div {
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
