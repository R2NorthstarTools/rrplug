//! contains all the exposed **"safe"** function

pub mod concommands;

pub mod convars;
pub mod engine;
pub mod northstar;
pub mod squirrel;
pub mod squirrel_traits;
pub mod vector;

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

    /// # Safety
    ///
    /// this might not be safe since types get auto implemented Sync and Send
    pub unsafe fn new(value: T) -> Self {
        Self { inner: value }
    }

    pub fn get(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn take(self) -> T {
        self.inner
    }
}

impl<T: Clone + Copy> UnsafeHandle<T> {
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
