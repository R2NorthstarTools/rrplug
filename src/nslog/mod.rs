use crate::bindings::plugin_abi::LogLevel;
use crate::high::UnsafeHandle;
use crate::mid::northstar::{NorthstarSys, NORTHSTAR_DATA};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::ffi::{c_char, CStr, CString};
use std::panic;
use windows::Win32::Foundation::HMODULE;

const C_STRING_ERROR: *const c_char =
    "rrplug logger failed to transform a String to a CString\0".as_ptr() as *const c_char;

static mut LOGGER: NorthstarLogger = NorthstarLogger {
    ns_sys: None,
    plugin_handle: HMODULE(0),
};

pub fn try_init(plugin_handle: HMODULE) -> Result<(), SetLoggerError> {
    panic::set_hook(Box::new(|info| {
        log::error!("");

        match info.location() {
            Some(location) => log::error!("plugin panicked at {}", location),
            None => log::error!("plugin panicked at unknown"),
        }

        log::error!("full message:");
        log::error!("{}", info.to_string());

        log::error!("");
    }));

    unsafe {
        LOGGER = NorthstarLogger::init(plugin_handle);

        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
    }
}

struct NorthstarLogger {
    ns_sys: Option<UnsafeHandle<&'static NorthstarSys>>,
    plugin_handle: HMODULE,
}

impl NorthstarLogger {
    fn init(plugin_handle: HMODULE) -> Self {
        Self {
            ns_sys: Some(UnsafeHandle::internal_new(NORTHSTAR_DATA.wait().sys.copy())),
            plugin_handle,
        }
    }
}

impl log::Log for NorthstarLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) || self.ns_sys.is_none() {
            return;
        }

        let msg = to_cstring(record.args());

        let Some(level) = level_to_log(record.metadata().level()) else {
            return;
        };

        unsafe {
            self.ns_sys.as_ref().unwrap_unchecked().get().log(
                self.plugin_handle,
                level,
                msg.as_ptr(),
            )
        }
    }

    fn flush(&self) {}
}

fn to_cstring<T>(string: T) -> CString
where
    T: ToString,
{
    CString::new(string.to_string())
        .unwrap_or_else(|_| CString::from(unsafe { CStr::from_ptr(C_STRING_ERROR) }))
}

/// this is needed because [`Level`] doesn't have the same order
const fn level_to_log(level: Level) -> Option<LogLevel> {
    match level {
        Level::Error => Some(LogLevel::LogErr),
        Level::Warn => Some(LogLevel::LogWarn),
        Level::Info => Some(LogLevel::LogInfo),
        Level::Debug => None,
        Level::Trace => None,
    }
}
