// Add this to src-tauri/src/window.rs

use tauri::{Manager, Window};
use tauri::window::WindowBuilder;

pub fn create_window_with_custom_titlebar(app: &tauri::AppHandle, url: &str, title: &str) -> tauri::Result<Window> {
    let window = WindowBuilder::new(
        app,
        "main",
        tauri::WindowUrl::External(url.parse().unwrap())
    )
    .title(title)
    .inner_size(1200.0, 800.0)
    .min_inner_size(800.0, 600.0)
    .decorations(false) // Disable default decorations
    .transparent(true)
    .build()?;

    Ok(window)
}

// Add these Tauri commands for navigation
#[tauri::command]
pub fn go_back(window: Window) {
    window.eval("window.history.back()").ok();
}

#[tauri::command]
pub fn go_forward(window: Window) {
    window.eval("window.history.forward()").ok();
}

#[tauri::command]
pub fn reload_page(window: Window) {
    window.eval("window.location.reload()").ok();
}
