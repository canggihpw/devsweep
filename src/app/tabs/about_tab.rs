use crate::app::state::DevSweep;
use crate::assets::Assets;
use crate::ui::Theme;
use gpui::*;

impl DevSweep {
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
                    .child(
                        if let Some(icon) = Assets::get_icon(self.theme_mode.icon_path()) {
                            img(icon)
                                .size(px(80.0))
                                .into_any_element()
                        } else {
                            div()
                                .size(px(80.0))
                                .into_any_element()
                        },
                    )
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
