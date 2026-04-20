#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::c_void,
};

use crate::{bindings::server::cbaseentity::CBaseEntity, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_NetworkManager {
    pub base: CBaseEntity,
    pub m_pEditOps: *mut c_void, // +0x9e0 size: 0x8 (0x1 * 0x8) type 31
    pub m_pNetwork: *mut c_void, // +0x9e8 size: 0x8 (0x1 * 0x8) type 31
    pub m_fInitalized: bool,     // +0x9f0 size: 0x1 (0x1 * 0x1) type 6
    pub m_bDontSaveGraph: bool,  // +0x9f1 size: 0x1 (0x1 * 0x1) type 6
    pub gap_9f2: [u8; 2],
    pub m_ainVersion: i32,     // +0x9f4 size: 0x4 (0x1 * 0x4) type 5
    pub m_ainMapFilesCRC: i32, // +0x9f8 size: 0x4 (0x1 * 0x4) type 5
    pub m_runtimeCreatedAINMapFilesCRC: i32, // +0x9fc size: 0x4 (0x1 * 0x4) type 5
    pub m_calculatedRuntimeAINMapFilesCRC: bool, // +0xa00 size: 0x1 (0x1 * 0x1) type 6
    pub gap_a01: [u8; 7],
    pub m_ThreadedBuild: [u8; 72], // +0xa08 size: 0x48 (0x1 * 0x48) type 0
}

size_assert!(CAI_NETWORKMANAGER where CAI_NetworkManager == 0xa50);
field_assert!(+ M_PEDITOPS where CAI_NetworkManager, m_pEditOps == 0x9d8);
field_assert!(+ M_PNETWORK where CAI_NetworkManager, m_pNetwork == 0x9e0);
field_assert!(+ M_FINITALIZED where CAI_NetworkManager, m_fInitalized == 0x9e8);
field_assert!(+ M_BDONTSAVEGRAPH where CAI_NetworkManager, m_bDontSaveGraph == 0x9e9);
field_assert!(+ M_AINVERSION where CAI_NetworkManager, m_ainVersion == 0x9ec);
field_assert!(+ M_AINMAPFILESCRC where CAI_NetworkManager, m_ainMapFilesCRC == 0x9f0);
field_assert!(+ M_RUNTIMECREATEDAINMAPFILESCRC where CAI_NetworkManager, m_runtimeCreatedAINMapFilesCRC == 0x9f4);
field_assert!(+ M_CALCULATEDRUNTIMEAINMAPFILESCRC where CAI_NetworkManager, m_calculatedRuntimeAINMapFilesCRC == 0x9f8);
field_assert!(+ M_THREADEDBUILD where CAI_NetworkManager, m_ThreadedBuild == 0xa00);

impl DerefMut for CAI_NetworkManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_NetworkManager {
    type Target = CBaseEntity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
