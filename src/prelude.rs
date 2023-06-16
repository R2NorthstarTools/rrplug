pub use crate::entry;
pub use crate::plugin::Plugin;
pub use crate::wrappers::northstar::PluginData;
pub use crate::wrappers::squirrel::{SQFUNCTIONS,Save,NoSave};
pub use log;

/// puts a thread on sleep in milliseconds
pub fn wait(milliseconds: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milliseconds))
}
