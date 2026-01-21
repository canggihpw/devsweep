mod app;
mod backend;
mod cache_settings;
mod checkers;
mod cleanup_history;
mod scan_cache;
mod types;
mod ui;
mod utils;

use app::DevSweep;
use gpui::*;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(1100.0), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("DevSweep".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| DevSweep::new()),
        )
        .unwrap();
    });
}
