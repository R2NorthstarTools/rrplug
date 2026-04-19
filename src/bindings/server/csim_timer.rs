#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::csimple_sim_timer::CSimpleSimTimer, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CSimTimer {
    pub base: CSimpleSimTimer,
    pub m_interval: f32, // +0x4 size: 0x4 (0x1 * 0x4) type 1
}

size_assert!(CSIMTIMER where CSimTimer == 0x8);
field_assert!(M_INTERVAL where CSimTimer, m_interval == 0x4);

impl DerefMut for CSimTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CSimTimer {
    type Target = CSimpleSimTimer;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
