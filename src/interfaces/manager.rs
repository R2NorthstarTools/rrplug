//! the interface registry is here pog!

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    ffi::{c_char, c_void, CStr},
};

use crate::high::UnsafeHandle;

use super::interface::{AsInterface, Interface};

/// SAFETY: keys cannot be removed or else a memory leak will accure, this whole thing lives for the entire duration of a plugin
static REGISTERED_INTERFACES: Lazy<Mutex<HashMap<&'static str, UnsafeHandle<*const c_void>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[export_name = "CreateInterface"]
unsafe extern "C" fn create_interface(
    interface_name: *const c_char,
    error: *mut i32,
) -> *const c_void {
    let interfaces = REGISTERED_INTERFACES.lock();

    unsafe { interface_name.as_ref() }
        .and_then(|interface_name| unsafe { CStr::from_ptr(interface_name) }.to_str().ok())
        .and_then(|name| interfaces.get(name))
        .map(|interface| interface.copy())
        .unwrap_or_else(|| {
            if let Some(error) = unsafe { error.as_mut() } {
                *error = 1;
            }
            std::ptr::null()
        })
}

/// registers a interface and allows for it's fetching from the CreateInterface function which is exposed by rrplug
///
/// # Example
/// ```
/// # use crate::rrplug;
/// # use rrplug_proc::as_interface;
/// use rrplug::prelude::*;
///
/// #[repr(C)]
/// struct Exposedinterface {
///     what_is_2p2: i32,
/// }
///
/// #[as_interface]
/// impl Exposedinterface {
///     fn new() -> Self {
///         Self { what_is_2p2: 2+2 }
///     }
///
///     pub const fn twoplustwo(&self) -> i32 {
///         self.what_is_2p2
///     }
/// }
///
/// _ = unsafe{ register_interface("Exposedinterface001", Exposedinterface::new()) };
/// ```
///
/// # Safety
///
/// marked as unsafe for now since I don't know if the current Interface api safe enough and wouldn't be exploited from safe rust
///
/// interfaces must be sync and send have to deal with the Interface struct instead of there struct which all handled by the [`crate::as_interface`] macro
pub unsafe fn register_interface<T: Send + Sync + 'static + AsInterface>(
    name: &'static str,
    interface: Interface<T>,
) -> &'static T {
    let interface = Box::leak(Box::new(interface));
    REGISTERED_INTERFACES.lock().insert(
        name,
        UnsafeHandle::internal_new(interface as *const _ as *const c_void),
    );

    &interface.data
}

#[cfg(test)]
mod test {
    use super::*;
    use crate as rrplug;
    struct Test;

    #[rrplug::as_interface]
    impl Test {
        pub fn new() -> Self {
            Self
        }
        pub fn foo(&self) {}
    }

    #[test]
    fn interface_exists_null() {
        unsafe {
            register_interface("test", Test::new());

            // check if it's not null, can't check if it's the actual interface since it's stored differently
            assert!(!create_interface(c"test".as_ptr(), std::ptr::null_mut()).is_null());
        }
    }

    #[test]
    fn interface_exists() {
        unsafe {
            register_interface("test2", Test::new());

            let mut error = 0;

            assert!(!create_interface(c"test2".as_ptr(), &mut error).is_null());
            assert_eq!(error, 0);
        }
    }

    #[test]
    fn interface_does_not_exist_null() {
        unsafe {
            assert!(create_interface(c"test3".as_ptr(), std::ptr::null_mut()).is_null(),);
        }
    }

    #[test]
    fn interface_does_not_exist() {
        unsafe {
            let mut error = 0;
            assert!(create_interface(c"test4".as_ptr(), &mut error).is_null());
            assert_eq!(error, 1);
        }
    }
}
