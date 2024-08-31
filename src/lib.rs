use objc2::rc::Retained;
use objc2_app_kit::{NSPopover, NSStatusBarButton, NSWindow};
use objc2_foundation::{CGSize,  NSRectEdge};
use tauri::{
    plugin::{Builder, TauriPlugin},
    tray::TrayIcon,
    AppHandle, LogicalSize, Manager, Runtime, State, WebviewWindow,
};

use std::sync::Mutex;

mod popover;

use popover::PopoverController;

pub trait WindowExt<R: Runtime> {
    fn to_popover(&self);
}
pub trait AppExt<R: Runtime> {
    fn is_popover_shown(&self) -> bool;
    fn show_popover(&self);
    fn hide_popover(&self);
}

pub use tauri::tray::TrayIconId;

#[allow(dead_code)]
pub struct StatusItem<R: Runtime> {
    id: TrayIconId,
    pub(crate) inner: tray_icon::TrayIcon,
    app_handle: AppHandle<R>,
}

pub trait StatusItemGetter {
    fn get_status_bar_button(&self) -> Retained<NSStatusBarButton>;
}

impl<R: Runtime> StatusItemGetter for TrayIcon<R> {
    fn get_status_bar_button(&self) -> Retained<NSStatusBarButton> {
        let status_item: &StatusItem<R> =
            unsafe { std::mem::transmute::<&TrayIcon<R>, &StatusItem<R>>(self) };

        let mtm = status_item.inner.tray.as_ref().borrow().mtm;

        let tray = unsafe { status_item.inner.tray.try_borrow_unguarded().unwrap() };

        let status = tray.ns_status_item.as_ref().unwrap();
        let btn = unsafe { status.button(mtm).unwrap() };

        return btn;
    }
}

impl<R: Runtime> WindowExt<R> for WebviewWindow<R> {
    fn to_popover(&self) {
        let tray = self.app_handle().tray_by_id("main").unwrap();

        let button = tray.get_status_bar_button();

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

        let popover = SafeNSPopover(popover_controller.popover().clone());
        let button = SafeNSStatusBarButton(button.clone());

        let state = self.app_handle().state() as State<'_, AppState>;
        *state.0.lock().unwrap() = Some(AppStateInner { popover, button });
    }
}

impl<R: Runtime> AppExt<R> for AppHandle<R> {
    fn is_popover_shown(&self) -> bool {
        let state: State<AppState> = self.state();

        if state.0.lock().unwrap().as_ref().is_none() {
            return false;
        }

        let popover = state.0.lock().unwrap().as_ref().unwrap().popover.0.clone();

        unsafe { popover.isShown() }
    }
    fn show_popover(&self) {
        let state: State<AppState> = self.state();
        if state.0.lock().unwrap().as_ref().is_none() {
            return;
        }

        let popover = state.0.lock().unwrap().as_ref().unwrap().popover.0.clone();
        let binding = state.0.lock().unwrap();
        let button = binding.as_ref().unwrap().button.0.clone();
        let rect = button.bounds();

        if unsafe { !popover.isShown() } {
            unsafe {
                popover.showRelativeToRect_ofView_preferredEdge(
                    rect,
                    button.as_ref(),
                    NSRectEdge::MaxY,
                );
            }
        }
    }
    fn hide_popover(&self) {
        let state: State<AppState> = self.state();

        if state.0.lock().unwrap().as_ref().is_none() {
            return;
        }
        let popover = state.0.lock().unwrap().as_ref().unwrap().popover.0.clone();

        if unsafe { popover.isShown() } {
            unsafe { popover.performClose(None) };
        }
    }
}

struct SafeNSPopover(Retained<NSPopover>);
struct SafeNSStatusBarButton(Retained<NSStatusBarButton>);

unsafe impl Send for SafeNSPopover {}
unsafe impl Send for SafeNSStatusBarButton {}

#[tauri::command]
fn show_popover<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    app.show_popover();

    return Ok(());
}

#[tauri::command]
fn hide_popover<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    app.hide_popover();

    Ok(())
}

#[tauri::command]
fn is_popover_shown<R: Runtime>(app: AppHandle<R>) -> Result<bool, String> {
    return Ok(app.is_popover_shown());
}

struct AppStateInner {
    popover: SafeNSPopover,
    button: SafeNSStatusBarButton,
}

struct AppState(Mutex<Option<AppStateInner>>);

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("nspopover")
        .invoke_handler(tauri::generate_handler![
            show_popover,
            hide_popover,
            is_popover_shown
        ])
        .setup(|app, _| {
            app.manage(AppState(Mutex::new(None)));

            Ok(())
        })
        .build()
}
