# Tauri Plugin NSpopover

Only for MacOS

<div style="display: flex; justify-content: center;">
  <img src="./screenshots/example.png" width="300"/>
</div>

### implementation for tauri v2 can be found [here](https://github.com/freethinkel/tauri-nspopover-plugin/tree/tauri-beta/v2)

## How to use?

Cargo.toml

```toml
[dependencies]
tauri-plugin-nspopover = { git = "https://github.com/freethinkel/tauri-nspopover-plugin.git" }
```

main.rs

```rust
use tauri::{ActivationPolicy, Manager};
#[cfg(target_os = "macos")]
use tauri_plugin_nspopover::WindowExt;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);
            let window = app.app_handle().get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            window.to_popover();

            Ok(())
        })
        .system_tray(tray)
        .plugin(tauri_plugin_nspopover::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

tauri.config.json

```json

  "systemTray": {
    "iconPath": "icons/statusbar-icon.png",
    "iconAsTemplate": true
  },
  ...
  "windows": [
    {
      "fullscreen": false,
      "resizable": true,
      "title": "inboxion",
      "width": 300,
      "height": 450,
      "visible": false,
      "transparent": true
    }
  ]
```

## Example

```sh
git clone https://github.com/freethinkel/tauri-nspopover-plugin
cd tauri-nspopover-plugin/example
npm install
npm run tauri dev
```
