#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, WindowEvent, api::shell};

fn main() {
    app_lib::run_with(|app| {
        let window = app.get_window("main").unwrap();

        // Intercept navigation / new tab events
        let handle = window.clone();
        window.on_window_event(move |event| {
            if let WindowEvent::NavigationRequested { url, .. } = event {
                if url.starts_with("https://yourwebsite.com") {
                    // open inside app
                    let _ = handle.load_url(&url);
                } else {
                    // external links open in system browser
                    let _ = shell::open(&handle.shell_scope(), url.clone(), None);
                }
            }
        });
    });
}
