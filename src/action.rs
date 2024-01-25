use icrate::objc2::{
    class,
    declare::ClassDecl,
    msg_send, msg_send_id,
    rc::{Allocated, Id},
    runtime::{AnyClass, AnyObject, Sel},
    sel, Encode, RefEncode,
};

use std::fmt;
use std::sync::Once;

#[repr(C)]
pub struct Object {
    _priv: PrivateMarker,
}

unsafe impl RefEncode for Object {
    const ENCODING_REF: icrate::objc2::Encoding = icrate::objc2::Encoding::Block;
    // Implement the required methods...
}

pub type idd = *mut Object;
type PrivateMarker = [u8; 0];

#[derive(Debug)]
pub struct TargetActionHandler {}

impl TargetActionHandler {
    /// Returns a new TargetEventHandler.
    pub fn new<F: Fn() + Send + Sync + 'static>(control: &AnyObject, action: F) -> Self {
        let block = Box::new(Action(Box::new(action)));
        let ptr = Box::into_raw(block);

        unsafe {
            let class = register_invoker_class::<F>().as_ref().unwrap();
            let obj: Allocated<AnyObject> = msg_send_id![class, alloc];
            let obj: Id<AnyObject> = msg_send_id![obj, init];

            unsafe fn set_ivar<T: Encode>(obj: &mut AnyObject, name: &str, value: T) {
                *obj.get_mut_ivar::<T>(name) = value;
            }

            let obj_ptr: *mut AnyObject = std::mem::transmute(obj);
            set_ivar(&mut *obj_ptr, ACTION_CALLBACK_PTR, ptr);

            let _: () = msg_send![control, setAction: sel!(perform:)];
            let _: () = msg_send![control, setTarget: obj_ptr];
        }

        TargetActionHandler {}
    }
}

pub struct Action(Box<dyn Fn() + Send + Sync + 'static>);

unsafe impl RefEncode for Action {
    const ENCODING_REF: icrate::objc2::Encoding = icrate::objc2::Encoding::Object;
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Action").finish()
    }
}

pub static ACTION_CALLBACK_PTR: &str = "rstTargetActionPtr";

pub fn load<'a, T>(this: &'a AnyObject, ptr_name: &str) -> &'a T {
    unsafe {
        let ptr: *mut Action = *this.get_ivar(ptr_name);
        let obj = ptr as *const T;
        &*obj
    }
}

extern "C" fn perform<'a, F: Fn() + 'static>(this: &'a mut AnyObject, _: Sel, _sender: idd) {
    let action = load::<Action>(this, ACTION_CALLBACK_PTR);

    (action.0)();
}

pub(crate) fn register_invoker_class<F: Fn() + 'static>() -> *const AnyClass {
    static mut VIEW_CLASS: *const AnyClass = 0 as *const AnyClass;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("RSTTargetActionHandler", superclass).unwrap();

        decl.add_ivar::<*mut Action>(ACTION_CALLBACK_PTR);
        decl.add_method(sel!(perform:), perform::<F> as extern "C" fn(_, _, _));

        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
