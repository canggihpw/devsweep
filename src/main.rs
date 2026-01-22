use devsweep::app::DevSweep;
use devsweep::assets::Assets;
use devsweep::single_instance::{cleanup_socket, get_socket_path, try_activate_existing_instance};
use gpui::*;
use std::io::{Read, Write};
use std::os::unix::net::UnixListener;

fn setup_single_instance_listener(cx: &mut AppContext) {
    let socket_path = get_socket_path();

    // Remove stale socket file if it exists
    let _ = std::fs::remove_file(&socket_path);

    // Create Unix socket listener
    if let Ok(listener) = UnixListener::bind(&socket_path) {
        listener.set_nonblocking(true).ok();

        // Poll for incoming connections periodically
        cx.spawn(|cx| async move {
            loop {
                Timer::after(std::time::Duration::from_millis(100)).await;

                if let Ok((mut stream, _)) = listener.accept() {
                    let mut buf = [0u8; 8];
                    if stream.read(&mut buf).is_ok() {
                        // Received activation request - activate the window
                        let _ = cx.update(|cx| {
                            cx.activate(true);
                            if let Some(window) = cx.windows().first() {
                                let _ = window.update(cx, |_, cx: &mut WindowContext| {
                                    cx.activate_window();
                                });
                            }
                        });
                        let _ = stream.write_all(b"ok");
                    }
                }
            }
        })
        .detach();
    }
}

fn main() {
    // Check if another instance is already running
    if try_activate_existing_instance() {
        return;
    }

    let app = App::new().with_assets(Assets);

    // Register reopen handler for macOS dock click behavior
    app.on_reopen(|cx: &mut AppContext| {
        if cx.windows().is_empty() {
            open_main_window(cx);
        } else {
            // Activate the existing window
            cx.activate(true);
            if let Some(window) = cx.windows().first() {
                let _ = window.update(cx, |_, cx: &mut WindowContext| {
                    cx.activate_window();
                });
            }
        }
    });

    app.run(|cx: &mut AppContext| {
        // Setup listener for other instances trying to launch
        setup_single_instance_listener(cx);

        open_main_window(cx);
    });

    // Cleanup socket on exit
    cleanup_socket();
}

fn open_main_window(cx: &mut AppContext) {
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

    // Activate the app to bring it to foreground
    cx.activate(true);
}
