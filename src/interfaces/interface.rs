//! definitions for memory layout of an interface and how to init it

use std::{ffi::c_void, marker::PhantomData, ptr::NonNull};

/// the memory layout for a exposed source interface
#[repr(C)]
pub struct Interface<T: Sync + Send> {
    vtable: NonNull<*const c_void>,
    pub(crate) data: T,
    marker: PhantomData<*mut T>, // should make it not sync
}

impl<T: Sync + Send> Interface<T> {
    /// Creates a new [`Interface<T>`] from an array of associated functions and the data of the struct/interface.
    pub const fn new(vtable: NonNull<*const c_void>, interface_data: T) -> Self {
        Self {
            vtable,
            data: interface_data,
            marker: PhantomData,
        }
    }
}

// TODO: add examples here or in some other place

/// used to created the interface layout before registering it
pub trait AsInterface: Sized + Sync + Send {
    /// used to created the interface layout before registering it
    fn to_interface(self) -> Interface<Self>;
}
