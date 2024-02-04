//! abstractions :)

pub mod engine;
pub mod northstar;
pub mod squirrel;
pub mod squirrel_traits;
pub mod vector;

#[cfg(feature = "async_engine")]
pub mod engine_sync;

#[doc(hidden)]
#[cfg(not(feature = "async_engine"))]
pub mod engine_sync {
    #[doc(hidden)]
    #[inline(always)]
    pub const fn init_async_routine() {}

    #[doc(hidden)]
    #[inline(always)]
    pub const unsafe fn run_async_routine() {}
}

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
    #[inline]
    pub(crate) const fn internal_new(value: T) -> Self {
        Self { inner: value }
    }

    /// creates a new [`UnsafeHandle`]
    /// # Safety
    /// the handle should be used corretly as to not cause race conditions
    #[inline]
    pub const unsafe fn new(value: T) -> Self {
        Self { inner: value }
    }

    /// returns a ref to the underlying value
    #[inline]
    pub const fn get(&self) -> &T {
        &self.inner
    }

    /// returns a mut ref to the underlying value
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// consumes the [`UnsafeHandle`] and returns the underlying value
    #[inline]
    pub fn take(self) -> T {
        self.inner
    }
}

impl<T: Clone + Copy> UnsafeHandle<T> {
    /// copies the underlying value if it has [`Copy`]
    #[inline]
    pub const fn copy(&self) -> T {
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
