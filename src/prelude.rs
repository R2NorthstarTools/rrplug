pub use crate::plugin::Plugin;
pub use crate::entry;
// pub use crate::bindings;
pub use crate::ffi::*;

pub fn wait(milliseconds: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milliseconds))
}