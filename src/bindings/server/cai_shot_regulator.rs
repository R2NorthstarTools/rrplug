#![allow(non_camel_case_types, non_snake_case)]
use std::os::raw::c_void;

use crate::{field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_ShotRegulator {
    pub m_nBurstShotsRemaining: i32, // +0x0 size: 0x4 (0x1 * 0x4) type 5
    pub m_nMinBurstShots: i32,       // +0x4 size: 0x4 (0x1 * 0x4) type 5
    pub m_nMaxBurstShots: i32,       // +0x8 size: 0x4 (0x1 * 0x4) type 5
    pub m_flNextShotTime: f32,       // +0xc size: 0x4 (0x1 * 0x4) type 16
    pub m_flFireRateDelay: f32,      // +0x10 size: 0x4 (0x1 * 0x4) type 1
    pub m_flMinRestInterval: f32,    // +0x14 size: 0x4 (0x1 * 0x4) type 1
    pub m_flMaxRestInterval: f32,    // +0x18 size: 0x4 (0x1 * 0x4) type 1
    pub m_flLastPreFireDelayTime: f32, // +0x1c size: 0x4 (0x1 * 0x4) type 16
    pub m_bInRestInterval: bool,     // +0x20 size: 0x1 (0x1 * 0x1) type 6
    pub m_bFirstShot: bool,          // +0x21 size: 0x1 (0x1 * 0x1) type 6
    pub gap_22: [u8; 6],
    pub m_ai: *mut c_void, // +0x28 size: 0x8 (0x1 * 0x8) type 31
}

size_assert!(CAI_SHOTREGULATOR where CAI_ShotRegulator == 0x30);
field_assert!(M_NBURSTSHOTSREMAINING where CAI_ShotRegulator, m_nBurstShotsRemaining == 0x0);
field_assert!(M_NMINBURSTSHOTS where CAI_ShotRegulator, m_nMinBurstShots == 0x4);
field_assert!(M_NMAXBURSTSHOTS where CAI_ShotRegulator, m_nMaxBurstShots == 0x8);
field_assert!(M_FLNEXTSHOTTIME where CAI_ShotRegulator, m_flNextShotTime == 0xc);
field_assert!(M_FLFIRERATEDELAY where CAI_ShotRegulator, m_flFireRateDelay == 0x10);
field_assert!(M_FLMINRESTINTERVAL where CAI_ShotRegulator, m_flMinRestInterval == 0x14);
field_assert!(M_FLMAXRESTINTERVAL where CAI_ShotRegulator, m_flMaxRestInterval == 0x18);
field_assert!(M_FLLASTPREFIREDELAYTIME where CAI_ShotRegulator, m_flLastPreFireDelayTime == 0x1c);
field_assert!(M_BINRESTINTERVAL where CAI_ShotRegulator, m_bInRestInterval == 0x20);
field_assert!(M_BFIRSTSHOT where CAI_ShotRegulator, m_bFirstShot == 0x21);
field_assert!(M_AI where CAI_ShotRegulator, m_ai == 0x28);
