//! abstractions :)

pub mod concommands;

pub mod class_types;
pub mod convars;
pub mod engine;
pub mod northstar;
pub mod squirrel;
pub mod squirrel_traits;
pub mod vector;

#[cfg(feature = "async_engine ")]
pub mod engine_sync;

/// allows some tf2 types to be send and sync
///
/// used to store tf2 pointers across threads
///
/// # Safety
/// when used outside of engine thread can cause race conditions or ub
///
/// [`UnsafeHandle`] should only be used to transfer the pointers to other places in the engine thread like sqfunctions or runframe
#[repr(transparent)]
pub struct UnsafeHandle<T> {
    inner: T,
}

impl<T> UnsafeHandle<T> {
    pub(crate) fn internal_new(value: T) -> Self {
        Self { inner: value }
    }

    /// creates a new [`UnsafeHandle`]
    /// # Safety
    /// the handle should be used corretly as to not cause race conditions
    pub unsafe fn new(value: T) -> Self {
        Self { inner: value }
    }

    /// returns a ref to the underlying value
    pub fn get(&self) -> &T {
        &self.inner
    }

    /// returns a mut ref to the underlying value
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// consumes the [`UnsafeHandle`] and returns the underlying value
    pub fn take(self) -> T {
        self.inner
    }
}

impl<T: Clone + Copy> UnsafeHandle<T> {
    /// copies the underlying value if it has [`Copy`]
    pub fn copy(&self) -> T {
        self.inner
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for UnsafeHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.inner))
    }
}

impl<T: std::fmt::Display> std::fmt::Display for UnsafeHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.inner))
    }
}

unsafe impl<T> Sync for UnsafeHandle<T> {}
unsafe impl<T> Send for UnsafeHandle<T> {}
