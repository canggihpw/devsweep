use crate::app::state::{CategoryItem, CleanupItemData, DevSweep};
use crate::ui::Theme;
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
}
