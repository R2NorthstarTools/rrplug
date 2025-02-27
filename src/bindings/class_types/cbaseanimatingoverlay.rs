#![allow(non_camel_case_types, non_snake_case)]

use std::{
    ffi::c_char,
    ops::{Deref, DerefMut},
};

use super::cbaseanimating::CBaseAnimating;
use crate::size_assert;

type CBaseEntity = *mut u8;

#[repr(C)]
pub struct CAnimationLayer {
    m_bSequenceFinished: bool, // 0x0 ( Size = 1 )
    gap_1: [c_char; 3],
    m_fFlags: i32,           // 0x4 ( Size = 4 )
    m_layerIndex: i32,       // 0x8 ( Size = 4 )
    m_modelIndex: i32,       // 0xc ( Size = 4 )
    m_flKillRate: f32,       // 0x10 ( Size = 4 )
    m_flKillDelay: f32,      // 0x14 ( Size = 4 )
    m_nActivity: i32,        // 0x18 ( Size = 4 )
    m_nPriority: i32,        // 0x1c ( Size = 4 )
    m_flLastEventCheck: f32, // 0x20 ( Size = 4 )
    gap_24: [c_char; 4],
    m_animationLayerOwner: *mut CBaseEntity, // 0x28 ( Size = 8 )
}
size_assert!(SIZE_ANIMATION_LAYER where CAnimationLayer == 48);

#[repr(C)]
pub struct CBaseAnimatingOverlay {
    base: CBaseAnimating,
    m_maxOverlays: i32, // 0xeb8 ( Size = 4 )
    gap_ebc: [c_char; 4],
    m_AnimOverlay: [CAnimationLayer; 8],    // 0xec0 ( Size = 48 )
    m_AnimOverlayCount: i32,                // 0x1040 ( Size = 4 )
    m_animOverlayIsActive: [bool; 8],       // 0x1044 ( Size = 8 )
    m_animOverlayModelIndex: [i32; 8],      // 0x104c ( Size = 32 )
    m_animOverlaySequence: [i32; 8],        // 0x106c ( Size = 32 )
    m_animOverlayCycle: [f32; 8],           // 0x108c ( Size = 32 )
    m_animOverlayStartTime: [f32; 8],       // 0x10ac ( Size = 32 )
    m_animOverlayStartCycle: [f32; 8],      // 0x10cc ( Size = 32 )
    m_animOverlayPlaybackRate: [f32; 8],    // 0x10ec ( Size = 32 )
    m_animOverlayWeight: [f32; 8],          // 0x110c ( Size = 32 )
    m_animOverlayOrder: [i32; 8],           // 0x112c ( Size = 32 )
    m_animOverlayAnimTime: [f32; 8],        // 0x114c ( Size = 32 )
    m_animOverlayFadeInDuration: [f32; 8],  // 0x116c ( Size = 32 )
    m_animOverlayFadeOutDuration: [f32; 8], // 0x118c ( Size = 32 )
    m_localAnimOverlayIsActive: [bool; 4],  // 0x11ac ( Size = 4 )
    m_localAnimOverlayModelIndex: [i32; 4], // 0x11b0 ( Size = 16 )
    m_localAnimOverlaySequence: [i32; 4],   // 0x11c0 ( Size = 16 )
    m_localAnimOverlayStartTime: [f32; 4],  // 0x11d0 ( Size = 16 )
    m_localAnimOverlayWeight: [f32; 4],     // 0x11e0 ( Size = 16 )
    m_localAnimOverlayFadeInDuration: [f32; 4], // 0x11f0 ( Size = 16 )
    m_localAnimOverlayFadeOutDuration: [f32; 4], // 0x1200 ( Size = 16 )
}
size_assert!(SIZE_AIMATING_OVERLAY where CBaseAnimatingOverlay == 0x1210);

impl DerefMut for CBaseAnimatingOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CBaseAnimatingOverlay {
    type Target = CBaseAnimating;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
