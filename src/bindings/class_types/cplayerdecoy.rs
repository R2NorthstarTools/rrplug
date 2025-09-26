#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use super::cplayer::EHandle;

use crate::{
    bindings::class_types::cbaseanimating::CBaseAnimating, field_assert, prelude::*, size_assert,
};

#[repr(C)]
struct CPlayerDecoy {
    base: CBaseAnimating,
    m_currentState: i32,        // +0xeb8 size: 0x4 (0x1 * 0x4) type 5
    m_decoyFlags: i32,          // +0xebc size: 0x4 (0x1 * 0x4) type 5
    m_lastPulseTime: f32,       // +0xec0 size: 0x4 (0x1 * 0x4) type 16
    m_previousState: i32,       // +0xec4 size: 0x4 (0x1 * 0x4) type 5
    m_deathTime: f32,           // +0xec8 size: 0x4 (0x1 * 0x4) type 16
    m_removeTime: f32,          // +0xecc size: 0x4 (0x1 * 0x4) type 16
    m_npcSoundTime: f32,        // +0xed0 size: 0x4 (0x1 * 0x4) type 16
    m_npcSoundIndex: i32,       // +0xed4 size: 0x4 (0x1 * 0x4) type 5
    m_fakeWeapon: EHandle,      // +0xed8 size: 0x4 (0x1 * 0x4) type 13
    m_stowedWeapon: EHandle,    // +0xedc size: 0x4 (0x1 * 0x4) type 13
    m_runSpeed: f32,            // +0xee0 size: 0x4 (0x1 * 0x4) type 1
    m_sprintSpeed: f32,         // +0xee4 size: 0x4 (0x1 * 0x4) type 1
    m_jumpHeight: f32,          // +0xee8 size: 0x4 (0x1 * 0x4) type 1
    m_wallrunJumpStrength: f32, // +0xeec size: 0x4 (0x1 * 0x4) type 1
    m_jumpTime: f32,            // +0xef0 size: 0x4 (0x1 * 0x4) type 16
    m_nextFlickerTime: f32,     // +0xef4 size: 0x4 (0x1 * 0x4) type 16
    m_flickerRate: f32,         // +0xef8 size: 0x4 (0x1 * 0x4) type 1
    m_curSpeed: f32,            // +0xefc size: 0x4 (0x1 * 0x4) type 1
    m_slideEndTime: f32,        // +0xf00 size: 0x4 (0x1 * 0x4) type 16
    m_animEndTime: f32,         // +0xf04 size: 0x4 (0x1 * 0x4) type 16
    m_decoyHeight: f32,         // +0xf08 size: 0x4 (0x1 * 0x4) type 1
    m_pulseRate: f32,           // +0xf0c size: 0x4 (0x1 * 0x4) type 1
    m_nextPulseTime: f32,       // +0xf10 size: 0x4 (0x1 * 0x4) type 16
    m_upDir: Vector3,           // +0xf14 size: 0xc (0x1 * 0xc) type 3
    modifiers: [u8; 64],        // +0xf20 size: 0x40 (0x1a * 0x2) type 31
}

size_assert!(CPLAYERDECOY_DATA where CPlayerDecoy == 0xf60);
field_assert!(CURRENTSTATE_DATA where CPlayerDecoy, m_currentState == 0xeb0);
field_assert!(DECOYFLAGS_DATA where CPlayerDecoy, m_decoyFlags == 0xeb4);
field_assert!(LASTPULSETIME_DATA where CPlayerDecoy, m_lastPulseTime == 0xeb8);
field_assert!(PREVIOUSSTATE_DATA where CPlayerDecoy, m_previousState == 0xebc);
field_assert!(DEATHTIME_DATA where CPlayerDecoy, m_deathTime == 0xec0);
field_assert!(REMOVETIME_DATA where CPlayerDecoy, m_removeTime == 0xec4);
field_assert!(NPCSOUNDTIME_DATA where CPlayerDecoy, m_npcSoundTime == 0xec8);
field_assert!(NPCSOUNDINDEX_DATA where CPlayerDecoy, m_npcSoundIndex == 0xecc);
field_assert!(FAKEWEAPON_DATA where CPlayerDecoy, m_fakeWeapon == 0xed0);
field_assert!(STOWEDWEAPON_DATA where CPlayerDecoy, m_stowedWeapon == 0xed4);
field_assert!(RUNSPEED_DATA where CPlayerDecoy, m_runSpeed == 0xed8);
field_assert!(SPRINTSPEED_DATA where CPlayerDecoy, m_sprintSpeed == 0xedc);
field_assert!(JUMPHEIGHT_DATA where CPlayerDecoy, m_jumpHeight == 0xee0);
field_assert!(WALLRUNJUMPSTRENGTH_DATA where CPlayerDecoy, m_wallrunJumpStrength == 0xee4);
field_assert!(JUMPTIME_DATA where CPlayerDecoy, m_jumpTime == 0xee8);
field_assert!(NEXTFLICKERTIME_DATA where CPlayerDecoy, m_nextFlickerTime == 0xeec);
field_assert!(FLICKERRATE_DATA where CPlayerDecoy, m_flickerRate == 0xef0);
field_assert!(CURSPEED_DATA where CPlayerDecoy, m_curSpeed == 0xef4);
field_assert!(SLIDEENDTIME_DATA where CPlayerDecoy, m_slideEndTime == 0xef8);
field_assert!(ANIMENDTIME_DATA where CPlayerDecoy, m_animEndTime == 0xefc);
field_assert!(DECOYHEIGHT_DATA where CPlayerDecoy, m_decoyHeight == 0xf00);
field_assert!(PULSERATE_DATA where CPlayerDecoy, m_pulseRate == 0xf04);
field_assert!(NEXTPULSETIME_DATA where CPlayerDecoy, m_nextPulseTime == 0xf08);
field_assert!(UPDIR_DATA where CPlayerDecoy, m_upDir == 0xf0c);
field_assert!(MODIFIERS_DATA where CPlayerDecoy, modifiers == 0xf18);

impl DerefMut for CPlayerDecoy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CPlayerDecoy {
    type Target = CBaseAnimating;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
