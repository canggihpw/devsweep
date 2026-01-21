use crate::app::state::{CacheTTLSetting, DevSweep};
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
}
