#![deny(unsafe_op_in_unsafe_fn)]
use icrate::objc2::ffi::{BOOL, NO};
use icrate::objc2::rc::Id;
use icrate::objc2::{declare_class, msg_send_id, mutability, ClassType, DeclaredClass};
use icrate::AppKit::{NSApplication, NSApplicationDelegate};
use icrate::Foundation::ns_string;
use icrate::Foundation::{MainThreadMarker, NSCopying, NSObject, NSObjectProtocol, NSString};

#[derive(Debug)]
#[allow(unused)]
pub struct Ivars {
    ivar: u8,
    another_ivar: bool,
    box_ivar: Box<i32>,
    maybe_box_ivar: Option<Box<i32>>,
    id_ivar: Id<NSString>,
    maybe_id_ivar: Option<Id<NSString>>,
}

declare_class!(
    pub struct AppDelegate;

    unsafe impl ClassType for AppDelegate {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "MyAppDelegate";
    }

    impl DeclaredClass for AppDelegate {
        type Ivars = Ivars;
    }

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {


      #[method(applicationShouldTerminateAfterLastWindowClosed:)]
      fn application_should_terminate_after_last_window_closed(&self,_sender: &NSApplication) -> BOOL {
        return NO.into();
      }
    }
);

impl AppDelegate {
    pub fn new(ivar: u8, another_ivar: bool, mtm: MainThreadMarker) -> Id<Self> {
        let this = mtm.alloc();
        let this = this.set_ivars(Ivars {
            ivar,
            another_ivar,
            box_ivar: Box::new(2),
            maybe_box_ivar: None,
            id_ivar: NSString::from_str("abc"),
            maybe_id_ivar: Some(ns_string!("def").copy()),
        });
        unsafe { msg_send_id![super(this), init] }
    }
}
