#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{
    bindings::server::{cbaseentity::CBaseEntity, csound::CSound},
    field_assert, size_assert,
};

#[repr(C)]
#[derive(Debug)]
pub struct CSoundEnt {
    pub base: CBaseEntity,
    pub m_iFreeSound: i32,          // +0x9e0 size: 0x4 (0x1 * 0x4) type 5
    pub m_iActiveSound: i32,        // +0x9e4 size: 0x4 (0x1 * 0x4) type 5
    pub m_SoundPool: [CSound; 128], // +0x9e8 size: 0x28 (0x80 * 0x0) type 10
}

size_assert!(CSOUNDENT where CSoundEnt == 0x1de8);
field_assert!(M_IFREESOUND where CSoundEnt, m_iFreeSound == 0x9d8);
field_assert!(M_IACTIVESOUND where CSoundEnt, m_iActiveSound == 0x9dc);
field_assert!(M_SOUNDPOOL where CSoundEnt, m_SoundPool == 0x9e0);

impl DerefMut for CSoundEnt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CSoundEnt {
    type Target = CBaseEntity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
