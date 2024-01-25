use std::sync::{Arc, Mutex};

use icrate::{
    objc2::{rc::Id, ClassType},
    AppKit::{NSImage, NSPopover, NSStatusBar, NSStatusBarButton},
    Foundation::{CGSize, MainThreadMarker, NSData, NSRectEdgeMaxY},
};

use crate::action::TargetActionHandler;

pub struct StatusBarController {
    button: Id<NSStatusBarButton>,
    popover: Id<NSPopover>,
}

struct SafeNSPopover(Id<NSPopover>);
struct SafeNSStatusBarButton(Id<NSStatusBarButton>);

unsafe impl Send for SafeNSPopover {}
unsafe impl Send for SafeNSStatusBarButton {}

pub static mut NSSTATUS_BAR_BUTTON_HANDLER: Option<TargetActionHandler> = None;

impl StatusBarController {
    pub fn new(popover: Id<NSPopover>, icon_data: Vec<u8>) -> Self {
        let button = Self::create_statusbar_button(icon_data.clone());

        return StatusBarController { button, popover };
    }

    pub fn set_on_click_handler(&self) {
        unsafe {
            let popover = Arc::new(Mutex::new(SafeNSPopover(self.popover.clone())));
            let popover = Arc::clone(&popover);
            let button = Arc::new(Mutex::new(SafeNSStatusBarButton(self.button.clone())));
            let button = Arc::clone(&button);

            let handler: TargetActionHandler = TargetActionHandler::new(&self.button, move || {
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
            NSSTATUS_BAR_BUTTON_HANDLER = Some(handler);
        }
    }

    fn create_statusbar_button(icon_data: Vec<u8>) -> Id<NSStatusBarButton> {
        unsafe {
            let mtm = MainThreadMarker::new().unwrap();
            let statusbar = NSStatusBar::systemStatusBar();
            let status_item = statusbar.statusItemWithLength(28.0);
            let status_button = status_item.button(mtm).unwrap();

            let ns_data = NSData::from_vec(icon_data);
            let image = NSImage::initWithData(NSImage::alloc(), ns_data.as_ref()).unwrap();

            image.setSize(CGSize {
                width: 18.0,
                height: 18.0,
            });
            image.setTemplate(true);
            status_button.setImage(Some(image.as_ref()));

            status_button
        }
    }
}
