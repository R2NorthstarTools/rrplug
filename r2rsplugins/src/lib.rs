#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[doc(hidden)]
pub use spin::once;

pub mod bindings; // (crate)
pub mod macros;
pub mod ffi;
pub mod plugin;
pub mod prelude;
