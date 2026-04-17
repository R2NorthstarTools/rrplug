#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use super::{cbaseanimating::CBaseAnimating, cplayer::EHandle};

use crate::size_assert;

#[repr(C)]
pub struct CBaseCombatWeapon {
    pub base: CBaseAnimating,
    pub m_weaponOwner: EHandle,         // +0xeb8 size: 0x4 (0x1 * 0x4) 13
    pub m_weaponOwnerPrevious: EHandle, // +0xebc size: 0x4 (0x1 * 0x4) 13
    pub m_weaponOwnerPreviousWasNPC: bool, // +0xec0 size: 0x1 (0x1 * 0x1) 6
    pub gap_ec1: [u8; 3],
    pub m_nextReadyTime: f32,         // +0xec4 size: 0x4 (0x1 * 0x4) 16
    pub m_nextPrimaryAttackTime: f32, // +0xec8 size: 0x4 (0x1 * 0x4) 16
    pub m_iWorldModelIndex: i32,      // +0xecc size: 0x4 (0x1 * 0x4) 5
    pub m_holsterModelIndex: i32,     // +0xed0 size: 0x4 (0x1 * 0x4) 5
    pub m_droppedModelIndex: i32,     // +0xed4 size: 0x4 (0x1 * 0x4) 5
    pub m_ActiveState: i32,           // +0xed8 size: 0x4 (0x1 * 0x4) 5
    pub m_ammoInClip: i32,            // +0xedc size: 0x4 (0x1 * 0x4) 5
    pub m_ammoInStockpile: i32,       // +0xee0 size: 0x4 (0x1 * 0x4) 5
    pub m_lifetimeShots: i32,         // +0xee4 size: 0x4 (0x1 * 0x4) 5
    pub m_flTimeWeaponIdle: f32,      // +0xee8 size: 0x4 (0x1 * 0x4) 16
    pub m_physicsAttacker: EHandle,   // +0xeec size: 0x4 (0x1 * 0x4) 13
    pub m_projectileModelIndex: i32,  // +0xef0 size: 0x4 (0x1 * 0x4) 5
    pub m_lastPrimaryAttack: f32,     // +0xef4 size: 0x4 (0x1 * 0x4) 16
    pub m_flNextEmptySoundTime: f32,  // +0xef8 size: 0x4 (0x1 * 0x4) 16
    pub m_weaponActivity: i32,        // +0xefc size: 0x4 (0x1 * 0x4) 5
    pub m_bRemoveable: bool,          // +0xf00 size: 0x1 (0x1 * 0x1) 6
    pub m_bInReload: bool,            // +0xf01 size: 0x1 (0x1 * 0x1) 6
    pub gap_f02: [u8; 2],
    pub m_nIdealSequence: i32,        // +0xf04 size: 0x4 (0x1 * 0x4) 5
    pub m_IdealActivity: i32,         // +0xf08 size: 0x4 (0x1 * 0x4) 5
    pub m_ownerMuzzleAttachment: i32, // +0xf0c size: 0x4 (0x1 * 0x4) 5
    pub m_weaponInfoFileHandle: i16,  // +0xf10 size: 0x2 (0x1 * 0x2) 7
    pub gap_f12: [u8; 6],
    pub m_pConstraint: [u8; 8],         // +0xf18 size: 0x8 (0x1 * 0x8) 11
    pub m_OnPlayerUse: [u8; 40],        // +0xf20 size: 0x28 (0x1 * 0x28) 11
    pub m_OnPlayerPickup: [u8; 40],     // +0xf48 size: 0x28 (0x1 * 0x28) 11
    pub m_OnNPCPickup: [u8; 40],        // +0xf70 size: 0x28 (0x1 * 0x28) 11
    pub m_OnCacheInteraction: [u8; 40], // +0xf98 size: 0x28 (0x1 * 0x28) 11
}

size_assert!(SIZE_WEAPON where CBaseCombatWeapon == 0xfc0);

impl DerefMut for CBaseCombatWeapon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CBaseCombatWeapon {
    type Target = CBaseAnimating;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
