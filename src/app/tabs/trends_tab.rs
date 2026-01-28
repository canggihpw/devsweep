use crate::app::state::DevSweep;
use crate::trends::TrendTimeRange;
use crate::ui::Theme;
use crate::utils::format_size;
use gpui::*;
use std::time::SystemTime;

impl DevSweep {
    pub fn render_trends_tab(&mut self, cx: &mut ViewContext<Self>) -> Div {
        let theme = self.theme_mode;

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
                    .p_4()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(Theme::surface0(theme))
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
                                    .child("Storage Trends"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(Theme::surface0(theme))
                                    .rounded_md()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(Theme::subtext0(theme))
                                            .child(format!(
                                                "{} snapshots",
                                                self.trend_snapshot_count
                                            )),
                                    ),
                            ),
                    )
                    // Time range selector
                    .child(self.render_time_range_selector(cx)),
            )
            // Content
            .child(
                div()
                    .id("trends-content")
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .p_4()
                    .child(if self.has_trend_data {
                        self.render_trends_content(cx)
                    } else {
                        self.render_no_trends_data(cx)
                    }),
            )
    }

    fn render_time_range_selector(&self, cx: &mut ViewContext<Self>) -> Div {
        let theme = self.theme_mode;
        let current_range = self.trend_time_range;

        div()
            .flex()
            .items_center()
            .gap_1()
            .children(TrendTimeRange::all_options().into_iter().map(|range| {
                let is_selected = range == current_range;
                div()
                    .id(SharedString::from(format!("trend-range-{:?}", range)))
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(if is_selected {
                        Theme::blue(theme)
                    } else {
                        Theme::surface0(theme)
                    })
                    .hover(|style| {
                        if is_selected {
                            style
                        } else {
                            style.bg(Theme::surface1(theme))
                        }
                    })
                    .on_click(cx.listener(move |this, _event, cx| {
                        this.set_trend_time_range(range, cx);
                    }))
                    .child(
                        div()
                            .text_sm()
                            .text_color(if is_selected {
                                Theme::base(theme)
                            } else {
                                Theme::text(theme)
                            })
                            .child(range.label()),
                    )
            }))
    }

    fn render_no_trends_data(&self, _cx: &mut ViewContext<Self>) -> Div {
        let theme = self.theme_mode;

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .text_3xl()
                    .child("📊"),
            )
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(Theme::text(theme))
                    .child("No Trend Data Yet"),
            )
            .child(
                div()
                    .max_w(px(400.0))
                    .flex()
                    .justify_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::subtext0(theme))
                            .child("Run a few scans over time to see storage trends. Each scan creates a snapshot of your reclaimable space."),
                    ),
            )
            .child(
                div()
                    .mt_4()
                    .p_4()
                    .bg(Theme::blue_tint(theme))
                    .rounded_lg()
                    .border_1()
                    .border_color(Theme::blue_border(theme))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_base().child("💡"))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::blue(theme))
                                    .child("Tip: Trends show how your reclaimable space changes over time"),
                            ),
                    ),
            )
    }

    fn render_trends_content(&self, cx: &mut ViewContext<Self>) -> Div {
        let theme = self.theme_mode;

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_6()
            // Summary cards
            .child(self.render_trend_summary_cards(cx))
            // Main chart
            .child(self.render_trend_chart(cx))
            // Category breakdown
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(Theme::text(theme))
                            .child("Category Breakdown"),
                    )
                    .child(self.render_category_trends(cx)),
            )
    }

    fn render_trend_summary_cards(&self, _cx: &mut ViewContext<Self>) -> Div {
        let theme = self.theme_mode;
        let trend_data = self.trend_data.as_ref();

        let total_freed = trend_data.map(|d| d.total_freed).unwrap_or(0);
        let net_change = trend_data.map(|d| d.net_change).unwrap_or(0);
        let cleanup_count = trend_data.map(|d| d.cleanup_count).unwrap_or(0);

        div()
            .w_full()
            .flex()
            .gap_4()
            // Total Space Freed card
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .bg(Theme::surface0(theme))
                    .rounded_lg()
                    .border_1()
                    .border_color(Theme::surface1(theme))
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_xl()
                                    .child("🧹"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::subtext0(theme))
                                    .child("Space Freed"),
                            ),
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(Theme::green(theme))
                            .child(format_size(total_freed)),
                    ),
            )
            // Net Change card
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .bg(Theme::surface0(theme))
                    .rounded_lg()
                    .border_1()
                    .border_color(Theme::surface1(theme))
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_xl()
                                    .child(if net_change >= 0 { "📈" } else { "📉" }),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::subtext0(theme))
                                    .child("Net Change"),
                            ),
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(if net_change >= 0 {
                                Theme::yellow(theme)
                            } else {
                                Theme::green(theme)
                            })
                            .child(format!(
                                "{}{}",
                                if net_change >= 0 { "+" } else { "" },
                                format_size(net_change.unsigned_abs())
                            )),
                    ),
            )
            // Cleanup Count card
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .bg(Theme::surface0(theme))
                    .rounded_lg()
                    .border_1()
                    .border_color(Theme::surface1(theme))
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_xl()
                                    .child("🔄"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::subtext0(theme))
                                    .child("Cleanups"),
                            ),
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(Theme::blue(theme))
                            .child(format!("{}", cleanup_count)),
                    ),
            )
    }

    fn render_trend_chart(&self, _cx: &mut ViewContext<Self>) -> Div {
        let theme = self.theme_mode;
        let trend_data = self.trend_data.as_ref();

        div()
            .w_full()
            .p_4()
            .bg(Theme::surface0(theme))
            .rounded_lg()
            .border_1()
            .border_color(Theme::surface1(theme))
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(Theme::text(theme))
                            .child("Reclaimable Space Over Time"),
                    )
                    .child(if let Some(data) = trend_data {
                        div()
                            .flex()
                            .gap_4()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::subtext0(theme))
                                    .child(format!("Min: {}", format_size(data.min_value))),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Theme::subtext0(theme))
                                    .child(format!("Max: {}", format_size(data.max_value))),
                            )
                            .into_any_element()
                    } else {
                        div().into_any_element()
                    }),
            )
            // Chart visualization
            .child(self.render_bar_chart(trend_data))
    }

    fn render_bar_chart(&self, trend_data: Option<&crate::trends::TrendData>) -> Div {
        let theme = self.theme_mode;
        let chart_height = 120.0;

        let Some(data) = trend_data else {
            return div()
                .w_full()
                .h(px(chart_height))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_sm()
                        .text_color(Theme::subtext0(theme))
                        .child("No data to display"),
                );
        };

        if data.points.is_empty() {
            return div()
                .w_full()
                .h(px(chart_height))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_sm()
                        .text_color(Theme::subtext0(theme))
                        .child("No data points available"),
                );
        }

        let range = if data.max_value > data.min_value {
            data.max_value - data.min_value
        } else {
            data.max_value.max(1)
        };

        // Sample points to fit display (max 30 bars)
        let max_bars = 30;
        let step = if data.points.len() > max_bars {
            data.points.len() / max_bars
        } else {
            1
        };

        let sampled_points: Vec<_> = data.points.iter().step_by(step).collect();

        div()
            .w_full()
            .h(px(chart_height))
            .flex()
            .flex_col()
            // Chart area
            .child(
                div()
                    .w_full()
                    .flex_1()
                    .flex()
                    .items_end()
                    .gap_1()
                    .children(sampled_points.iter().enumerate().map(|(i, (timestamp, value))| {
                        // Calculate bar height as pixels (chart area is ~100px)
                        let chart_area_height = 100.0_f32;
                        let height_px = if range > 0 {
                            (((*value - data.min_value) as f32 / range as f32) * chart_area_height).max(4.0)
                        } else {
                            chart_area_height / 2.0
                        };

                        let bar_id = SharedString::from(format!("chart-bar-{}", i));
                        let _time_str = format_timestamp(*timestamp);
                        let _size_str = format_size(*value);

                        div()
                            .id(bar_id)
                            .flex_1()
                            .min_w(px(8.0))
                            .max_w(px(20.0))
                            .h(px(height_px))
                            .bg(Theme::blue(theme))
                            .rounded_t_sm()
                            .cursor_pointer()
                            .hover(|style| style.bg(Theme::blue_hover(theme)))
                    })),
            )
            // X-axis line
            .child(
                div()
                    .w_full()
                    .h(px(1.0))
                    .bg(Theme::surface1(theme)),
            )
            // X-axis labels (first and last)
            .child(
                div()
                    .w_full()
                    .flex()
                    .justify_between()
                    .mt_1()
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::subtext0(theme))
                            .child(
                                sampled_points
                                    .last()
                                    .map(|(ts, _)| format_timestamp(*ts))
                                    .unwrap_or_default(),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::subtext0(theme))
                            .child(
                                sampled_points
                                    .first()
                                    .map(|(ts, _)| format_timestamp(*ts))
                                    .unwrap_or_default(),
                            ),
                    ),
            )
    }

    fn render_category_trends(&self, _cx: &mut ViewContext<Self>) -> Div {
        let theme = self.theme_mode;

        if self.category_trends.is_empty() {
            return div()
                .w_full()
                .p_4()
                .bg(Theme::surface0(theme))
                .rounded_lg()
                .flex()
                .justify_center()
                .child(
                    div()
                        .text_sm()
                        .text_color(Theme::subtext0(theme))
                        .child("No category data available"),
                );
        }

        // Find max size for scaling bars
        let max_size = self
            .category_trends
            .iter()
            .map(|c| c.current_size)
            .max()
            .unwrap_or(1);

        div()
            .w_full()
            .bg(Theme::surface0(theme))
            .rounded_lg()
            .border_1()
            .border_color(Theme::surface1(theme))
            .flex()
            .flex_col()
            .children(
                self.category_trends
                    .iter()
                    .take(10) // Show top 10 categories
                    .map(|cat_trend| {
                        // Calculate bar width as a fraction (0.0 to 1.0)
                        let bar_width_fraction = if max_size > 0 {
                            (cat_trend.current_size as f64 / max_size as f64).max(0.01)
                        } else {
                            0.0
                        };

                        let change_str = if cat_trend.change >= 0 {
                            format!("+{}", format_size(cat_trend.change as u64))
                        } else {
                            format!("-{}", format_size(cat_trend.change.unsigned_abs()))
                        };

                        div()
                            .w_full()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(Theme::surface1(theme))
                            .flex()
                            .flex_col()
                            .gap_2()
                            // Category name and size
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(Theme::text(theme))
                                            .child(cat_trend.category.clone()),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(Theme::text(theme))
                                                    .child(format_size(cat_trend.current_size)),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(if cat_trend.change >= 0 {
                                                        Theme::yellow(theme)
                                                    } else {
                                                        Theme::green(theme)
                                                    })
                                                    .child(change_str),
                                            ),
                                    ),
                            )
                            // Progress bar
                            .child(
                                div()
                                    .w_full()
                                    .h(px(6.0))
                                    .bg(Theme::surface1(theme))
                                    .rounded_full()
                                    .child(
                                        div()
                                            .h_full()
                                            .w(relative(bar_width_fraction as f32))
                                            .bg(Theme::blue(theme))
                                            .rounded_full(),
                                    ),
                            )
                    }),
            )
    }

    /// Update trend data based on current time range
    pub fn refresh_trends_data(&mut self) {
        let backend = self.backend.lock().unwrap();
        self.has_trend_data = backend.has_trend_data();
        self.trend_snapshot_count = backend.trend_snapshot_count();

        if self.has_trend_data {
            self.trend_data = Some(backend.get_trend_data(self.trend_time_range));
            self.category_trends = backend.get_category_trends(self.trend_time_range);
        } else {
            self.trend_data = None;
            self.category_trends = Vec::new();
        }
    }

    /// Set the trend time range and refresh data
    pub fn set_trend_time_range(&mut self, range: TrendTimeRange, cx: &mut ViewContext<Self>) {
        self.trend_time_range = range;
        self.refresh_trends_data();
        cx.notify();
    }
}

/// Format a SystemTime as a human-readable date string
fn format_timestamp(time: SystemTime) -> String {
    let duration = time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Convert to naive date (simple approach without chrono)
    let days_since_epoch = secs / 86400;
    let remaining_secs = secs % 86400;
    let hours = remaining_secs / 3600;

    // Simple date calculation (not accounting for leap years perfectly)
    let _years = 1970 + (days_since_epoch / 365);
    let day_of_year = days_since_epoch % 365;
    let month = (day_of_year / 30).min(11) + 1;
    let day = (day_of_year % 30) + 1;

    format!("{:02}/{:02} {:02}:00", month, day, hours)
}
