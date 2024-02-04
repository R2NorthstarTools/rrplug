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

/// plugin sys related stuff
///
/// more at [`NorthstarData`]
pub static NORTHSTAR_DATA: OnceCell<NorthstarData> = OnceCell::new();

/// plugin sys related stuff
///
/// has the handle for logging and the "NSSys001" interface
pub struct NorthstarData {
    pub(crate) handle: HMODULE,
    pub(crate) sys: UnsafeHandle<&'static NorthstarSys>,
}

impl std::fmt::Debug for NorthstarData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NorhstarData")
            .field("handle", &self.handle)
            .finish_non_exhaustive()
    }
}

impl NorthstarData {
    /// returns the "NSSys001" interface
    pub const fn sys(&self) -> &'static NorthstarSys {
        self.sys.copy()
    }

    /// Returns the handle for logging system (useless).
    /// just here if you really need it
    pub const fn handle(&self) -> HMODULE {
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
    NORTHSTAR_DATA
        .set(NorthstarData {
            handle: plugin_data.handle,
            sys: unsafe {
                UnsafeHandle::new(
                    NorthstarSys::from_dll_ptr(dll_ptr, "NSSys001").expect("NSSys001 is invalid"),
                )
            },
        })
        .expect("northstar interfaces don't exist????????");

    super::source_alloc::SOURCE_ALLOC.init();
}
