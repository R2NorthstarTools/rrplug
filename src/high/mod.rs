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
#[repr(transparent)]
pub struct Handle<T> {
    inner: T,
}

impl<T> Handle<T> {
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

impl<T: Clone + Copy> Handle<T> {
    pub fn copy(&self) -> T {
        self.inner
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.inner))
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.inner))
    }
}

unsafe impl<T> Sync for Handle<T> {}
unsafe impl<T> Send for Handle<T> {}
