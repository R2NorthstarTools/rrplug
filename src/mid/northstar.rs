//! has low abstractions to northstar stuff
use once_cell::sync::OnceCell;
use std::ffi::c_char;
use windows::Win32::Foundation::HMODULE;

use crate::{
    bindings::plugin_abi::{LogLevel, PluginNorthstarData},
    create_external_interface,
    high::UnsafeHandle,
    interfaces::external::SourceInterface,
};

pub static NORTHSTAR_DATA: OnceCell<NorthstarData> = OnceCell::new();

pub struct NorthstarData {
    pub(crate) handle: HMODULE,
    pub(crate) sys: UnsafeHandle<&'static NorthstarSys>,
}

impl NorthstarData {
    fn sys(&self) -> &'static NorthstarSys {
        self.sys.copy()
    }

    fn handle(&self) -> HMODULE {
        self.handle
    }
}

create_external_interface! {
    pub NorthstarSys + NorthstarSysMod => {
        pub fn log(handle: HMODULE, level: LogLevel, msg: *const c_char) -> ();
        pub fn unload(handle: HMODULE) -> ();
    }
}
/// should only be used by [`crate::entry`] to init northstar interfaces
#[doc(hidden)]
pub unsafe fn init_northstar_interfaces(dll_ptr: HMODULE, plugin_data: &PluginNorthstarData) {
    NORTHSTAR_DATA.set(NorthstarData {
        handle: plugin_data.handle,
        sys: unsafe {
            UnsafeHandle::new(
                NorthstarSys::from_dll_ptr(dll_ptr, "NSSys001").expect("NSSys001 is invalid"),
            )
        },
    });
}
