use crate::app::state::{CategoryItem, CleanupItemData, DevSweep, SizeFilter, SuperCategoryItem};
use crate::ui::Theme;
use crate::utils;
use gpui::prelude::FluentBuilder;
use gpui::*;

impl DevSweep {
    pub fn render_scan_tab(&mut self, cx: &mut ViewContext<Self>) -> Div {
        let is_scanning = self.is_scanning;
        let is_cleaning = self.is_cleaning;
        let status_text = self.status_text.clone();
        let total_reclaimable = self.total_reclaimable.clone();
        let selected_count = self.selected_items_count;
        let selected_size = self.selected_items_size.clone();
        let categories = self.categories.clone();
        let items = self.all_items.clone();
        let size_filter = self.size_filter;
        let size_filter_dropdown_open = self.size_filter_dropdown_open;
        let scan_dropdown_open = self.scan_dropdown_open;

        // Use filtered super categories when a filter is active
        let super_categories = if size_filter == SizeFilter::All {
            self.super_categories.clone()
        } else {
            self.filtered_super_categories.clone()
        };

        // Calculate filtered total if filter is active
        let filtered_total: SharedString = if size_filter == SizeFilter::All {
            total_reclaimable.clone()
        } else {
            let total: u64 = self.filtered_items.iter().map(|i| i.size).sum();
            utils::format_size(total).into()
        };

        div()
            .relative()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .bg(Theme::base(self.theme_mode))
            // Unified header toolbar (merged title and controls)
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_3()
                    .flex()
                    .items_center()
                    .justify_between()
                    // Left: Title and status
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(Theme::text(self.theme_mode))
                                    .child("Scan & Clean"),
                            )
                            .when(!status_text.is_empty() && status_text != "Ready", |d| {
                                d.child(
                                    div()
                                        .text_sm()
                                        .text_color(Theme::subtext0(self.theme_mode))
                                        .child(status_text),
                                )
                            }),
                    )
                    // Right: Split Scan button
                    .child(
                        div()
                            .flex()
                            .items_center()
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
                                d.child(self.render_split_scan_button(scan_dropdown_open, cx))
                            }),
                    ),
            )
            // Divider line
            .child(
                div()
                    .w_full()
                    .h(px(1.0))
                    .bg(Theme::surface0(self.theme_mode)),
            )
            // Control bar (filter left, stats center, actions right)
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_3()
                    .flex()
                    .items_center()
                    .justify_between()
                    // Left: Filter dropdown
                    .child(self.render_size_filter_button(
                        size_filter,
                        size_filter_dropdown_open,
                        cx,
                    ))
                    // Center: Stats as pills
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            // Reclaimable pill
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .bg(Theme::surface0(self.theme_mode))
                                    .rounded_full()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(Theme::subtext0(self.theme_mode))
                                            .child(if size_filter == SizeFilter::All {
                                                "Reclaimable"
                                            } else {
                                                "Filtered"
                                            }),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(Theme::peach(self.theme_mode))
                                            .child(filtered_total),
                                    ),
                            )
                            // Selected pill (only show if something selected)
                            .when(selected_count > 0, |d| {
                                d.child(
                                    div()
                                        .px_3()
                                        .py_1()
                                        .bg(Theme::blue_tint(self.theme_mode))
                                        .rounded_full()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(Theme::subtext0(self.theme_mode))
                                                .child("Selected"),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(Theme::blue(self.theme_mode))
                                                .child(format!(
                                                    "{} ({})",
                                                    selected_count, selected_size
                                                )),
                                        ),
                                )
                            }),
                    )
                    // Right: Action buttons (ghost style for select/deselect, solid for clean)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            // Ghost button: Select All
                            .child(
                                div()
                                    .id("select-all-btn")
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(Theme::surface0(self.theme_mode)))
                                    .active(|style| {
                                        style.bg(Theme::surface1(self.theme_mode)).opacity(0.9)
                                    })
                                    .on_click(cx.listener(|this, _event, cx| {
                                        this.select_all(cx);
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::subtext0(self.theme_mode))
                                            .child("Select All"),
                                    ),
                            )
                            // Ghost button: Deselect
                            .child(
                                div()
                                    .id("deselect-all-btn")
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(Theme::surface0(self.theme_mode)))
                                    .active(|style| {
                                        style.bg(Theme::surface1(self.theme_mode)).opacity(0.9)
                                    })
                                    .on_click(cx.listener(|this, _event, cx| {
                                        this.deselect_all(cx);
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::subtext0(self.theme_mode))
                                            .child("Deselect"),
                                    ),
                            )
                            // Clean button (solid red when active, disabled otherwise)
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
                                                .child("Clean"),
                                        ),
                                )
                            })
                            // Disabled clean button
                            .when(selected_count == 0 || is_cleaning, |d| {
                                d.child(
                                    div()
                                        .px_4()
                                        .py_2()
                                        .bg(Theme::surface0(self.theme_mode))
                                        .rounded_md()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(Theme::overlay0(self.theme_mode))
                                                .child("Clean"),
                                        ),
                                )
                            }),
                    ),
            )
            // Divider line before content
            .child(
                div()
                    .w_full()
                    .h(px(1.0))
                    .bg(Theme::surface0(self.theme_mode)),
            )
            // Super categories list
            .child(
                div()
                    .id("scan-content")
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .child(if super_categories.is_empty() {
                        self.render_empty_state()
                    } else {
                        div().w_full().flex().flex_col().children(
                            super_categories
                                .iter()
                                .enumerate()
                                .map(|(super_idx, super_cat)| {
                                    self.render_super_category_section(
                                        super_cat.clone(),
                                        super_idx,
                                        &categories,
                                        &items,
                                        cx,
                                    )
                                }),
                        )
                    }),
            )
            // Dropdown overlays (rendered last to appear on top)
            .when(size_filter_dropdown_open, |d| {
                d.child(self.render_size_filter_overlay(size_filter, cx))
            })
            .when(scan_dropdown_open, |d| {
                d.child(self.render_scan_dropdown_overlay(cx))
            })
    }

    /// Render the scan button with inline dropdown menu
    fn render_split_scan_button(&self, is_open: bool, cx: &mut ViewContext<Self>) -> Div {
        div()
            .relative()
            .flex()
            .flex_col()
            // Button row
            .child(
                div()
                    .flex()
                    .items_center()
                    // Main scan button
                    .child(
                        div()
                            .id("scan-btn")
                            .px_4()
                            .py_2()
                            .bg(Theme::blue(self.theme_mode))
                            .rounded_l_md()
                            .cursor_pointer()
                            .hover(|style| style.bg(Theme::blue_hover(self.theme_mode)))
                            .active(|style| {
                                style.bg(Theme::blue_active(self.theme_mode)).opacity(0.9)
                            })
                            .on_click(cx.listener(|this, _event, cx| {
                                if !this.is_scanning && !this.is_cleaning {
                                    this.scan_dropdown_open = false;
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
                    // Dropdown trigger (chevron)
                    .child(
                        div()
                            .id("scan-dropdown-btn")
                            .px_2()
                            .py_2()
                            .bg(if is_open {
                                Theme::blue_active(self.theme_mode)
                            } else {
                                Theme::blue(self.theme_mode)
                            })
                            .rounded_r_md()
                            .border_l_1()
                            .border_color(rgba(0xffffff30))
                            .cursor_pointer()
                            .hover(|style| style.bg(Theme::blue_hover(self.theme_mode)))
                            .active(|style| {
                                style.bg(Theme::blue_active(self.theme_mode)).opacity(0.9)
                            })
                            .on_click(cx.listener(|this, _event, cx| {
                                this.toggle_scan_dropdown(cx);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(Theme::crust(self.theme_mode))
                                    .child(if is_open { "▲" } else { "▼" }),
                            ),
                    ),
            )
    }

    /// Render the scan dropdown overlay - positioned relative to button
    fn render_scan_dropdown_overlay(&self, cx: &mut ViewContext<Self>) -> Div {
        div()
            .absolute()
            .top(px(48.0))
            .right(px(16.0))
            .min_w(px(160.0))
            .bg(Theme::base(self.theme_mode))
            .border_1()
            .border_color(Theme::surface1(self.theme_mode))
            .rounded_md()
            .shadow_lg()
            .flex()
            .flex_col()
            .overflow_hidden()
            // Full Rescan option
            .child(
                div()
                    .id("full-rescan-option")
                    .w_full()
                    .px_4()
                    .py_3()
                    .cursor_pointer()
                    .bg(Theme::base(self.theme_mode))
                    .hover(|style| style.bg(Theme::surface0(self.theme_mode)))
                    .active(|style| style.bg(Theme::surface1(self.theme_mode)).opacity(0.9))
                    .on_click(cx.listener(|this, _event, cx| {
                        this.scan_dropdown_open = false;
                        if !this.is_scanning && !this.is_cleaning {
                            this.start_scan(false, cx);
                        }
                        cx.notify();
                    }))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(Theme::text(self.theme_mode))
                            .child("Full Rescan"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::subtext0(self.theme_mode))
                            .child("Bypass cache, scan everything"),
                    ),
            )
    }

    /// Render elegant empty state - centered with larger icon
    pub fn render_empty_state(&self) -> Div {
        div()
            .w_full()
            .flex_1()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_6()
            .pb_16() // Add bottom padding to visually center better
            // Large icon
            .child(
                div()
                    .w(px(80.0))
                    .h(px(80.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_2xl()
                    .bg(Theme::surface0(self.theme_mode))
                    .child(
                        div()
                            .text_color(Theme::subtext0(self.theme_mode))
                            .child("🧹")
                            .text_size(px(40.0)),
                    ),
            )
            // Text hierarchy
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(Theme::text(self.theme_mode))
                            .child("Ready to sweep?"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::subtext0(self.theme_mode))
                            .child("Click Scan to analyze your projects."),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::overlay0(self.theme_mode))
                            .child("Your cleanup results will appear here."),
                    ),
            )
    }

    // Keep the old method name for compatibility but redirect to new one
    pub fn empty_state(&self, _message: &str) -> Div {
        self.render_empty_state()
    }

    /// Render a super category section with its child categories
    pub fn render_super_category_section(
        &self,
        super_cat: SuperCategoryItem,
        super_idx: usize,
        all_categories: &[CategoryItem],
        all_items: &[CleanupItemData],
        cx: &mut ViewContext<Self>,
    ) -> Div {
        let expanded = super_cat.expanded;

        div()
            .w_full()
            .flex()
            .flex_col()
            .border_b_1()
            .border_color(Theme::surface0(self.theme_mode))
            // Super category header
            .child(
                div()
                    .id(SharedString::from(format!(
                        "super-cat-header-{}",
                        super_idx
                    )))
                    .w_full()
                    .px_4()
                    .py_3()
                    .flex()
                    .items_center()
                    .gap_3()
                    .bg(Theme::base(self.theme_mode))
                    .hover(|style| style.bg(Theme::surface0(self.theme_mode)))
                    .active(|style| style.bg(Theme::surface1(self.theme_mode)).opacity(0.9))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _event, cx| {
                        this.toggle_super_category_expand(super_idx, cx);
                        cx.notify();
                    }))
                    // Expand arrow
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::subtext0(self.theme_mode))
                            .child(if expanded { "▼" } else { "▶" }),
                    )
                    // Checkbox
                    .child(
                        div()
                            .id(SharedString::from(format!("super-cat-check-{}", super_idx)))
                            .w_4()
                            .h_4()
                            .rounded_sm()
                            .border_1()
                            .border_color(if super_cat.checked {
                                Theme::blue(self.theme_mode)
                            } else {
                                Theme::surface2(self.theme_mode)
                            })
                            .bg(if super_cat.checked {
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
                                this.toggle_super_category(super_idx, cx);
                                cx.notify();
                            }))
                            .when(super_cat.checked, |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(Theme::crust(self.theme_mode))
                                        .child("✓"),
                                )
                            }),
                    )
                    // Icon
                    .child(div().text_base().child(super_cat.icon.clone()))
                    // Name
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(Theme::text(self.theme_mode))
                            .child(super_cat.name.clone()),
                    )
                    // Category count badge
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .bg(Theme::surface0(self.theme_mode))
                            .rounded_full()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(Theme::subtext0(self.theme_mode))
                                    .child(format!(
                                        "{} categories, {} items",
                                        super_cat.category_count, super_cat.item_count
                                    )),
                            ),
                    )
                    // Total size
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(Theme::peach(self.theme_mode))
                            .child(super_cat.size_str.clone()),
                    ),
            )
            // Child categories (if expanded)
            .when(expanded, |d| {
                d.child(
                    div()
                        .w_full()
                        .flex()
                        .flex_col()
                        .bg(Theme::base(self.theme_mode))
                        .children(super_cat.category_indices.iter().map(|&cat_idx| {
                            let category = all_categories[cat_idx].clone();
                            let cat_items: Vec<_> = all_items
                                .iter()
                                .filter(|item| item.category_index == cat_idx)
                                .cloned()
                                .collect();
                            self.render_category_section(category, cat_items, cat_idx, cx)
                        })),
                )
            })
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
            .border_color(Theme::border_subtle(self.theme_mode))
            // Category header (indented since it's inside super category)
            .child(
                div()
                    .id(SharedString::from(format!("cat-header-{}", cat_idx)))
                    .w_full()
                    .pl_8() // Indent to show hierarchy
                    .pr_4()
                    .py_2()
                    .flex()
                    .items_center()
                    .gap_3()
                    .bg(Theme::base(self.theme_mode))
                    .hover(|style| style.bg(Theme::surface0(self.theme_mode)))
                    .active(|style| style.bg(Theme::surface1(self.theme_mode)).opacity(0.9))
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
                            .bg(Theme::surface0(self.theme_mode))
                            .rounded_full()
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
                        .children(items.iter().map(|item| {
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
            .pl_16() // Further indent for items (inside category, inside super category)
            .pr_4()
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
                        .rounded_full()
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
                        .rounded_full()
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

    /// Render the size filter button (with light border and rounded corners)
    fn render_size_filter_button(
        &self,
        current_filter: SizeFilter,
        is_open: bool,
        cx: &mut ViewContext<Self>,
    ) -> Stateful<Div> {
        div()
            .id("size-filter-btn")
            .px_3()
            .py_2()
            .bg(Theme::base(self.theme_mode))
            .rounded_md()
            .cursor_pointer()
            .border_1()
            .border_color(if is_open {
                Theme::blue(self.theme_mode)
            } else {
                Theme::surface1(self.theme_mode)
            })
            .hover(|style| style.border_color(Theme::surface2(self.theme_mode)))
            .active(|style| style.bg(Theme::surface0(self.theme_mode)).opacity(0.9))
            .on_click(cx.listener(|this, _event, cx| {
                this.toggle_size_filter_dropdown(cx);
                cx.notify();
            }))
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .text_sm()
                    .text_color(Theme::text(self.theme_mode))
                    .child(current_filter.label()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(Theme::subtext0(self.theme_mode))
                    .child(if is_open { "▲" } else { "▼" }),
            )
    }

    /// Render the size filter dropdown overlay (rendered at root level to appear on top)
    fn render_size_filter_overlay(
        &self,
        current_filter: SizeFilter,
        cx: &mut ViewContext<Self>,
    ) -> Div {
        div()
            .absolute()
            .top(px(110.0)) // Position below unified header
            .left(px(16.0))
            .min_w(px(140.0))
            .bg(Theme::base(self.theme_mode))
            .border_1()
            .border_color(Theme::surface1(self.theme_mode))
            .rounded_md()
            .shadow_lg()
            .flex()
            .flex_col()
            .children(SizeFilter::all_options().into_iter().map(|filter| {
                let is_selected = filter == current_filter;
                div()
                    .id(SharedString::from(format!(
                        "filter-option-{}",
                        filter.label()
                    )))
                    .px_3()
                    .py_2()
                    .cursor_pointer()
                    .bg(if is_selected {
                        Theme::surface0(self.theme_mode)
                    } else {
                        Theme::transparent()
                    })
                    .hover(|style| style.bg(Theme::surface0(self.theme_mode)))
                    .active(|style| style.bg(Theme::surface1(self.theme_mode)).opacity(0.9))
                    .on_click(cx.listener(move |this, _event, cx| {
                        this.set_size_filter(filter, cx);
                        cx.notify();
                    }))
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(if is_selected {
                                Theme::blue(self.theme_mode)
                            } else {
                                Theme::text(self.theme_mode)
                            })
                            .font_weight(if is_selected {
                                FontWeight::SEMIBOLD
                            } else {
                                FontWeight::NORMAL
                            })
                            .child(filter.label()),
                    )
                    .when(is_selected, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(Theme::blue(self.theme_mode))
                                .child("✓"),
                        )
                    })
            }))
    }
}
