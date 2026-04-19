#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::cstopwatch_base::CStopwatchBase, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CRandStopwatch {
    pub base: CStopwatchBase,
    pub m_minInterval: f32, // +0x8 size: 0x4 (0x1 * 0x4) type 1
    pub m_maxInterval: f32, // +0xc size: 0x4 (0x1 * 0x4) type 1
}

size_assert!(CRandStopwatch where CRandStopwatch == 0x10);
field_assert!(m_minInterval where CRandStopwatch, m_minInterval == 0x8);
field_assert!(m_maxInterval where CRandStopwatch, m_maxInterval == 0xc);

impl DerefMut for CRandStopwatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CRandStopwatch {
    type Target = CStopwatchBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
