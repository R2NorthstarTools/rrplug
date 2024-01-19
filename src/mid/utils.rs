use std::ffi::{c_char, CStr, CString, NulError};

#[inline]
pub unsafe fn set_c_char_array<const U: usize>(buf: &mut [c_char; U], new: &str) {
    *buf = [0; U]; // null everything
    buf.iter_mut()
        .zip(new.as_bytes())
        .for_each(|(buf_char, new)| *buf_char = *new as i8);
    buf[U - 1] = 0; // also null last byte
}

#[inline]
pub unsafe fn from_char_ptr<T: From<String>>(ptr: *const c_char) -> T {
    unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .to_string()
        .into()
}

#[inline]
pub unsafe fn str_from_char_ptr<'a>(ptr: *const c_char) -> Option<&'a str> {
    unsafe { CStr::from_ptr(ptr) }.to_str().ok()
}

#[inline]
pub fn to_cstring(s: &str) -> CString {
    CString::new(s).expect("cstring had a null char")
}

#[inline]
pub fn try_cstring(s: &str) -> Result<CString, NulError> {
    CString::new(s)
}
