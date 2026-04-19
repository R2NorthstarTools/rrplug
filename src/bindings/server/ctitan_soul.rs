#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use super::{cbaseentity::CBaseEntity, EHandle};
use crate::{
    bindings::server::cplayer::{StatusEffectEndlessData, StatusEffectTimedData},
    field_assert,
    prelude::Vector3,
    size_assert,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DefensivePlacement {
    pub size: f32,
    pub endTime: f32,
    pub effectiveAngleCos: f32,
    pub blocksProjectile: bool,
    pub attached: bool,
    pub handlePlacement: bool,
    pub gap_f: [u8; 1usize],
    pub pos: Vector3,
    pub dir: Vector3,
}

#[repr(C)]
#[derive(Debug)]
pub struct CTitanSoul {
    pub base: CBaseEntity,
    pub m_titanSoulScriptNetData: EHandle,
    pub m_defensivePlacement: DefensivePlacement,
    pub m_lastRodeoHitTime: f32,
    pub m_stance: i32,
    pub m_playerSettingsNum: i32,
    pub m_doomed: bool,
    pub m_invalidHealthBarEnt: bool,
    pub m_bEjecting: bool,
    pub m_isValidRodeoTarget: bool,
    pub m_nextCoreChargeAvailable: f32,
    pub m_coreChargeExpireTime: f32,
    pub m_coreChargeStartTime: f32,
    pub m_coreUseDuration: f32,
    pub m_damageComboLatestUpdateTime: f32,
    pub m_damageComboStartHealth: i32,
    pub gap_a34: [u8; 4usize],
    pub m_statusEffectsTimedTitanSoulNV: [StatusEffectTimedData; 10usize],
    pub m_statusEffectsEndlessTitanSoulNV: [StatusEffectEndlessData; 10usize],
    pub m_titan: EHandle,
}

size_assert!(SIZE_CTITANSOUL where CTitanSoul == 0xbd0 );
field_assert!(SIZE_TITANSOULSCRIPTNETDATA where CTitanSoul, m_titanSoulScriptNetData == 0x9d8);
field_assert!(SIZE_DEFENSIVEPLACEMENT where CTitanSoul, m_defensivePlacement == 0x9dc);
field_assert!(SIZE_LASTRODEOHITTIME where CTitanSoul, m_lastRodeoHitTime == 0xa04);
field_assert!(SIZE_STANCE where CTitanSoul, m_stance == 0xa08);
field_assert!(SIZE_PLAYERSETTINGSNUM where CTitanSoul, m_playerSettingsNum == 0xa0c);
field_assert!(SIZE_DOOMED where CTitanSoul, m_doomed == 0xa10);
field_assert!(SIZE_INVALIDHEALTHBARENT where CTitanSoul, m_invalidHealthBarEnt == 0xa11);
field_assert!(SIZE_BEJECTING where CTitanSoul, m_bEjecting == 0xa12);
field_assert!(SIZE_ISVALIDRODEOTARGET where CTitanSoul, m_isValidRodeoTarget == 0xa13);
field_assert!(SIZE_NEXTCORECHARGEAVAILABLE where CTitanSoul, m_nextCoreChargeAvailable == 0xa14);
field_assert!(SIZE_CORECHARGEEXPIRETIME where CTitanSoul, m_coreChargeExpireTime == 0xa18);
field_assert!(SIZE_CORECHARGESTARTTIME where CTitanSoul, m_coreChargeStartTime == 0xa1c);
field_assert!(SIZE_COREUSEDURATION where CTitanSoul, m_coreUseDuration == 0xa20);
field_assert!(SIZE_DAMAGECOMBOLATESTUPDATETIME where CTitanSoul, m_damageComboLatestUpdateTime == 0xa24);
field_assert!(SIZE_DAMAGECOMBOSTARTHEALTH where CTitanSoul, m_damageComboStartHealth == 0xa28);
field_assert!(SIZE_STATUSEFFECTSTIMEDTITANSOULNV where CTitanSoul, m_statusEffectsTimedTitanSoulNV == 0xa30);
field_assert!(SIZE_STATUSEFFECTSENDLESSTITANSOULNV where CTitanSoul, m_statusEffectsEndlessTitanSoulNV == 0xb20);
field_assert!(SIZE_TITAN where CTitanSoul, m_titan == 0xbc0);

impl DerefMut for CTitanSoul {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CTitanSoul {
    type Target = CBaseEntity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
