// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconEvent}, ActivationPolicy, Manager};
use tauri_plugin_nspopover::{AppExt, WindowExt};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_nspopover::init())
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);
            let window = app.handle().get_webview_window("main").unwrap();
            window.to_popover();

            let tray = app.tray_by_id("main").unwrap();
            let handle = app.handle().clone();


            tray.on_tray_icon_event(move |_, event| {
              match event {
                TrayIconEvent::Click { button, button_state, .. } => {
                  if button == MouseButton::Left && button_state == MouseButtonState::Up {
                      if !handle.is_popover_shown() {
                          handle.show_popover();
                      } else {
                          handle.hide_popover();
                      }
                  }
                },
                _ => {}
              }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
