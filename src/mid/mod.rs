//! contains structs and functions with minimal abstraction

pub mod engine;
pub mod northstar;
pub mod reloading;
pub mod server;
pub mod source_alloc;
pub mod squirrel;
pub mod utils;

/// iterator that yields fields of an array terminated by null until it hits null :)
#[repr(transparent)]
pub struct PointerIterator<T> {
    ptr: *mut T,
}

impl<T> Iterator for PointerIterator<T> {
    type Item = *mut T;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        let rtn: *mut T = unsafe { self.ptr.as_mut()? };
        self.ptr = unsafe { self.ptr.add(1) };
        Some(rtn)
    }
}

impl<T> PointerIterator<T> {
    /// # Safety
    /// the pointer must be an array of pointers to `T` terminated by a null pointer
    pub const unsafe fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }
}
