# Tauri Plugin NSpopover

### for Tauri v2

Only for MacOS

<div style="display: flex; justify-content: center;">
  <img src="./screenshots/example.png" width="300"/>
</div>

## Install

```toml
# Cargo.toml
[dependencies]
tauri-plugin-nspopover = { git = "https://github.com/freethinkel/tauri-nspopover-plugin.git", branch = "tauri-beta/v2", version = "3.0.0" }
```

```json
// package.json
"dependencies": {
  "tauri-plugin-nspopover": "git+https://github.com/freethinkel/tauri-nspopover-plugin#tauri-beta/v2"
}
```

## Usage

```rust
// main.rs
use tauri::{ActivationPolicy, Manager};
use tauri_plugin_nspopover::WindowExt;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);
            let window = app.app_handle().get_window("main").unwrap();
            window.to_popover();
            Ok(())
        })
        .plugin(tauri_plugin_nspopover::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

```ts
// main.ts
import { TrayIcon } from "@tauri-apps/api/tray";
import { isOpen, show, hide } from "tauri-plugin-nspopover";

TrayIcon.new({
  id: "main",
  async action() {
    const isShown = await isOpen();
    if (isShown) {
      hide();
    } else {
      show();
    }
  },
});
```

OR you can use rust api

```rust
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
                if (!handle.is_popover_shown()) {
                    handle.show_popover();
                } else {
                    handle.hide_popover();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

```json
// tauri.config.json
"systemTray": {
  "iconPath": "icons/statusbar-icon.png",
  "iconAsTemplate": true,
  "id": "main"
},
...
"windows": [
  {
    "fullscreen": false,
    "visible": false,
    "title": "inboxion",
    "width": 300,
    "height": 450,
    "transparent": true
  }
]
```

## Example

```sh
git clone https://github.com/freethinkel/tauri-plugin-nspopover
cd tauri-plugin-accent-color/example
pnpm install
pnpm tauri dev
```
