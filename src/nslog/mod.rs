use crate::bindings::plugin_abi::{loggerfunc_t, LogMsg, MessageSource};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::ffi::{c_char, CStr, CString};
use std::panic;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const C_STRING_ERROR: *const c_char =
    "rrplug logger failed to transform a String to a CString\0".as_ptr() as *const c_char;

static mut LOGGER: NorthstarLogger = NorthstarLogger {
    logger: None,
    plugin_handle: 0,
};

pub fn try_init(logger: loggerfunc_t, plugin_handle: i32) -> Result<(), SetLoggerError> {
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
        LOGGER = NorthstarLogger::init(logger, plugin_handle);

        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
    }
}

pub fn init(logger: loggerfunc_t, plugin_handle: i32) {
    try_init(logger, plugin_handle).unwrap();
}

struct NorthstarLogger {
    logger: loggerfunc_t,
    plugin_handle: ::std::os::raw::c_int,
}

impl NorthstarLogger {
    fn init(logger: loggerfunc_t, plugin_handle: i32) -> Self {
        Self {
            logger,
            plugin_handle,
        }
    }
}

impl log::Log for NorthstarLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) || self.logger.is_none() {
            return;
        }

        // probably should be reworked to use less to_cstring, like who needs file, func and line context?

        let msg = to_cstring(record.args());
        let file = to_cstring(record.module_path().unwrap_or(" "));
        let func = to_cstring(record.file().unwrap_or(" "));
        let line = record.line().unwrap_or(0) as i32;

        let source = MessageSource {
            file: file.as_ptr(),
            func: func.as_ptr(),
            line,
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis();

        let mut logs = LogMsg {
            level: level_to_int(record.metadata().level()),
            timestamp: timestamp.try_into().unwrap_or(0), // lmao cpp logs use u64
            msg: msg.as_ptr(),
            source,
            pluginHandle: self.plugin_handle,
        };

        // if this is null then smth went very wrong
        unsafe { self.logger.unwrap()(&mut logs as *mut LogMsg) }
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
fn level_to_int(level: Level) -> i32 {
    match level {
        Level::Error => 4,
        Level::Warn => 3,
        Level::Info => 2,
        Level::Debug => 1,
        Level::Trace => 0,
    }
}
