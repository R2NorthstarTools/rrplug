#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{cai_component::CAI_Component, cai_move_probe::CAI_MoveProbe},
    field_assert,
    prelude::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct dtStraightPathResult {
    pathCount: i32,      // +0x0 size: 0x4 (0x1 * 0x4) type 5
    path: [Vector3; 5],  // +0x4 size: 0x3c (0x5 * 0xc) type 3
    polys: [i32; 5],     // +0x40 size: 0x14 (0x5 * 0x4) type 5
    jumps: [i8; 5],      // +0x54 size: 0x5 (0x5 * 0x1) type 8
    flags: [i8; 5],      // +0x59 size: 0x5 (0x5 * 0x1) type 8
    pathEndIsGoal: bool, // +0x5e size: 0x1 (0x1 * 0x1) type 6
}

size_assert!(DTSTRAIGHTPATHRESULT where dtStraightPathResult == 0x60);
field_assert!(PATHCOUNT where dtStraightPathResult, pathCount == 0x0);
field_assert!(PATH where dtStraightPathResult, path == 0x4);
field_assert!(POLYS where dtStraightPathResult, polys == 0x40);
field_assert!(JUMPS where dtStraightPathResult, jumps == 0x54);
field_assert!(FLAGS where dtStraightPathResult, flags == 0x59);
field_assert!(PATHENDISGOAL where dtStraightPathResult, pathEndIsGoal == 0x5e);

#[repr(C)]
#[derive(Debug)]
pub struct CAI_Motor {
    pub base: CAI_Component,
    pub CAI_ProxyMovementSink: [u8; 16], // +0x10 size: 0x10 (0x0 * 0x10) type 0
    pub m_steerDirection: Vector3,       // +0x20 size: 0xc (0x1 * 0xc) type 3
    pub m_straightPathCached: dtStraightPathResult, // +0x2c size: 0x60 (0x1 * 0x60) type 10
    pub m_pushedVel: Vector3,            // +0x8c size: 0xc (0x1 * 0xc) type 3
    pub m_flMoveInterval: f32,           // +0x98 size: 0x4 (0x1 * 0x4) type 1
    pub m_IdealYaw: f32,                 // +0x9c size: 0x4 (0x1 * 0x4) type 1
    pub m_fMoveYaw: f32,                 // +0xa0 size: 0x4 (0x1 * 0x4) type 1
    pub m_vecVelocity: Vector3,          // +0xa4 size: 0xc (0x1 * 0xc) type 3
    pub m_vecAngularVelocity: Vector3,   // +0xb0 size: 0xc (0x1 * 0xc) type 3
    pub m_bMoving: bool,                 // +0xbc size: 0x1 (0x1 * 0x1) type 6
    pub gap_bd: [u8; 3],
    pub m_moveSpeedScale: f32,       // +0xc0 size: 0x4 (0x1 * 0x4) type 1
    pub m_moveSpeedScaleScript: f32, // +0xc4 size: 0x4 (0x1 * 0x4) type 1
    pub m_nDismountSequence: i32,    // +0xc8 size: 0x4 (0x1 * 0x4) type 5
    pub m_vecDismount: Vector3,      // +0xcc size: 0xc (0x1 * 0xc) type 3
    pub m_facingQueue: [u8; 32],     // +0xd8 size: 0x20 (0x1 * 0x20) type 11
    pub m_pMoveProbe: *mut CAI_MoveProbe, // +0xf8 size: 0x8 (0x1 * 0x8) type 10
}

size_assert!(CAI_MOTOR where CAI_Motor == 0x100);
field_assert!(CAI_PROXYMOVEMENTSINK where CAI_Motor, CAI_ProxyMovementSink == 0x8);
field_assert!(STEERDIRECTION where CAI_Motor, m_steerDirection == 0x18);
field_assert!(STRAIGHTPATHCACHED where CAI_Motor, m_straightPathCached == 0x24);
field_assert!(PUSHEDVEL where CAI_Motor, m_pushedVel == 0x84);
field_assert!(FLMOVEINTERVAL where CAI_Motor, m_flMoveInterval == 0x90);
field_assert!(IDEALYAW where CAI_Motor, m_IdealYaw == 0x94);
field_assert!(FMOVEYAW where CAI_Motor, m_fMoveYaw == 0x98);
field_assert!(VECVELOCITY where CAI_Motor, m_vecVelocity == 0x9c);
field_assert!(VECANGULARVELOCITY where CAI_Motor, m_vecAngularVelocity == 0xa8);
field_assert!(BMOVING where CAI_Motor, m_bMoving == 0xb4);
field_assert!(MOVESPEEDSCALE where CAI_Motor, m_moveSpeedScale == 0xb8);
field_assert!(MOVESPEEDSCALESCRIPT where CAI_Motor, m_moveSpeedScaleScript == 0xbc);
field_assert!(NDISMOUNTSEQUENCE where CAI_Motor, m_nDismountSequence == 0xc0);
field_assert!(VECDISMOUNT where CAI_Motor, m_vecDismount == 0xc4);
field_assert!(FACINGQUEUE where CAI_Motor, m_facingQueue == 0xd0);
field_assert!(PMOVEPROBE where CAI_Motor, m_pMoveProbe == 0xf0);

impl DerefMut for CAI_Motor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_Motor {
    type Target = CAI_Component;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
