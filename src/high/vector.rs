//! just used to define the struct for vectors

#![allow(clippy::not_unsafe_ptr_arg_deref)] // cluless
#![allow(clippy::from_over_into)]

use std::ops::{Add, Div, Mul, Sub};

/// the repersention of the source engine's vector
///
/// This is a copied struct since in reality its much more unsafe
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl From<*mut f32> for Vector3 {
    fn from(value: *mut f32) -> Self {
        unsafe { *std::mem::transmute::<*mut f32, *const Self>(value) }
    }
}

impl Into<*const f32> for &Vector3 {
    fn into(self) -> *const f32 {
        self as *const Vector3 as *const f32 // do we need to leak it?, uh wait we can't leak this, I think the caller is responsible for the memeory
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(value: (f32, f32, f32)) -> Self {
        Vector3 {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<(f32, f32)> for Vector3 {
    fn from(value: (f32, f32)) -> Self {
        Vector3 {
            x: value.0,
            y: value.1,
            z: 0.,
        }
    }
}

impl From<[f32; 3]> for Vector3 {
    fn from(value: [f32; 3]) -> Self {
        Vector3 {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<[f32; 2]> for Vector3 {
    fn from(value: [f32; 2]) -> Self {
        Vector3 {
            x: value[0],
            y: value[1],
            z: 0.,
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
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
