#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::csimple_sim_timer::CSimpleSimTimer, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CStopwatchBase {
    pub base: CSimpleSimTimer,
    pub m_fIsRunning: bool, // +0x4 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(CSTOPWATCHBASE where CStopwatchBase == 0x8);
field_assert!(M_FISRUNNING where CStopwatchBase, m_fIsRunning == 0x4);

impl DerefMut for CStopwatchBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CStopwatchBase {
    type Target = CSimpleSimTimer;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
