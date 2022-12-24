#![allow(clippy::not_unsafe_ptr_arg_deref)] // cluless

#[derive(Copy, Clone, Debug)]
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
        unsafe {
            let raw = std::mem::transmute::<*mut f32, *mut [f32; 3usize]>(value);

            match raw.as_ref() {
                None => Self::default(),
                Some(raw) => Self {
                    x: raw[0],
                    y: raw[1],
                    z: raw[2],
                },
            }
        }
    }
}
