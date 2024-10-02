use bitflags::bitflags;
use std::ffi::{c_char, c_void};
use windows::Win32::Foundation::HMODULE;

pub type CreateInterface =
    unsafe extern "C" fn(*const c_char, *mut InterfaceStatus) -> *const c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum InterfaceStatus {
    IfaceOk = 0,
    IfaceFailed,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PluginNorthstarData {
    pub handle: HMODULE,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum LogLevel {
    LogInfo,
    LogWarn,
    LogErr,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PluginString {
    Name,           // the name of the plugin
    LogName,        // the name used for logging
    DependencyName, // the name used for squirrel dependency constants created. The value returned for this has to be a valid squirrel identifier or the plugin will fail to load
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PluginField {
    Context,
    Color,
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct PluginContext: u32 {
        const DEDICATED = 0x1;
        const CLIENT = 0x2;
    }
}

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct PluginColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
