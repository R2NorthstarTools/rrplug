#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ops::{Deref, DerefMut},
    os::raw::c_void,
};

use crate::{bindings::server::cai_component::CAI_Component, field_assert, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_Pathfinder {
    pub base: CAI_Component,
    pub m_TriDebugOverlay: *mut c_void, // +0x10 size: 0x8 (0x1 * 0x8) type 0
    pub m_flLimitDistFactor: f32,       // +0x18 size: 0x4 (0x1 * 0x4) type 1
    pub m_flLastStaleLinkCheckTime: f32, // +0x1c size: 0x4 (0x1 * 0x4) type 16
    pub m_bIgnoreStaleLinks: [u8; 1],   // +0x20 size: 0x1 (0x1 * 0x1) type 0
    pub gap_21: [u8; 7],
    pub m_pNetwork: *mut c_void, // +0x28 size: 0x8 (0x1 * 0x8) type 31
    pub m_navQuery: [u8; 96],    // +0x30 size: 0x60 (0x1 * 0x60) type 0
    pub m_navFilter: [u8; 136],  // +0x90 size: 0x88 (0x1 * 0x88) type 0
    pub m_useClusterExclusions: bool, // +0x118 size: 0x1 (0x1 * 0x1) type 6
    pub gap_119: [u8; 3],
    pub m_clusterPathMaxDetourBase: f32, // +0x11c size: 0x4 (0x1 * 0x4) type 1
    pub m_clusterPathMaxDetourMultiplier: f32, // +0x120 size: 0x4 (0x1 * 0x4) type 1
    pub gap_124: [u8; 4],
    pub m_excludedClusterNodes: [u8; 32], // +0x128 size: 0x20 (0x1 * 0x20) type 11
    pub m_excludedClusterLinks: [u8; 32], // +0x148 size: 0x20 (0x1 * 0x20) type 11
    pub m_pClusterPath: *mut c_void,      // +0x168 size: 0x8 (0x1 * 0x8) type 31
    pub m_pClusterNoExclusionPath: *mut c_void, // +0x170 size: 0x8 (0x1 * 0x8) type 31
    pub m_buildFlags: i32,                // +0x178 size: 0x4 (0x1 * 0x4) type 5
    pub m_failReason: i32,                // +0x17c size: 0x4 (0x1 * 0x4) type 5
}

size_assert!(CAI_PATHFINDER where CAI_Pathfinder == 0x180);
field_assert!(+ M_TRIDEBUGOVERLAY where CAI_Pathfinder, m_TriDebugOverlay == 0x8);
field_assert!(+ M_FLLIMITDISTFACTOR where CAI_Pathfinder, m_flLimitDistFactor == 0x10);
field_assert!(+ M_FLLASTSTALELINKCHECKTIME where CAI_Pathfinder, m_flLastStaleLinkCheckTime == 0x14);
field_assert!(+ M_BIGNORESTALELINKS where CAI_Pathfinder, m_bIgnoreStaleLinks == 0x18);
field_assert!(+ M_PNETWORK where CAI_Pathfinder, m_pNetwork == 0x20);
field_assert!(+ M_NAVQUERY where CAI_Pathfinder, m_navQuery == 0x28);
field_assert!(+ M_NAVFILTER where CAI_Pathfinder, m_navFilter == 0x88);
field_assert!(+ M_USECLUSTEREXCLUSIONS where CAI_Pathfinder, m_useClusterExclusions == 0x110);
field_assert!(+ M_CLUSTERPATHMAXDETOURBASE where CAI_Pathfinder, m_clusterPathMaxDetourBase == 0x114);
field_assert!(+ M_CLUSTERPATHMAXDETOURMULTIPLIER where CAI_Pathfinder, m_clusterPathMaxDetourMultiplier == 0x118);
field_assert!(+ M_EXCLUDEDCLUSTERNODES where CAI_Pathfinder, m_excludedClusterNodes == 0x120);
field_assert!(+ M_EXCLUDEDCLUSTERLINKS where CAI_Pathfinder, m_excludedClusterLinks == 0x140);
field_assert!(+ M_PCLUSTERPATH where CAI_Pathfinder, m_pClusterPath == 0x160);
field_assert!(+ M_PCLUSTERNOEXCLUSIONPATH where CAI_Pathfinder, m_pClusterNoExclusionPath == 0x168);
field_assert!(+ M_BUILDFLAGS where CAI_Pathfinder, m_buildFlags == 0x170);
field_assert!(+ M_FAILREASON where CAI_Pathfinder, m_failReason == 0x174);

impl DerefMut for CAI_Pathfinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_Pathfinder {
    type Target = CAI_Component;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
