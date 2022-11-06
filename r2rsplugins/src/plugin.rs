use crate::bindings::*;
use std::os::raw::c_void;

pub trait Plugin {
    fn new() -> Self;

    fn initialize(&self, getPluginData_external: c_void);

    fn main(&self);
}
