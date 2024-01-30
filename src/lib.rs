use icrate::{AppKit::NSWindow, Foundation::CGSize};
use statusbar::StatusBarController;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Env, LogicalSize, Manager, Runtime, Window,
};

use std::fs;

mod action;
mod popover;
mod statusbar;

use popover::PopoverController;

pub trait WindowExt<R: Runtime> {
    fn to_popover(&self);
}

impl<R: Runtime> WindowExt<R> for Window<R> {
    fn to_popover(&self) {
        let system_tray_config = self.app_handle().config().tauri.system_tray.clone();
        let icon_path = String::from(system_tray_config.unwrap().icon_path.to_str().unwrap());
        let env = Env::default();
        let res = tauri::api::path::resource_dir(self.app_handle().package_info(), &env).unwrap();
        let icon_path = res.join(icon_path);
        let icon = fs::read(icon_path).unwrap();

        let window = self;
        let window = window.ns_window().unwrap();
        let ns_window = unsafe { (window.cast() as *mut NSWindow).as_ref().unwrap() };

        let scale = self.scale_factor().unwrap();
        let size: LogicalSize<f64> = self.inner_size().unwrap().to_logical(scale);

        let popover_controller = PopoverController::new(
            ns_window,
            CGSize {
                width: size.width,
                height: size.height,
            },
        );
        let _ = self.hide();
        let statusbar_controller = StatusBarController::new(popover_controller.popover(), icon);
        statusbar_controller.set_on_click_handler();
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("nspopover").build()
}
