//! just used to define the struct for vectors

#![allow(clippy::not_unsafe_ptr_arg_deref)] // cluless
#![allow(clippy::from_over_into)]

use std::ops::{Add, Div, Mul, Sub};

use crate::bindings::squirreldatatypes::{SQObject, SQVector};

/// the source engine's vector
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector3 {
    /// x plane
    pub x: f32,
    /// y plane
    pub y: f32,
    /// z plane
    pub z: f32,
}

impl Vector3 {
    /// const for a zeroed [`Vector3`]
    pub const ZERO: Self = Self {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    /// const for largest [`Vector3`] possible
    pub const MAX: Self = Self {
        x: f32::MAX,
        y: f32::MAX,
        z: f32::MAX,
    };

    /// const for smallest [`Vector3`] possible
    pub const MIN: Self = Self {
        x: f32::MIN,
        y: f32::MIN,
        z: f32::MIN,
    };

    /// creates a new [`Vector3`] from the 3 planes
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl From<*mut f32> for Vector3 {
    #[inline]
    fn from(value: *mut f32) -> Self {
        unsafe { *std::mem::transmute::<*mut f32, *const Self>(value) }
    }
}

impl From<SQVector> for Vector3 {
    #[inline]
    fn from(value: SQVector) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<*const SQObject> for Vector3 {
    #[inline]
    fn from(value: *const SQObject) -> Self {
        unsafe { std::mem::transmute::<_, SQVector>(*value) }.into()
    }
}

impl Into<*const f32> for &Vector3 {
    #[inline]
    fn into(self) -> *const f32 {
        self as *const Vector3 as *const f32 // do we need to leak it?, uh wait we can't leak this, I think the caller is responsible for the memeory
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(value: (f32, f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<(f32, f32)> for Vector3 {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: 0.,
        }
    }
}

impl From<[f32; 3]> for Vector3 {
    fn from(value: [f32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<[f32; 2]> for Vector3 {
    fn from(value: [f32; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: 0.,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
#[repr(C)]
pub struct QAngle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl QAngle {
    pub const ZERO: Self = Self {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    pub const MAX: Self = Self {
        x: f32::MAX,
        y: f32::MAX,
        z: f32::MAX,
    };

    pub const MIN: Self = Self {
        x: f32::MIN,
        y: f32::MIN,
        z: f32::MIN,
    };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl From<[f32; 4]> for QAngle {
    fn from(value: [f32; 4]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl Add for QAngle {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for QAngle {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for QAngle {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for QAngle {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
