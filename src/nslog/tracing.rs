use once_cell::sync::OnceCell;
use std::{
    ffi::{CStr, CString},
    io::Write,
    panic,
};
use tracing::{error, Subscriber};
use tracing_subscriber::fmt::{format::Writer, MakeWriter};
use windows::Win32::Foundation::HMODULE;

use crate::{
    bindings::plugin_abi::LogLevel,
    high::UnsafeHandle,
    mid::northstar::{NorthstarSys, NORTHSTAR_DATA},
};

const C_STRING_ERROR: &CStr = c"rrplug logger failed to transform a String to a CString";

static LOGGER: OnceCell<NorthstarLogger> = OnceCell::new();

pub fn try_init(plugin_handle: HMODULE) -> Result<(), SetLoggerError> {
    panic::set_hook(Box::new(|info| {
        error!("");

        match info.location() {
            Some(location) => error!("plugin panicked at {}", location),
            None => error!("plugin panicked at unknown"),
        }

        error!("full message:");
        error!("{}", info);

        error!("");
    }));

    _ = LOGGER.set(NorthstarLogger::init(plugin_handle));

    tracing_subscriber::FmtSubscriber::builder().with_writer(make_writer)

    // log    tracing::set_logger(LOGGER.wait()).map(|()| log::set_max_level(LevelFilter::Info))
}

#[derive(Clone)]
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

// impl Subscriber for NorthstarLogger {
//     fn enabled(&self, metadata: &Metadata) -> bool {
//         metadata.level() <= Level::Info
//     }

//     fn log(&self, record: &Record) {
//         if !self.enabled(record.metadata()) || self.ns_sys.is_none() {
//             return;
//         }

//         let msg = to_cstring(record.args());

//         let Some(level) = level_to_log(record.metadata().level()) else {
//             return;
//         };

//         unsafe {
//             self.ns_sys.as_ref().unwrap_unchecked().get().log(
//                 self.plugin_handle,
//                 level,
//                 msg.as_ptr(),
//             )
//         }
//     }
// }

impl<'a> MakeWriter<'a> for NorthstarLogger {
    type Writer = &'a Self;

    fn make_writer(&'a self) -> Self::Writer {
        self
    }
}

impl<'a> Write for &'a NorthstarLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn to_cstring<T>(string: T) -> CString
where
    T: ToString,
{
    CString::new(string.to_string()).unwrap_or_else(|_| CString::from(C_STRING_ERROR))
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
