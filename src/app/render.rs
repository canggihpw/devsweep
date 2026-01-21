use crate::app::state::DevSweep;
use crate::assets::Assets;
use crate::ui::sidebar::Tab;
use crate::ui::Theme;
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
            // Logo and title (side by side)
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
                            .text_xl()
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
}
