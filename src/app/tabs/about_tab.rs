use crate::app::state::DevSweep;
use crate::assets::Assets;
use crate::ui::Theme;
use crate::update_checker;
use gpui::*;

impl DevSweep {
    pub fn render_about_tab(&self, cx: &mut ViewContext<Self>) -> Div {
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
                            .child(format!("Version {}", update_checker::current_version())),
                    ),
            )
            // Update status section
            .child(self.render_update_section(cx))
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

    fn render_update_section(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        let theme = self.theme_mode;

        if self.is_checking_update {
            // Checking for updates
            div()
                .id("update-checking")
                .flex()
                .items_center()
                .gap_2()
                .px_4()
                .py_2()
                .bg(Theme::surface0(theme))
                .rounded_lg()
                .child(
                    div()
                        .text_sm()
                        .text_color(Theme::subtext1(theme))
                        .child("Checking for updates..."),
                )
                .into_any_element()
        } else if let Some(ref info) = self.update_info {
            // Update available
            let version = info.version.clone();
            let changelog = truncate_changelog(&info.changelog, 150);

            div()
                .id("update-available")
                .flex()
                .flex_col()
                .gap_3()
                .px_4()
                .py_3()
                .bg(Theme::surface0(theme))
                .rounded_lg()
                .max_w(px(400.0))
                .child(
                    div().flex().items_center().gap_2().child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(Theme::green(theme))
                            .child(format!("Update available: v{}", version)),
                    ),
                )
                // Changelog preview (truncated)
                .child(
                    div()
                        .text_xs()
                        .text_color(Theme::subtext0(theme))
                        .max_h(px(60.0))
                        .overflow_hidden()
                        .child(changelog),
                )
                // Action buttons
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .id("download-update-btn")
                                .px_3()
                                .py_1()
                                .bg(Theme::green(theme))
                                .rounded_md()
                                .cursor_pointer()
                                .hover(|style| style.opacity(0.8))
                                .on_click(cx.listener(|this, _event, _cx| {
                                    this.download_update();
                                }))
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(Theme::base(theme))
                                        .child("Download"),
                                ),
                        )
                        .child(
                            div()
                                .id("view-release-btn")
                                .px_3()
                                .py_1()
                                .bg(Theme::surface1(theme))
                                .rounded_md()
                                .cursor_pointer()
                                .hover(|style| style.opacity(0.8))
                                .on_click(cx.listener(|this, _event, _cx| {
                                    this.open_release_page();
                                }))
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(Theme::text(theme))
                                        .child("View Release"),
                                ),
                        ),
                )
                .into_any_element()
        } else if let Some(ref error) = self.update_error {
            // Error checking for updates
            let error_msg = truncate_error(error);

            div()
                .id("update-error")
                .flex()
                .flex_col()
                .gap_2()
                .px_4()
                .py_2()
                .bg(Theme::surface0(theme))
                .rounded_lg()
                .child(
                    div()
                        .text_sm()
                        .text_color(Theme::red(theme))
                        .child(format!("Could not check for updates: {}", error_msg)),
                )
                .child(
                    div()
                        .id("retry-update-check-btn")
                        .px_3()
                        .py_1()
                        .bg(Theme::surface1(theme))
                        .rounded_md()
                        .cursor_pointer()
                        .hover(|style| style.opacity(0.8))
                        .on_click(cx.listener(|this, _event, cx| {
                            this.check_for_updates(cx);
                        }))
                        .child(
                            div()
                                .text_sm()
                                .text_color(Theme::text(theme))
                                .child("Retry"),
                        ),
                )
                .into_any_element()
        } else if self.update_check_completed {
            // Up to date
            div()
                .id("update-up-to-date")
                .flex()
                .items_center()
                .gap_2()
                .px_4()
                .py_2()
                .bg(Theme::surface0(theme))
                .rounded_lg()
                .child(
                    div()
                        .text_sm()
                        .text_color(Theme::green(theme))
                        .child(format!(
                            "You're up to date (v{})",
                            update_checker::current_version()
                        )),
                )
                .child(
                    div()
                        .id("check-again-btn")
                        .px_3()
                        .py_1()
                        .bg(Theme::surface1(theme))
                        .rounded_md()
                        .cursor_pointer()
                        .hover(|style| style.opacity(0.8))
                        .on_click(cx.listener(|this, _event, cx| {
                            this.check_for_updates(cx);
                        }))
                        .child(
                            div()
                                .text_xs()
                                .text_color(Theme::subtext0(theme))
                                .child("Check Again"),
                        ),
                )
                .into_any_element()
        } else {
            // Initial state - not yet checked
            div()
                .id("check-for-updates-btn")
                .px_3()
                .py_1()
                .bg(Theme::surface0(theme))
                .rounded_md()
                .cursor_pointer()
                .hover(|style| style.bg(Theme::surface1(theme)))
                .on_click(cx.listener(|this, _event, cx| {
                    this.check_for_updates(cx);
                }))
                .child(
                    div()
                        .text_sm()
                        .text_color(Theme::text(theme))
                        .child("Check for Updates"),
                )
                .into_any_element()
        }
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

/// Truncate changelog text for preview display
fn truncate_changelog(text: &str, max_len: usize) -> String {
    // Remove markdown headers and clean up the text
    let cleaned: String = text
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();

    if cleaned.len() <= max_len {
        cleaned
    } else {
        format!("{}...", &cleaned[..max_len])
    }
}

/// Truncate error message for display
fn truncate_error(error: &str) -> String {
    if error.len() <= 50 {
        error.to_string()
    } else {
        format!("{}...", &error[..50])
    }
}
