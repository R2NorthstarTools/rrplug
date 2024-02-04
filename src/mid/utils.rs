//! rrplug various utils

use std::ffi::{c_char, CStr, CString, NulError};

/// writes a [`str`] into a array buffer and terminating it with a null byte
#[inline]
pub fn set_c_char_array<const U: usize>(buf: &mut [c_char; U], new: &str) {
    *buf = [0; U]; // null everything
    buf.iter_mut()
        .zip(new.as_bytes())
        .for_each(|(buf_char, new)| *buf_char = *new as i8);
    buf[U - 1] = 0; // also null last byte
}

/// utility to get [`String`] from a char pointer
///
/// # Safety
///
/// read about [`CStr`]
#[inline]
pub unsafe fn from_char_ptr(ptr: *const c_char) -> String {
    unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string()
}

/// utility to get [`str`] from a char pointer
///
/// # Safety
///
/// read about [`CStr`]
#[inline]
pub unsafe fn str_from_char_ptr<'a>(ptr: *const c_char) -> Option<&'a str> {
    unsafe { CStr::from_ptr(ptr) }.to_str().ok()
}

/// adds a null byte to the end of the [`str`] which requires a allocation :(
///
/// # Panics
///
/// Panics if the [`str`] has a null char
#[inline]
pub fn to_cstring(s: &str) -> CString {
    CString::new(s).expect("cstring had a null char")
}

/// adds a null byte to the end of the [`str`] which requires a allocation :(
///
/// # Errors
///
/// This function will return an error if the [`str`] has a null char
#[inline]
pub fn try_cstring(s: &str) -> Result<CString, NulError> {
    CString::new(s)
}
