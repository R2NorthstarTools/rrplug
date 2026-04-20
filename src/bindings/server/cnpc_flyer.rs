#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::cai_baseflyingbot::CAI_BaseFlyingBot, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CNPC_Flyer {
    pub base: CAI_BaseFlyingBot,
    pub m_onGround: bool, // +0x2458 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(CNPC_FLYER where CNPC_Flyer == 0x2460);
field_assert!(+ ONGROUND where CNPC_Flyer, m_onGround == 0x2450);

impl DerefMut for CNPC_Flyer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CNPC_Flyer {
    type Target = CAI_BaseFlyingBot;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
