#![allow(non_camel_case_types, non_snake_case)]
use std::ops::{Deref, DerefMut};

use crate::{bindings::server::cai_behaviorbase::CAI_BehaviorBase, size_assert};

#[repr(C)]
#[derive(Debug)]
pub struct CAI_PatrolBehavior {
    pub base: CAI_BehaviorBase,
}

size_assert!(A where CAI_PatrolBehavior == 0x50);

impl DerefMut for CAI_PatrolBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl Deref for CAI_PatrolBehavior {
    type Target = CAI_BehaviorBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
