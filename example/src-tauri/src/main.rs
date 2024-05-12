// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_nspopover::WindowExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_nspopover::init())
        .setup(|app| {
            let window = app.handle().get_webview_window("main").unwrap();
            window.to_popover();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
