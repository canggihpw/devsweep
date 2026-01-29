use crate::app::state::DevSweep;
use crate::port_manager::{self, PortProcess, COMMON_PORTS};
use crate::ui::Theme;
use gpui::prelude::FluentBuilder;
use gpui::*;

impl DevSweep {
    pub fn render_ports_tab(&mut self, cx: &mut ViewContext<Self>) -> Div {
        let theme = self.theme_mode;
        let is_scanning = self.is_scanning_ports;
        let is_killing = self.is_killing_process;
        let port_filter = self.port_filter.clone();
        let processes = self.port_processes.clone();

        // Filter processes based on search
        let filtered_processes: Vec<_> = if port_filter.is_empty() {
            processes.clone()
        } else {
            let filter_lower = port_filter.to_lowercase();
            processes
                .iter()
                .filter(|p| {
                    p.port.to_string().contains(&port_filter)
                        || p.process_name.to_lowercase().contains(&filter_lower)
                        || p.user.to_lowercase().contains(&filter_lower)
                })
                .cloned()
                .collect()
        };

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .bg(Theme::base(theme))
            // Header
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_3()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(Theme::surface0(theme))
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
                                    .text_color(Theme::text(theme))
                                    .child("Port Manager"),
                            )
                            .when(!self.port_status.is_empty(), |d| {
                                d.child(
                                    div()
                                        .text_sm()
                                        .text_color(Theme::subtext0(theme))
                                        .child(self.port_status.clone()),
                                )
                            }),
                    )
                    // Right: Refresh button
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .when(is_scanning || is_killing, |d| {
                                d.child(
                                    div()
                                        .px_4()
                                        .py_2()
                                        .bg(Theme::surface1(theme))
                                        .rounded_md()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(Theme::subtext0(theme))
                                                .child(if is_scanning {
                                                    "Scanning..."
                                                } else {
                                                    "Killing..."
                                                }),
                                        ),
                                )
                            })
                            .when(!is_scanning && !is_killing, |d| {
                                d.child(
                                    div()
                                        .id("refresh-ports-btn")
                                        .px_4()
                                        .py_2()
                                        .bg(Theme::blue(theme))
                                        .rounded_md()
                                        .cursor_pointer()
                                        .hover(|style| style.bg(Theme::blue_hover(theme)))
                                        .active(|style| {
                                            style.bg(Theme::blue_active(theme)).opacity(0.9)
                                        })
                                        .on_click(cx.listener(|this, _event, cx| {
                                            this.scan_ports(cx);
                                        }))
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(Theme::crust(theme))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Refresh"),
                                        ),
                                )
                            }),
                    ),
            )
            // Filter/search bar
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_3()
                    .flex()
                    .items_center()
                    .gap_4()
                    .border_b_1()
                    .border_color(Theme::surface0(theme))
                    // Search input
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py_2()
                            .bg(Theme::surface0(theme))
                            .rounded_md()
                            .border_1()
                            .border_color(Theme::surface1(theme))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::subtext0(theme))
                                    .child("🔍"),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .text_color(if port_filter.is_empty() {
                                        Theme::overlay0(theme)
                                    } else {
                                        Theme::text(theme)
                                    })
                                    .child(if port_filter.is_empty() {
                                        "Filter by port or process name...".to_string()
                                    } else {
                                        port_filter.clone()
                                    }),
                            ),
                    )
                    // Stats pill
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .bg(Theme::surface0(theme))
                            .rounded_full()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(Theme::subtext0(theme))
                                    .child("Listening"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(Theme::blue(theme))
                                    .child(format!("{}", filtered_processes.len())),
                            ),
                    ),
            )
            // Common ports quick-access
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_2()
                    .flex()
                    .items_center()
                    .gap_2()
                    .border_b_1()
                    .border_color(Theme::surface0(theme))
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::subtext0(theme))
                            .child("Common:"),
                    )
                    .children(COMMON_PORTS.iter().take(8).map(|(port, _desc)| {
                        let port_val = *port;
                        let is_in_use = processes.iter().any(|p| p.port == port_val);

                        div()
                            .id(SharedString::from(format!("quick-port-{}", port_val)))
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(if is_in_use {
                                Theme::red_tint(theme)
                            } else {
                                Theme::surface0(theme)
                            })
                            .border_1()
                            .border_color(if is_in_use {
                                Theme::red(theme)
                            } else {
                                Theme::surface1(theme)
                            })
                            .hover(|style| style.bg(Theme::surface1(theme)))
                            .on_click(cx.listener(move |this, _event, cx| {
                                this.set_port_filter(port_val.to_string(), cx);
                            }))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if is_in_use {
                                        Theme::red(theme)
                                    } else {
                                        Theme::text(theme)
                                    })
                                    .child(format!("{}", port_val)),
                            )
                    }))
                    // Clear filter button
                    .when(!port_filter.is_empty(), |d| {
                        d.child(
                            div()
                                .id("clear-port-filter")
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .cursor_pointer()
                                .bg(Theme::surface0(theme))
                                .hover(|style| style.bg(Theme::surface1(theme)))
                                .on_click(cx.listener(|this, _event, cx| {
                                    this.set_port_filter(String::new(), cx);
                                }))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(Theme::subtext0(theme))
                                        .child("Clear"),
                                ),
                        )
                    }),
            )
            // Port list
            .child(
                div()
                    .id("ports-content")
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .child(if filtered_processes.is_empty() {
                        self.render_ports_empty_state()
                    } else {
                        div().w_full().flex().flex_col().children(
                            filtered_processes.iter().enumerate().map(|(idx, process)| {
                                self.render_port_item(process.clone(), idx, cx)
                            }),
                        )
                    }),
            )
    }

    fn render_ports_empty_state(&self) -> Div {
        let theme = self.theme_mode;

        div()
            .w_full()
            .flex_1()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_6()
            .pb_16()
            // Large icon
            .child(
                div()
                    .w(px(80.0))
                    .h(px(80.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_2xl()
                    .bg(Theme::surface0(theme))
                    .child(
                        div()
                            .text_color(Theme::subtext0(theme))
                            .child("🔌")
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
                            .text_color(Theme::text(theme))
                            .child(if self.port_filter.is_empty() {
                                "No ports in use"
                            } else {
                                "No matching ports"
                            }),
                    )
                    .child(div().text_sm().text_color(Theme::subtext0(theme)).child(
                        if self.port_filter.is_empty() {
                            "Click Refresh to scan for listening ports."
                        } else {
                            "Try a different filter or clear the search."
                        },
                    )),
            )
    }

    fn render_port_item(
        &self,
        process: PortProcess,
        _idx: usize,
        cx: &mut ViewContext<Self>,
    ) -> Div {
        let theme = self.theme_mode;
        let pid = process.pid;
        let port = process.port;
        let is_killing = self.is_killing_process;

        // Get description for common ports
        let port_desc = port_manager::get_port_description(port);

        div()
            .w_full()
            .px_4()
            .py_3()
            .flex()
            .items_center()
            .gap_4()
            .border_b_1()
            .border_color(Theme::surface0(theme))
            .hover(|style| style.bg(Theme::surface0(theme)))
            // Port number (prominent)
            .child(
                div()
                    .w(px(80.0))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .text_color(Theme::blue(theme))
                            .child(format!("{}", port)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::subtext0(theme))
                            .child(process.protocol.clone()),
                    ),
            )
            // Process info (expandable section)
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_1()
                    // Process name
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(Theme::text(theme))
                                    .child(process.process_name.clone()),
                            )
                            .when(port_desc.is_some(), |d| {
                                d.child(
                                    div()
                                        .px_2()
                                        .py_0()
                                        .bg(Theme::surface1(theme))
                                        .rounded_sm()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(Theme::subtext0(theme))
                                                .child(port_desc.unwrap_or("")),
                                        ),
                                )
                            }),
                    )
                    // PID and user
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(Theme::overlay0(theme))
                                    .child(format!("PID: {}", pid)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(Theme::overlay0(theme))
                                    .child(format!("User: {}", process.user)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(Theme::overlay0(theme))
                                    .child(format!("Addr: {}", process.local_address)),
                            ),
                    ),
            )
            // State badge
            .child(
                div()
                    .px_2()
                    .py_1()
                    .bg(if process.state == "LISTEN" {
                        Theme::green_tint(theme)
                    } else {
                        Theme::surface1(theme)
                    })
                    .rounded_md()
                    .child(
                        div()
                            .text_xs()
                            .text_color(if process.state == "LISTEN" {
                                Theme::green(theme)
                            } else {
                                Theme::subtext0(theme)
                            })
                            .child(process.state.clone()),
                    ),
            )
            // Kill button
            .when(!is_killing, |d| {
                d.child(
                    div()
                        .id(SharedString::from(format!("kill-btn-{}", pid)))
                        .px_3()
                        .py_2()
                        .bg(Theme::red(theme))
                        .rounded_md()
                        .cursor_pointer()
                        .hover(|style| style.bg(Theme::red_hover(theme)))
                        .active(|style| style.bg(Theme::red_active(theme)).opacity(0.9))
                        .on_click(cx.listener(move |this, _event, cx| {
                            this.kill_port_process(pid, port, false, cx);
                        }))
                        .child(
                            div()
                                .text_xs()
                                .text_color(Theme::crust(theme))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("Kill"),
                        ),
                )
            })
    }
}
