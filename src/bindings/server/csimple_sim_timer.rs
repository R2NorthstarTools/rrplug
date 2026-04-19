#![allow(non_camel_case_types, non_snake_case)]
use crate::{field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CSimpleSimTimer {
    pub m_next: f32, // +0x0 size: 0x4 (0x1 * 0x4) type 16
}

size_assert!(CSIMPLESIMTIMER where CSimpleSimTimer == 0x4);
field_assert!(NEXT where CSimpleSimTimer, m_next == 0x0);
