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

    unsafe { CStr::from_ptr(interface_name) }
        .to_str()
        .ok()
        .and_then(|name| interfaces.get(name))
        .map(|interface| interface.copy())
        .unwrap_or_else(|| {
            unsafe { *error = 1 };
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
