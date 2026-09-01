use crate::app::state::DevSweep;
use crate::largest::{LargestEntry, TreemapRect};
use crate::ui::Theme;
use gpui::prelude::FluentBuilder;
use gpui::*;

impl DevSweep {
    pub fn render_largest_tab(&mut self, cx: &mut ViewContext<Self>) -> Div {
        let status_text = self.largest_status.clone();
        let dirs = self.largest_dirs.clone();
        let files = self.largest_files.clone();
        let is_scanning = self.is_largest_scanning;
        let has_data = !self.largest_treemap.is_empty();

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
                    .px_4()
                    .py_3()
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
                                    .child("Largest Files & Folders"),
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
                            .id("largest-rescan-btn")
                            .px_4()
                            .py_2()
                            .bg(Theme::blue(self.theme_mode))
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|style| style.bg(Theme::sapphire(self.theme_mode)))
                            .active(|style| {
                                style.bg(Theme::blue_active(self.theme_mode)).opacity(0.9)
                            })
                            .on_click(cx.listener(|this, _event, cx| {
                                this.start_largest_scan(cx);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(Theme::crust(self.theme_mode))
                                    .child(if is_scanning { "Scanning..." } else { "Rescan" }),
                            ),
                    ),
            )
            // Content
            .child(
                div()
                    .id("largest-content")
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .px_4()
                    .py_4()
                    .flex()
                    .flex_col()
                    .gap_5()
                    // Treemap of the largest folders
                    .when(has_data, |d| {
                        d.child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(Theme::subtext0(self.theme_mode))
                                        .child("Largest folders — treemap"),
                                )
                                .child(self.render_treemap()),
                        )
                    })
                    // Largest folders (list)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(Theme::subtext0(self.theme_mode))
                                    .child("Largest folders"),
                            )
                            .children(dirs.iter().enumerate().map(|(i, d)| {
                                self.render_entry_row(i, d, Theme::peach(self.theme_mode))
                            })),
                    )
                    // Largest files (list)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(Theme::subtext0(self.theme_mode))
                                    .child("Largest files"),
                            )
                            .children(files.iter().enumerate().map(|(i, f)| {
                                self.render_entry_row(i, f, Theme::blue(self.theme_mode))
                            })),
                    )
                    // Empty state
                    .when(!has_data && !is_scanning, |d| {
                        d.child(
                            div()
                                .py_10()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .gap_2()
                                .child(div().text_2xl().child("📊"))
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(Theme::subtext0(self.theme_mode))
                                        .child(
                                            "Click \"Rescan\" to find the largest files and folders",
                                        ),
                                ),
                        )
                    }),
            )
    }

    /// The treemap viewport, rendered as absolutely-positioned colored blocks.
    fn render_treemap(&self) -> Stateful<Div> {
        let rects = self.largest_treemap.clone();

        div()
            .id("largest-treemap")
            .w(px(crate::largest::TREEMAP_W))
            .h(px(crate::largest::TREEMAP_H))
            .relative()
            .bg(Theme::mantle(self.theme_mode))
            .rounded_md()
            .overflow_hidden()
            .border_1()
            .border_color(Theme::surface0(self.theme_mode))
            .children(
                rects
                    .iter()
                    .enumerate()
                    .map(|(i, rect)| self.render_treemap_rect(i, rect)),
            )
    }

    fn render_treemap_rect(&self, index: usize, rect: &TreemapRect) -> Div {
        let color = self.treemap_color(index);
        let show_label = rect.w >= 44.0 && rect.h >= 18.0;

        div()
            .absolute()
            .left(px(rect.x))
            .top(px(rect.y))
            .w(px(rect.w))
            .h(px(rect.h))
            .bg(color)
            .border_r_1()
            .border_b_1()
            .border_color(Theme::base(self.theme_mode))
            .flex()
            .items_start()
            .px_1()
            .py_1()
            .when(show_label, |d| {
                d.child(
                    div()
                        .text_xs()
                        .w_full()
                        .text_color(Theme::crust(self.theme_mode))
                        .child(rect.name.clone()),
                )
            })
    }

    /// A single list row: rank badge, name, path, size.
    fn render_entry_row(&self, index: usize, entry: &LargestEntry, accent: Rgba) -> Div {
        div()
            .w_full()
            .px_3()
            .py_2()
            .flex()
            .items_center()
            .gap_3()
            .rounded_md()
            .bg(Theme::surface0(self.theme_mode))
            .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
            .child(
                div()
                    .w_6()
                    .text_sm()
                    .text_color(accent)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child((index + 1).to_string()),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_0()
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::text(self.theme_mode))
                            .child(entry.name.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::overlay0(self.theme_mode))
                            .child(entry.path.display().to_string()),
                    ),
            )
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(accent)
                    .child(crate::utils::format_size(entry.size)),
            )
    }

    fn treemap_color(&self, index: usize) -> Rgba {
        let palette: [fn(crate::ui::ThemeMode) -> Rgba; 12] = [
            Theme::blue,
            Theme::sapphire,
            Theme::sky,
            Theme::teal,
            Theme::green,
            Theme::yellow,
            Theme::peach,
            Theme::red,
            Theme::mauve,
            Theme::pink,
            Theme::lavender,
            Theme::flamingo,
        ];
        palette[index % palette.len()](self.theme_mode)
    }
}
