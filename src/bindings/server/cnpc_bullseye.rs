#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{cai_base_npc::CAI_BaseNPC, EHandle},
    field_assert, size_assert,
};

#[repr(C)]
#[derive(Debug)]
struct CNPC_Bullseye {
    base: CAI_BaseNPC,
    m_hPainPartner: EHandle, // +0x20b0 size: 0x4 (0x1 * 0x4) type 13
    gap_20b4: [u8; 4],
    m_OnTargeted: [u8; 40],          // +0x20b8 size: 0x28 (0x1 * 0x28) type 11
    m_OnReleased: [u8; 40],          // +0x20e0 size: 0x28 (0x1 * 0x28) type 11
    m_flMinDistValidEnemy: f32,      // +0x2108 size: 0x4 (0x1 * 0x4) type 1
    m_bPerfectAccuracy: bool,        // +0x210c size: 0x1 (0x1 * 0x1) type 6
    m_bAlwaysTransmitToClient: bool, // +0x210d size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(SIZE_Bullseye where CNPC_Bullseye == 0x2110);
field_assert!(WHERE_HPAINPARTNER where CNPC_Bullseye, m_hPainPartner == 0x20a8);
field_assert!(WHERE_ONTARGETED where CNPC_Bullseye, m_OnTargeted == 0x20b0);
field_assert!(WHERE_ONRELEASED where CNPC_Bullseye, m_OnReleased == 0x20d8);
field_assert!(WHERE_FLMINDISTVALIDENEMY where CNPC_Bullseye, m_flMinDistValidEnemy == 0x2100);
field_assert!(WHERE_BPERFECTACCURACY where CNPC_Bullseye, m_bPerfectAccuracy == 0x2104);
field_assert!(WHERE_BALWAYSTRANSMITTOCLIENT where CNPC_Bullseye, m_bAlwaysTransmitToClient == 0x2105);

impl DerefMut for CNPC_Bullseye {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CNPC_Bullseye {
    type Target = CAI_BaseNPC;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
