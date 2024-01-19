use std::{ffi::c_void, marker::PhantomData, ptr::NonNull};

// #[allow(unused)] // because vtable will never be read by the plugin iteself
#[repr(C)]
pub struct Interface<T: Sync + Send> {
    vtable: NonNull<*const c_void>,
    data: T,
    marker: PhantomData<*mut T>, // should make it not sync
}

impl<T: Sync + Send> Interface<T> {
    pub const fn new(vtable: NonNull<*const c_void>, interface_data: T) -> Self {
        Self {
            vtable,
            data: interface_data,
            marker: PhantomData,
        }
    }
}

pub trait AsInterface: Sized + Sync + Send {
    fn to_interface(self) -> Interface<Self>;
}
