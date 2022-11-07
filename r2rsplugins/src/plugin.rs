use crate::bindings::*;
use std::ffi::c_void;

pub trait Plugin {
    fn new() -> Self;

    fn initialize(&mut self, get_plugin_data_external: &c_void);

    fn main(&self);
}
