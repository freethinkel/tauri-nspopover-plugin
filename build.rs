const COMMANDS: &[&str] = &["show_popover", "hide_popover", "is_popover_shown"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
