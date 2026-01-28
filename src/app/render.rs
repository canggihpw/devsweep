use crate::app::state::DevSweep;
use crate::assets::Assets;
use crate::ui::sidebar::Tab;
use crate::ui::Theme;
use crate::update_checker;
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
            // Main content area with left margin for breathing room from sidebar
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .overflow_hidden()
                    .pl_2() // Add left padding to separate content from sidebar
                    .child(match active_tab {
                        Tab::Scan => self.render_scan_tab(cx),
                        Tab::Trends => self.render_trends_tab(cx),
                        Tab::Quarantine => self.render_quarantine_tab(cx),
                        Tab::Settings => self.render_settings_tab(cx),
                        Tab::About => self.render_about_tab(cx),
                    }),
            )
    }
}

impl DevSweep {
    pub fn render_sidebar(&mut self, cx: &mut ViewContext<Self>) -> Div {
        let active_tab = self.active_tab;
        let storage_available = self.storage_available.clone();
        let storage_used_fraction = self.storage_used_fraction;

        div()
            .w(px(200.0))
            .h_full()
            .bg(Theme::mantle(self.theme_mode))
            .border_r_1()
            .border_color(Theme::surface0(self.theme_mode))
            .flex()
            .flex_col()
            // Logo, title, and version info
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_4()
                    .flex()
                    .items_center()
                    .gap_3()
                    .border_b_1()
                    .border_color(Theme::surface0(self.theme_mode))
                    .child(
                        if let Some(icon) = Assets::get_icon(self.theme_mode.icon_path()) {
                            img(icon)
                                .w(px(36.0))
                                .h(px(36.0))
                                .id("sidebar-logo")
                                .into_any_element()
                        } else {
                            div().w(px(36.0)).h(px(36.0)).into_any_element()
                        },
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_0()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(Theme::text(self.theme_mode))
                                    .child("DevSweep"),
                            )
                            .child(self.render_version_indicator(cx)),
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
                    .child(self.sidebar_item(Tab::Trends, active_tab == Tab::Trends, cx))
                    .child(self.sidebar_item(Tab::Quarantine, active_tab == Tab::Quarantine, cx))
                    .child(self.sidebar_item(Tab::Settings, active_tab == Tab::Settings, cx))
                    .child(self.sidebar_item(Tab::About, active_tab == Tab::About, cx)),
            )
            // Storage info and theme toggle at bottom
            .child(
                div()
                    .w_full()
                    .p_4()
                    .border_t_1()
                    .border_color(Theme::surface0(self.theme_mode))
                    .flex()
                    .flex_col()
                    .gap_3()
                    // Storage info with progress bar
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
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
                            )
                            // Storage progress bar
                            .child(
                                div()
                                    .w_full()
                                    .h(px(4.0))
                                    .bg(Theme::surface1(self.theme_mode))
                                    .rounded_full()
                                    .child(
                                        div()
                                            .h_full()
                                            .w(relative(storage_used_fraction))
                                            .bg(if storage_used_fraction > 0.9 {
                                                Theme::red(self.theme_mode)
                                            } else if storage_used_fraction > 0.75 {
                                                Theme::yellow(self.theme_mode)
                                            } else {
                                                Theme::green(self.theme_mode)
                                            })
                                            .rounded_full(),
                                    ),
                            ),
                    )
                    // Compact theme toggle (icon-only style)
                    .child(
                        div()
                            .id("theme-toggle")
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(Theme::subtext0(self.theme_mode))
                                    .child("Theme"),
                            )
                            .child(
                                div()
                                    .id("theme-toggle-btn")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(Theme::surface0(self.theme_mode))
                                    .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
                                    .active(|style| {
                                        style.bg(Theme::surface2(self.theme_mode)).opacity(0.9)
                                    })
                                    .on_click(cx.listener(|this, _event, cx| {
                                        this.theme_mode = this.theme_mode.toggle();
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .child(if self.theme_mode.is_dark() {
                                                "🌙"
                                            } else {
                                                "☀️"
                                            }),
                                    ),
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

    /// Renders the version indicator below the app name
    /// Shows current version, and update availability status
    fn render_version_indicator(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        let theme = self.theme_mode;
        let current_version = update_checker::current_version();

        if self.is_checking_update {
            // Checking for updates - show version with subtle indicator
            div()
                .flex()
                .items_center()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .text_color(Theme::subtext0(theme))
                        .child(format!("v{}", current_version)),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(Theme::subtext0(theme))
                        .child("..."),
                )
                .into_any_element()
        } else if let Some(ref info) = self.update_info {
            // Update available - show clickable indicator
            let new_version = info.version.clone();

            div()
                .id("sidebar-update-available")
                .flex()
                .items_center()
                .gap_1()
                .cursor_pointer()
                .hover(|style| style.opacity(0.8))
                .on_click(cx.listener(|this, _event, _cx| {
                    this.download_update();
                }))
                .child(
                    div()
                        .text_xs()
                        .text_color(Theme::subtext0(theme))
                        .child(format!("v{}", current_version)),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(Theme::green(theme))
                        .child(format!("-> v{}", new_version)),
                )
                .into_any_element()
        } else {
            // No update or up to date - just show version
            div()
                .text_xs()
                .text_color(Theme::subtext0(theme))
                .child(format!("v{}", current_version))
                .into_any_element()
        }
    }
}
