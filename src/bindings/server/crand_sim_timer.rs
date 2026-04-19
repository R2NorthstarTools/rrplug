#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::csimple_sim_timer::CSimpleSimTimer, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CRandSimTimer {
    pub base: CSimpleSimTimer,
    pub m_minInterval: f32, // +0x4 size: 0x4 (0x1 * 0x4) type 1
    pub m_maxInterval: f32, // +0x8 size: 0x4 (0x1 * 0x4) type 1
}

size_assert!(CRANDSIMTIMER where CRandSimTimer == 0xc);
field_assert!(M_MININTERVAL where CRandSimTimer, m_minInterval == 0x4);
field_assert!(M_MAXINTERVAL where CRandSimTimer, m_maxInterval == 0x8);

impl DerefMut for CRandSimTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CRandSimTimer {
    type Target = CSimpleSimTimer;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
