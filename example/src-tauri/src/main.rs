// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{ActivationPolicy, Manager, SystemTray};
#[cfg(target_os = "macos")]
use tauri_plugin_nspopover::WindowExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let tray = SystemTray::new();

    tauri::Builder::default()
        .setup(move |app| {
            app.set_activation_policy(ActivationPolicy::Accessory);
            let window = app.handle().get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            window.to_popover();

            Ok(())
        })
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
