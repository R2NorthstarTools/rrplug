#![allow(non_camel_case_types, non_snake_case)]
use std::os::raw::c_void;

use crate::{field_assert, size_assert};

// TODO: figure out if this correct (putting the vtable here)
#[repr(C)]
#[derive(Debug)]
pub struct CAI_Component {
    pub vtable: *const c_void,
    pub m_pOuter: *mut c_void, // +0x8 size: 0x8 (0x1 * 0x8) type 31
}

size_assert!(A where CAI_Component == 0x10);
field_assert!(B where CAI_Component, m_pOuter == 0x0);
