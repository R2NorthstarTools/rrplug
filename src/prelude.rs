pub use crate::wrappers::squrriel::SQFUNCTIONS;
pub use crate::plugin::Plugin;
pub use crate::entry;
pub use log;

/// puts a thread on sleep in milliseconds
pub fn wait(milliseconds: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milliseconds))
}
