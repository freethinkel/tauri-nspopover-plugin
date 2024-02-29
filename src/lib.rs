use cocoa::{appkit::NSStatusItem, base::id};
use icrate::{
    objc2::rc::Id,
    AppKit::{NSPopover, NSStatusBarButton, NSWindow},
    Foundation::{CGSize, NSRectEdgeMaxY},
};

use tauri::{tray::TrayIcon, AppHandle, LogicalSize, Manager, Runtime, WebviewWindow};

use std::{
    cell::RefCell,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex},
};

mod popover;

use popover::PopoverController;

pub trait WindowExt<R: Runtime> {
    fn to_popover(&self);
}

pub use tauri::tray::TrayIconId;

pub struct StatusItem {
    pub(crate) id: TrayIconId,
    inner: TrayIconWrapper,
    pub(crate) app_handle: AppHandle,
}

impl AsRef<str> for StatusItem {
    fn as_ref(&self) -> &str {
        <TrayIconId as AsRef<str>>::as_ref(&self.id)
    }
}

impl StatusItem {
    pub fn app_handle(&self) -> &AppHandle {
        &self.app_handle
    }
}

#[derive(Debug)]
pub struct TrayIconWrapper {
    pub(crate) id: TrayIconId,
    tray: Rc<RefCell<MacosTrayIcon>>,
}

impl AsRef<str> for TrayIconWrapper {
    fn as_ref(&self) -> &str {
        <TrayIconId as AsRef<str>>::as_ref(&self.id)
    }
}

#[derive(Debug)]
pub struct MacosTrayIcon {
    ns_status_item: Option<id>,
    pub(crate) tray_target: Option<id>,
    pub(crate) id: TrayIconId,
}

impl Deref for MacosTrayIcon {
    type Target = Option<id>;

    fn deref(&self) -> &Self::Target {
        &self.tray_target
    }
}

impl AsRef<str> for MacosTrayIcon {
    fn as_ref(&self) -> &str {
        <TrayIconId as AsRef<str>>::as_ref(&self.id)
    }
}

pub trait StatusItemGetter {
    fn get_status_bar_button(&self) -> Id<NSStatusBarButton>;
}

impl<R: Runtime> StatusItemGetter for TrayIcon<R> {
    fn get_status_bar_button(&self) -> Id<NSStatusBarButton> {
        let status_item =
            unsafe { std::mem::transmute::<&TrayIcon<R>, &StatusItem>(self) as &StatusItem };

        let macos_tray = status_item.inner.tray.clone();
        let ns_status_item = unsafe { macos_tray.as_ptr().read().ns_status_item.unwrap() };
        let ns_status_item: id = unsafe { std::mem::transmute(ns_status_item) };
        let ns_status_button = unsafe { ns_status_item.button() };
        let ns_status_button = unsafe { std::mem::transmute(ns_status_button) };

        return ns_status_button;
    }
}

impl<R: Runtime> WindowExt<R> for WebviewWindow<R> {
    fn to_popover(&self) {
        let tray = self.app_handle().tray().unwrap();

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

        let popover = Arc::new(Mutex::new(SafeNSPopover(
            popover_controller.popover().clone(),
        )));
        let popover = Arc::clone(&popover);
        let button = Arc::new(Mutex::new(SafeNSStatusBarButton(button.clone())));
        let button = Arc::clone(&button);
        tray.on_tray_icon_event(move |_, _event| unsafe {
            let popover = popover.lock().unwrap().0.clone();
            let button = button.lock().unwrap().0.clone();
            let rect = button.bounds();

            if !popover.isShown() {
                popover.showRelativeToRect_ofView_preferredEdge(
                    rect,
                    button.as_ref(),
                    NSRectEdgeMaxY,
                );
            } else {
                popover.performClose(None);
            }
        });
    }
}

struct SafeNSPopover(Id<NSPopover>);
struct SafeNSStatusBarButton(Id<NSStatusBarButton>);

unsafe impl Send for SafeNSPopover {}
unsafe impl Send for SafeNSStatusBarButton {}
