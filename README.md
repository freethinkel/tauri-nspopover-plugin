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
tauri-plugin-nspopover = { git = "https://github.com/freethinkel/tauri-nspopover-plugin.git" }
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
    "resizable": true,
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
