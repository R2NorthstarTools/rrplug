#![allow(unsafe_op_in_unsafe_fn)] // some bindings are genereted by cbinden and it doesn't follow this...

use std::fmt::{Debug, Display};

#[cfg(doc)]
use class_types::{cbaseentity::CBaseEntity, cplayer::CPlayer};

pub mod class_types;
pub mod cvar;
pub mod plugin_abi;
pub mod squirrelclasstypes;
pub mod squirreldatatypes;
pub mod squirrelfunctions;

#[repr(C)]
pub struct OffsetStructField<T, const U: usize> {
    _pad: [::std::os::raw::c_char; U],
    value: T,
}

impl<T, const U: usize> std::ops::Deref for OffsetStructField<T, U> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, const U: usize> std::ops::DerefMut for OffsetStructField<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T, const U: usize> OffsetStructField<T, U> {
    pub const fn get_inner(&self) -> &T {
        &self.value
    }

    pub const fn get_inner_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: Copy + Clone, const U: usize> OffsetStructField<T, U> {
    /// Returns the copy inner of this [`OffsetStructField<T, U>`].
    pub const fn copy_inner(&self) -> T {
        self.value
    }
}

impl<T: Debug, const U: usize> Debug for OffsetStructField<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.value))
    }
}

impl<T: Display, const U: usize> Display for OffsetStructField<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.value))
    }
}

/// trait to share to allow easier multiple up castings of certain classes
///
/// # Ex
///
/// [CBaseEntity] -> [CPlayer]
pub trait DynamicCast<T> {
    fn dynamic_cast(&self) -> Option<&T>;
    fn dynamic_cast_mut(&mut self) -> Option<&mut T>;
}
