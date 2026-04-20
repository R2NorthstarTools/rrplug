#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::cai_track_pather::CAI_TrackPather, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CBaseHelicopterBehaviorHost {
    pub base: CAI_TrackPather,
    pub m_bCalledBehaviorSelectSchedule: bool, // +0x2190 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(A where CBaseHelicopterBehaviorHost == 0x2198);
field_assert!(+ B where CBaseHelicopterBehaviorHost, m_bCalledBehaviorSelectSchedule == 0x2188);

impl DerefMut for CBaseHelicopterBehaviorHost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CBaseHelicopterBehaviorHost {
    type Target = CAI_TrackPather;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
