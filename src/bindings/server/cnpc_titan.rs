#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{cai_combatant::CAI_Combatant, cplayer::GrappleData, EHandle},
    field_assert,
    prelude::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CNPC_Titan {
    pub base: CAI_Combatant,
    pub m_outOfBoundsDeadTime: f32, // +0x2500 size: 0x4 (0x1 * 0x4) type 16
    pub m_bCanVortex: bool,         // +0x2504 size: 0x1 (0x1 * 0x1) type 6
    pub gap_2505: [u8; 3],
    pub m_defenseUseDamage: f32, // +0x2508 size: 0x4 (0x1 * 0x4) type 1
    pub m_defenseUseDamageRandom: f32, // +0x250c size: 0x4 (0x1 * 0x4) type 1
    pub m_lastVortexTime: f32,   // +0x2510 size: 0x4 (0x1 * 0x4) type 16
    pub m_vortexStartTime: f32,  // +0x2514 size: 0x4 (0x1 * 0x4) type 16
    pub m_vortexDuration: f32,   // +0x2518 size: 0x4 (0x1 * 0x4) type 1
    pub m_hLastDamager: EHandle, // +0x251c size: 0x4 (0x1 * 0x4) type 13
    pub m_defensiveAbility: i32, // +0x2520 size: 0x4 (0x1 * 0x4) type 5
    pub m_defensiveAbilityWeapon: EHandle, // +0x2524 size: 0x4 (0x1 * 0x4) type 13
    pub m_tacticalAbility: EHandle, // +0x2528 size: 0x4 (0x1 * 0x4) type 13
    pub m_titanCoreWeapon: EHandle, // +0x252c size: 0x4 (0x1 * 0x4) type 13
    pub m_meleeAttackCheckTarget: EHandle, // +0x2530 size: 0x4 (0x1 * 0x4) type 13
    pub m_blockingPhysEnt: EHandle, // +0x2534 size: 0x4 (0x1 * 0x4) type 13
    pub m_blockingPhysFaceDir: Vector3, // +0x2538 size: 0xc (0x1 * 0xc) type 3
    pub m_blockingPhysEntTime: f32, // +0x2544 size: 0x4 (0x1 * 0x4) type 16
    pub m_PhysEntKnockCount: i32, // +0x2548 size: 0x4 (0x1 * 0x4) type 5
    pub m_PhysEntKnockCountDecrTime: f32, // +0x254c size: 0x4 (0x1 * 0x4) type 16
    pub m_grappleHook: EHandle,  // +0x2550 size: 0x4 (0x1 * 0x4) type 13
    pub gap_2554: [u8; 4],
    pub m_grapple: GrappleData, // +0x2558 size: 0x68 (0x1 * 0x68) type 10
    pub m_grappleActive: bool,  // +0x25c0 size: 0x1 (0x1 * 0x1) type 6
    pub m_canStand: bool,       // +0x25c1 size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(CNPC_TITAN where CNPC_Titan == 0x25c8);
field_assert!(+ SIZE_OUTOFBOUNDSDEADTIME where CNPC_Titan, m_outOfBoundsDeadTime == 0x24f8);
field_assert!(+ SIZE_BCANVORTEX where CNPC_Titan, m_bCanVortex == 0x24fc);
field_assert!(+ SIZE_DEFENSEUSEDAMAGE where CNPC_Titan, m_defenseUseDamage == 0x2500);
field_assert!(+ SIZE_DEFENSEUSEDAMAGERANDOM where CNPC_Titan, m_defenseUseDamageRandom == 0x2504);
field_assert!(+ SIZE_LASTVORTEXTIME where CNPC_Titan, m_lastVortexTime == 0x2508);
field_assert!(+ SIZE_VORTEXSTARTTIME where CNPC_Titan, m_vortexStartTime == 0x250c);
field_assert!(+ SIZE_VORTEXDURATION where CNPC_Titan, m_vortexDuration == 0x2510);
field_assert!(+ SIZE_HLASTDAMAGER where CNPC_Titan, m_hLastDamager == 0x2514);
field_assert!(+ SIZE_DEFENSIVEABILITY where CNPC_Titan, m_defensiveAbility == 0x2518);
field_assert!(+ SIZE_DEFENSIVEABILITYWEAPON where CNPC_Titan, m_defensiveAbilityWeapon == 0x251c);
field_assert!(+ SIZE_TACTICALABILITY where CNPC_Titan, m_tacticalAbility == 0x2520);
field_assert!(+ SIZE_TITANCOREWEAPON where CNPC_Titan, m_titanCoreWeapon == 0x2524);
field_assert!(+ SIZE_MELEEATTACKCHECKTARGET where CNPC_Titan, m_meleeAttackCheckTarget == 0x2528);
field_assert!(+ SIZE_BLOCKINGPHYSENT where CNPC_Titan, m_blockingPhysEnt == 0x252c);
field_assert!(+ SIZE_BLOCKINGPHYSFACEDIR where CNPC_Titan, m_blockingPhysFaceDir == 0x2530);
field_assert!(+ SIZE_BLOCKINGPHYSENTTIME where CNPC_Titan, m_blockingPhysEntTime == 0x253c);
field_assert!(+ SIZE_PHYSENTKNOCKCOUNT where CNPC_Titan, m_PhysEntKnockCount == 0x2540);
field_assert!(+ SIZE_PHYSENTKNOCKCOUNTDECRTIME where CNPC_Titan, m_PhysEntKnockCountDecrTime == 0x2544);
field_assert!(+ SIZE_GRAPPLEHOOK where CNPC_Titan, m_grappleHook == 0x2548);
field_assert!(+ SIZE_GRAPPLE where CNPC_Titan, m_grapple == 0x2550);
field_assert!(+ SIZE_GRAPPLEACTIVE where CNPC_Titan, m_grappleActive == 0x25b8);
field_assert!(+ SIZE_CANSTAND where CNPC_Titan, m_canStand == 0x25b9);

impl DerefMut for CNPC_Titan {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CNPC_Titan {
    type Target = CAI_Combatant;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
