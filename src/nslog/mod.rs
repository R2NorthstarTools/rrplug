use std::ffi::CString;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::bindings::plugin_abi::{loggerfunc_t, LogMsg, MessageSource};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

static mut LOGGER: NorthstarLogger = NorthstarLogger {
    logger: None,
    plugin_handle: 0,
};

pub fn try_init(logger: loggerfunc_t, plugin_handle: i32) -> Result<(), SetLoggerError> {
    unsafe {
        LOGGER = NorthstarLogger::init(logger, plugin_handle);

        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
    }
}

pub fn init( logger: loggerfunc_t, plugin_handle: i32 ) {
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

        let msg = to_cstring(record.args().as_str());
        let file = to_cstring(record.module_path());
        let func = to_cstring(record.file());
        let line = record.line().unwrap_or(0) as i32;

        let source = MessageSource {
            file: file.as_ptr(),
            func: func.as_ptr(),
            line,
        };

        let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

        let mut logs = LogMsg {
            level: level_to_int( record.metadata().level() ),
            timestamp: timestamp.try_into().unwrap_or(0),
            msg: msg.as_ptr(),
            source,
            pluginHandle: self.plugin_handle,
        };

        unsafe { self.logger.unwrap()(&mut logs as *mut LogMsg) }
    }

    fn flush(&self) {}
}

fn to_cstring(string: Option<&str>) -> CString {
    CString::new(string.unwrap_or(" ")).unwrap_or_else(|_| CString::new(" ").unwrap())
}

fn level_to_int( level: Level ) -> i32 {
    match level {
        Level::Error => 4,
        Level::Warn => 3,
        Level::Info => 2,
        Level::Debug => 1,
        Level::Trace => 0,
    }
}