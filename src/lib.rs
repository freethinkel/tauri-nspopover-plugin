use icrate::{AppKit::NSWindow, Foundation::CGSize};
use statusbar::StatusBarController;
use tauri::{
    plugin::{Builder, TauriPlugin},
    LogicalSize, Manager, Runtime, Window,
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
        let _ = self.app_handle().tray_handle().destroy();
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("nspopover").build()
}
