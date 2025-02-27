//! client vesrion of [`super::cplayer`]

use crate::{high::vector::Vector3, impl_vmethods};

// opaque type
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct C_Player {
    pub vftable: *const usize,
}

impl_vmethods! {
    impl C_Player {
        pub fn get_origin() -> *const Vector3 where offset(8);
    }
}
