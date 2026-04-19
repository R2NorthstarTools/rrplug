#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::cstopwatch_base::CStopwatchBase, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CStopwatch {
    pub base: CStopwatchBase,
    pub m_interval: f32, // +0x8 size: 0x4 (0x1 * 0x4) type 1
}

size_assert!(CSTOPWATCH where CStopwatch == 0xc);
field_assert!(M_INTERVAL where CStopwatch, m_interval == 0x8);

impl DerefMut for CStopwatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CStopwatch {
    type Target = CStopwatchBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
