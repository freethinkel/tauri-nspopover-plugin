use objc2::{msg_send, rc::Retained, runtime::Bool};
use objc2_app_kit::{NSColor, NSPopover, NSPopoverBehavior, NSView, NSViewController, NSWindow};
use objc2_foundation::MainThreadMarker;

pub struct PopoverController {
    popover: Retained<NSPopover>,
}

impl PopoverController {
    pub fn new(window: &NSWindow) -> Self {
        let popover = Self::create_popover(window);
        return PopoverController { popover };
    }

    pub fn popover(&self) -> Retained<NSPopover> {
        self.popover.clone()
    }

    fn get_target_view(ns_window: &NSWindow) -> Retained<NSView> {
        let view = ns_window.contentView().unwrap();
        view.setWantsLayer(true);
        unsafe {
            let color = NSColor::clearColor();
            let _: () = msg_send![&*view, setBackgroundColor: &*color];
            let _: () = msg_send![&*view, setOpaque: Bool::YES];
        }

        return view;
    }

    fn create_popover(window: &NSWindow) -> Retained<NSPopover> {
        let view = Self::get_target_view(window);
        unsafe {
            let mtm = MainThreadMarker::new().unwrap();
            let ctrl = NSViewController::new(mtm);

            ctrl.setView(view.as_ref());

            let popover = NSPopover::new(mtm);
            popover.setBehavior(NSPopoverBehavior::Transient);
            popover.setContentViewController(Some(ctrl.as_ref()));
            let content_size = window.frame().size;
            popover.setContentSize(content_size);

            popover
        }
    }
}
