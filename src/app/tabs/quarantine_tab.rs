use crate::app::state::{DevSweep, QuarantineItemData, QuarantineRecordData};
use crate::ui::Theme;
use gpui::prelude::FluentBuilder;
use gpui::*;

impl DevSweep {
    pub fn render_quarantine_tab(&mut self, cx: &mut ViewContext<Self>) -> Div {
        let is_cleaning = self.is_cleaning;
        let status_text = self.status_text.clone();
        let total_size = self.quarantine_total_size.clone();
        let total_items = self.quarantine_total_items;
        let records = self.quarantine_records.clone();
        let items = self.quarantine_items.clone();
        let records_empty = records.is_empty();

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
                                    .child("Quarantine"),
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
                            .child(
                                div()
                                    .id("refresh-quarantine-btn")
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
                                        this.refresh_quarantine();
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::text(self.theme_mode))
                                            .child("Refresh"),
                                    ),
                            )
                            .child(
                                div()
                                    .id("open-finder-btn")
                                    .px_4()
                                    .py_2()
                                    .bg(Theme::surface0(self.theme_mode))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|style| style.bg(Theme::surface1(self.theme_mode)))
                                    .active(|style| {
                                        style.bg(Theme::surface2(self.theme_mode)).opacity(0.9)
                                    })
                                    .on_click(cx.listener(|_this, _event, _cx| {
                                        if let Some(cache_dir) = dirs::cache_dir() {
                                            let quarantine_path = cache_dir
                                                .join("development-cleaner")
                                                .join("quarantine");
                                            let _ = std::fs::create_dir_all(&quarantine_path);
                                            let _ = std::process::Command::new("open")
                                                .arg(quarantine_path)
                                                .spawn();
                                        }
                                    }))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(Theme::text(self.theme_mode))
                                            .child("Open in Finder"),
                                    ),
                            )
                            .when(!records_empty && !is_cleaning, |d| {
                                d.child(
                                    div()
                                        .id("clear-all-btn")
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
                                            this.clear_all_quarantine(cx);
                                        }))
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(Theme::crust(self.theme_mode))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Delete All"),
                                        ),
                                )
                            })
                            .when(records_empty || is_cleaning, |d| {
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
                                                .child("Delete All"),
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
                                    .child("Quarantine Size:"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(Theme::peach(self.theme_mode))
                                    .child(total_size),
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
                                    .child("Total Items:"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(Theme::blue(self.theme_mode))
                                    .child(format!("{}", total_items)),
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
                                    .child("Records:"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(Theme::lavender(self.theme_mode))
                                    .child(format!("{}", records.len())),
                            ),
                    ),
            )
            // Info banner
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_2()
                    .bg(Theme::blue_tint(self.theme_mode))
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().text_sm().child("ℹ️"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::blue(self.theme_mode))
                            .child("Deleted files are quarantined for undo support. They are automatically cleaned when exceeding 10GB."),
                    ),
            )
            // Records list
            .child(
                div()
                    .id("quarantine-content")
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .child(if records_empty {
                        self.empty_state("No cleanup history yet")
                    } else {
                        div().w_full().flex().flex_col().children(
                            records.iter().enumerate().map(|(record_idx, record)| {
                                let record_items: Vec<_> = items
                                    .iter()
                                    .filter(|item| item.record_id == record.id)
                                    .cloned()
                                    .collect();

                                self.render_quarantine_record(
                                    record.clone(),
                                    record_items,
                                    record_idx,
                                    cx,
                                )
                            }),
                        )
                    }),
            )
    }

    pub fn render_quarantine_record(
        &self,
        record: QuarantineRecordData,
        items: Vec<QuarantineItemData>,
        record_idx: usize,
        cx: &mut ViewContext<Self>,
    ) -> Div {
        let record_id = record.id.to_string();
        let expanded = record.expanded;
        let can_undo = record.can_undo;
        let has_errors = record.error_count > 0;

        div()
            .w_full()
            .flex()
            .flex_col()
            .border_b_1()
            .border_color(Theme::surface0(self.theme_mode))
            // Record header
            .child(
                div()
                    .id(SharedString::from(format!("qr-header-{}", record_idx)))
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
                        this.toggle_quarantine_record_expand(record_idx, cx);
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::subtext0(self.theme_mode))
                            .child(if expanded { "▼" } else { "▶" }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(Theme::text(self.theme_mode))
                                    .child(record.timestamp.clone()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(Theme::green(self.theme_mode))
                                            .child(format!("✓ {}", record.success_count)),
                                    )
                                    .when(has_errors, |d| {
                                        d.child(
                                            div()
                                                .text_xs()
                                                .text_color(Theme::red(self.theme_mode))
                                                .child(format!("✗ {}", record.error_count)),
                                        )
                                    }),
                            ),
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
                                    .child(format!("{} items", record.item_count)),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(Theme::peach(self.theme_mode))
                            .child(record.total_size.clone()),
                    )
                    .when(can_undo, |d| {
                        let record_id_clone = record_id.clone();
                        d.child(
                            div()
                                .id(SharedString::from(format!("undo-btn-{}", record_idx)))
                                .px_3()
                                .py_1()
                                .bg(Theme::blue(self.theme_mode))
                                .rounded_md()
                                .cursor_pointer()
                                .hover(|style| style.bg(Theme::sapphire(self.theme_mode)))
                                .active(|style| {
                                    style.bg(Theme::blue_active(self.theme_mode)).opacity(0.9)
                                })
                                .on_click(cx.listener(move |this, _event, cx| {
                                    this.undo_record(record_id_clone.clone(), cx);
                                }))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(Theme::crust(self.theme_mode))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Undo All"),
                                ),
                        )
                    }),
            )
            // Items (if expanded)
            .when(expanded, |d| {
                d.child(
                    div()
                        .w_full()
                        .flex()
                        .flex_col()
                        .bg(Theme::base(self.theme_mode))
                        .children(
                            items
                                .iter()
                                .map(|item| self.render_quarantine_item(item.clone(), cx)),
                        ),
                )
            })
    }

    pub fn render_quarantine_item(
        &self,
        item: QuarantineItemData,
        cx: &mut ViewContext<Self>,
    ) -> Div {
        let success = item.success;
        let has_error = !item.error_message.is_empty();
        let deleted_permanently = item.deleted_permanently;
        let can_delete = !deleted_permanently && item.quarantine_path.is_some();
        let record_id = item.record_id.clone();
        let item_index = item.item_index;

        div()
            .w_full()
            .px_4()
            .pl_12()
            .py_2()
            .flex()
            .items_center()
            .gap_3()
            .hover(|style| style.bg(Theme::surface0(self.theme_mode)))
            .border_b_1()
            .border_color(Theme::border_subtle(self.theme_mode))
            .child(
                div()
                    .text_sm()
                    .child(if success { "✓" } else { "✗" })
                    .text_color(if success {
                        Theme::green(self.theme_mode)
                    } else {
                        Theme::red(self.theme_mode)
                    }),
            )
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
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::overlay0(self.theme_mode))
                            .child(item.original_path.clone()),
                    )
                    .when(has_error, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(Theme::red(self.theme_mode))
                                .child(item.error_message.clone()),
                        )
                    }),
            )
            .when(deleted_permanently, |d| {
                d.child(
                    div()
                        .px_2()
                        .py_1()
                        .bg(Theme::surface1(self.theme_mode))
                        .rounded_sm()
                        .child(
                            div()
                                .text_xs()
                                .text_color(Theme::subtext0(self.theme_mode))
                                .child("Permanent"),
                        ),
                )
            })
            .child(
                div()
                    .text_sm()
                    .text_color(Theme::subtext1(self.theme_mode))
                    .child(item.size_str.clone()),
            )
            .when(can_delete, |d| {
                d.child(
                    div()
                        .id(SharedString::from(format!(
                            "delete-item-{}-{}",
                            record_id, item_index
                        )))
                        .px_3()
                        .py_1()
                        .bg(Theme::red(self.theme_mode))
                        .rounded_md()
                        .cursor_pointer()
                        .hover(|style| style.bg(Theme::red_hover(self.theme_mode)))
                        .active(|style| style.bg(Theme::red_active(self.theme_mode)).opacity(0.9))
                        .on_click(cx.listener(move |this, _event, cx| {
                            this.delete_quarantine_item(record_id.to_string(), item_index, cx);
                        }))
                        .child(
                            div()
                                .text_xs()
                                .text_color(Theme::crust(self.theme_mode))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("Delete"),
                        ),
                )
            })
    }
}
