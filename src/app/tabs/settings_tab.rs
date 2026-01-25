use crate::app::state::{CacheTTLSetting, DevSweep, SuperCategoryType};
use crate::custom_paths::CustomPath;
use crate::ui::Theme;
use gpui::*;

impl DevSweep {
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
                            .child(self.render_cache_ttl_section(&cache_ttls, cx))
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
                            )
                            // Custom Paths Section
                            .child(self.render_custom_paths_section(cx)),
                    ),
            )
    }

    /// Render the entire Cache TTL section grouped by super category
    fn render_cache_ttl_section(
        &self,
        cache_ttls: &[CacheTTLSetting],
        cx: &mut ViewContext<Self>,
    ) -> Div {
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_4()
            // Header
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
                            .child(
                                "Configure how long scan results are cached. Use +/- to adjust.",
                            ),
                    ),
            )
            // Grouped TTL settings
            .child(
                div().w_full().flex().flex_col().gap_3().children(
                    SuperCategoryType::all()
                        .into_iter()
                        .filter_map(|super_type| {
                            // Get TTL settings for this super category
                            let ttls_in_group: Vec<_> = cache_ttls
                                .iter()
                                .filter(|ttl| {
                                    SuperCategoryType::from_category_name(ttl.category.as_ref())
                                        == super_type
                                })
                                .cloned()
                                .collect();

                            if ttls_in_group.is_empty() {
                                None
                            } else {
                                Some(self.render_ttl_group(super_type, ttls_in_group, cx))
                            }
                        }),
                ),
            )
    }

    /// Render a group of TTL settings under a super category header
    fn render_ttl_group(
        &self,
        super_type: SuperCategoryType,
        ttls: Vec<CacheTTLSetting>,
        cx: &mut ViewContext<Self>,
    ) -> Div {
        div()
            .w_full()
            .flex()
            .flex_col()
            // Group header
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_2()
                    .bg(Theme::mantle(self.theme_mode))
                    .rounded_t_lg()
                    .border_1()
                    .border_color(Theme::surface1(self.theme_mode))
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().text_base().child(super_type.icon()))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(Theme::text(self.theme_mode))
                            .child(super_type.name()),
                    ),
            )
            // TTL items
            .child(
                div()
                    .w_full()
                    .bg(Theme::surface0(self.theme_mode))
                    .rounded_b_lg()
                    .border_l_1()
                    .border_r_1()
                    .border_b_1()
                    .border_color(Theme::surface1(self.theme_mode))
                    .flex()
                    .flex_col()
                    .children(
                        ttls.iter()
                            .map(|ttl| self.render_ttl_setting(ttl.clone(), cx)),
                    ),
            )
    }

    pub fn render_ttl_setting(&self, setting: CacheTTLSetting, cx: &mut ViewContext<Self>) -> Div {
        let ttl_display = if setting.ttl_minutes == 0 {
            "Never".to_string()
        } else if setting.ttl_minutes < 60 {
            format!("{} min", setting.ttl_minutes)
        } else if setting.ttl_minutes < 1440 {
            format!("{} hr", setting.ttl_minutes / 60)
        } else {
            format!("{} day", setting.ttl_minutes / 1440)
        };

        let category = setting.category.clone();
        let category_for_decrease = category.to_string();
        let category_for_increase = category.to_string();
        let is_at_min = setting.ttl_minutes == 0;
        let is_at_max = setting.ttl_minutes >= 1440; // 24 hours max

        div()
            .w_full()
            .px_4()
            .py_2()
            .flex()
            .items_center()
            .justify_between()
            .border_b_1()
            .border_color(Theme::border_subtle(self.theme_mode))
            // Category name
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(Theme::text(self.theme_mode))
                    .child(category),
            )
            // TTL controls
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    // Decrease button
                    .child(if is_at_min {
                        div()
                            .w_6()
                            .h_6()
                            .rounded_sm()
                            .bg(Theme::surface1(self.theme_mode))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::overlay0(self.theme_mode))
                                    .child("−"),
                            )
                            .into_any_element()
                    } else {
                        div()
                            .id(SharedString::from(format!(
                                "ttl-dec-{}",
                                category_for_decrease
                            )))
                            .w_6()
                            .h_6()
                            .rounded_sm()
                            .bg(Theme::surface1(self.theme_mode))
                            .cursor_pointer()
                            .hover(|style| style.bg(Theme::surface2(self.theme_mode)))
                            .active(|style| style.opacity(0.8))
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_click(cx.listener(move |this, _event, cx| {
                                this.decrease_ttl(&category_for_decrease, cx);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::text(self.theme_mode))
                                    .child("−"),
                            )
                            .into_any_element()
                    })
                    // TTL value display
                    .child(
                        div()
                            .w(px(70.0))
                            .px_2()
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
                    // Increase button
                    .child(if is_at_max {
                        div()
                            .w_6()
                            .h_6()
                            .rounded_sm()
                            .bg(Theme::surface1(self.theme_mode))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::overlay0(self.theme_mode))
                                    .child("+"),
                            )
                            .into_any_element()
                    } else {
                        div()
                            .id(SharedString::from(format!(
                                "ttl-inc-{}",
                                category_for_increase
                            )))
                            .w_6()
                            .h_6()
                            .rounded_sm()
                            .bg(Theme::surface1(self.theme_mode))
                            .cursor_pointer()
                            .hover(|style| style.bg(Theme::surface2(self.theme_mode)))
                            .active(|style| style.opacity(0.8))
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_click(cx.listener(move |this, _event, cx| {
                                this.increase_ttl(&category_for_increase, cx);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::text(self.theme_mode))
                                    .child("+"),
                            )
                            .into_any_element()
                    }),
            )
    }

    /// Render the custom paths section
    fn render_custom_paths_section(&self, cx: &mut ViewContext<Self>) -> Div {
        let custom_paths = self.custom_paths.clone();

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_4()
            .mt_6()
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
                            .child("Custom Scan Paths"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::subtext0(self.theme_mode))
                            .child("Add your own directories to include in scans."),
                    ),
            )
            // Add path form - two rows for better UX
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap_2()
                    // Row 1: Buttons
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .id("browse-path-btn")
                                    .px_3()
                                    .py_2()
                                    .bg(Theme::surface0(self.theme_mode))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
                                    .on_click(cx.listener(|this, _event, cx| {
                                        this.browse_for_custom_path(cx);
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::text(self.theme_mode))
                                            .child("Browse Folder..."),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::subtext0(self.theme_mode))
                                    .child("or"),
                            )
                            .child(
                                div()
                                    .id("type-path-btn")
                                    .px_3()
                                    .py_2()
                                    .bg(Theme::surface0(self.theme_mode))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
                                    .on_click(cx.listener(|this, _event, cx| {
                                        this.type_custom_path(cx);
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::text(self.theme_mode))
                                            .child("Enter Path..."),
                                    ),
                            ),
                    )
                    // Row 2: Selected path display and Add button
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .flex_1()
                                    .px_3()
                                    .py_2()
                                    .bg(Theme::surface0(self.theme_mode))
                                    .rounded_md()
                                    .border_1()
                                    .border_color(Theme::surface1(self.theme_mode))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(if self.new_custom_path_input.is_empty() {
                                                Theme::subtext0(self.theme_mode)
                                            } else {
                                                Theme::text(self.theme_mode)
                                            })
                                            .child(if self.new_custom_path_input.is_empty() {
                                                "No path selected".to_string()
                                            } else {
                                                self.new_custom_path_input.clone()
                                            }),
                                    ),
                            )
                            .child(if !self.new_custom_path_input.is_empty() {
                                div()
                                    .id("clear-path-btn")
                                    .px_2()
                                    .py_2()
                                    .bg(Theme::surface1(self.theme_mode))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.opacity(0.8))
                                    .on_click(cx.listener(|this, _event, cx| {
                                        this.new_custom_path_input.clear();
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::text(self.theme_mode))
                                            .child("Clear"),
                                    )
                                    .into_any_element()
                            } else {
                                div().into_any_element()
                            })
                            .child(if self.new_custom_path_input.is_empty() {
                                // Disabled state - no click handler needed
                                div()
                                    .px_3()
                                    .py_2()
                                    .bg(Theme::surface1(self.theme_mode))
                                    .rounded_md()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::subtext0(self.theme_mode))
                                            .child("Add Path"),
                                    )
                                    .into_any_element()
                            } else {
                                // Enabled state - with click handler
                                div()
                                    .id("add-path-btn")
                                    .px_3()
                                    .py_2()
                                    .bg(Theme::green(self.theme_mode))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.opacity(0.8))
                                    .on_click(cx.listener(|this, _event, cx| {
                                        this.add_custom_path(cx);
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::base(self.theme_mode))
                                            .child("Add Path"),
                                    )
                                    .into_any_element()
                            }),
                    ),
            )
            // Custom paths list
            .child(
                div()
                    .w_full()
                    .bg(Theme::surface0(self.theme_mode))
                    .rounded_lg()
                    .border_1()
                    .border_color(Theme::surface1(self.theme_mode))
                    .flex()
                    .flex_col()
                    .children(if custom_paths.is_empty() {
                        vec![div().w_full().px_4().py_6().flex().justify_center().child(
                            div()
                                .text_sm()
                                .text_color(Theme::subtext0(self.theme_mode))
                                .child("No custom paths configured"),
                        )]
                    } else {
                        custom_paths
                            .iter()
                            .enumerate()
                            .map(|(idx, path)| self.render_custom_path_item(idx, path.clone(), cx))
                            .collect()
                    }),
            )
    }

    /// Render a single custom path item
    fn render_custom_path_item(
        &self,
        index: usize,
        path: CustomPath,
        cx: &mut ViewContext<Self>,
    ) -> Div {
        let path_display = path.path.display().to_string();
        let is_enabled = path.enabled;

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
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(if is_enabled {
                                Theme::text(self.theme_mode)
                            } else {
                                Theme::subtext0(self.theme_mode)
                            })
                            .child(path.label.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::subtext0(self.theme_mode))
                            .child(path_display),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Toggle button
                    .child(
                        div()
                            .id(SharedString::from(format!("toggle-path-{}", index)))
                            .px_2()
                            .py_1()
                            .bg(if is_enabled {
                                Theme::green(self.theme_mode)
                            } else {
                                Theme::surface1(self.theme_mode)
                            })
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|style| style.opacity(0.8))
                            .on_click(cx.listener(move |this, _event, cx| {
                                this.toggle_custom_path(index, cx);
                            }))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if is_enabled {
                                        Theme::base(self.theme_mode)
                                    } else {
                                        Theme::text(self.theme_mode)
                                    })
                                    .child(if is_enabled { "On" } else { "Off" }),
                            ),
                    )
                    // Remove button
                    .child(
                        div()
                            .id(SharedString::from(format!("remove-path-{}", index)))
                            .px_2()
                            .py_1()
                            .bg(Theme::red(self.theme_mode))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|style| style.opacity(0.8))
                            .on_click(cx.listener(move |this, _event, cx| {
                                this.remove_custom_path(index, cx);
                            }))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(Theme::base(self.theme_mode))
                                    .child("X"),
                            ),
                    ),
            )
    }
}
