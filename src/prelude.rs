pub use crate::plugin::Plugin;
pub use crate::entry;
pub use log;

/// puts a thread on sleep in milliseconds
pub fn wait(milliseconds: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milliseconds))
}

/// lock a thread forever
/// 
/// can be used to prevent the plugin callbacks from droping after the main function has ended.
pub fn wait_forever() {
    std::thread::sleep(std::time::Duration::MAX)
}