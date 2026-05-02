# Changelog

## 4.1.0

- Drop `freethinkel/tray-icon` fork dependency
- Switch `get_status_bar_button` to `tauri::tray::TrayIcon::with_inner_tray_icon`
  + upstream `tray_icon::TrayIcon::ns_status_item()` (available since
  `tray-icon` 0.21.1)
- Remove `unsafe { mem::transmute }` (eliminates undefined behavior risk)
- Public API unchanged
