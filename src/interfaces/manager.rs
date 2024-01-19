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

#[no_mangle]
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

pub unsafe fn register_interface<T: Send + Sync + 'static + AsInterface>(
    name: &'static str,
    interface: Interface<T>,
) {
    REGISTERED_INTERFACES.lock().insert(
        name,
        UnsafeHandle::internal_new(Box::leak(Box::new(interface)) as *const _ as *const c_void),
    );
}
