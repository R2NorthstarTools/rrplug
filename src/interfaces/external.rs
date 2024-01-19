use std::{mem::MaybeUninit, ptr::NonNull};

use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HMODULE,
        System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
    },
};

use crate::{bindings::plugin_abi::CreateInterface, mid::utils::try_cstring};

#[macro_export]
macro_rules! create_external_interface {
    { $struct_vis:vis $interface_name:ident + $mod_name:ident => {$($func_vis:vis fn $name:ident( $( $arg_name:ident : $arg:ty),*) -> $output:ty );*;}} => {
        $struct_vis use $mod_name::$interface_name;

        #[doc(hidden)]
        $struct_vis mod $mod_name {
            #![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

            #[repr(C)]
            pub struct $interface_name {
                vtable: core::ptr::NonNull<fn()>,
                _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
            }

            impl $crate::interfaces::external::SourceInterface for $interface_name {
                fn get_vtable(&self) -> core::ptr::NonNull<fn()> {
                    self.vtable
                }
            }

            #[allow(dead_code)]
            #[repr(i32)]
            pub(super) enum Counter {
                $($name,)*
            }
        }

        #[deny(unsafe_op_in_unsafe_fn)]
        #[allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]
        impl $interface_name {
            $(
                $func_vis unsafe fn $name( &self, $($arg_name: $arg,)* ) -> $output {
                    use $crate::interfaces::external::SourceInterface;
                    unsafe { (std::mem::transmute::<_,unsafe extern "C" fn($($arg),*) -> $output>(self.get_func($mod_name::Counter::$name as usize)))( $($arg_name),* ) }
                }
            )*
        }
    };
}
// document that Rtrn should not be modified
pub trait SourceInterface<Rtrn = Self> {
    fn get_vtable(&self) -> NonNull<fn()>;

    fn get_func(&self, index: usize) -> fn() {
        unsafe { *self.get_vtable().as_ptr().add(index) }
    }

    // maybe add a result here since it can fail for mutliple reasons
    unsafe fn from_dll_ptr(dll_ptr: HMODULE, interface_name: &str) -> Option<&'static Rtrn> {
        let mut status = MaybeUninit::uninit();
        unsafe {
            let create_interface = std::mem::transmute::<_, CreateInterface>(GetProcAddress(
                dll_ptr,
                PCSTR("CreateInterface\0".as_ptr()),
            )?);

            let interface_name = try_cstring(interface_name).ok()?;
            let interface =
                create_interface(interface_name.as_ptr(), status.as_mut_ptr()) as *const Rtrn;
            interface.as_ref()
        }
    }

    unsafe fn from_dll_name(dll_name: &str, interface_name: &str) -> Option<&'static Rtrn> {
        let mut status = MaybeUninit::uninit();
        unsafe {
            let dll_name = try_cstring(dll_name).ok()?;
            let create_interface = std::mem::transmute::<_, CreateInterface>(GetProcAddress(
                GetModuleHandleA(PCSTR(dll_name.as_ptr() as *const u8)).ok()?,
                PCSTR("CreateInterface\0".as_ptr()),
            )?);

            let interface_name = try_cstring(interface_name).ok()?;
            let interface =
                create_interface(interface_name.as_ptr(), status.as_mut_ptr()) as *const Rtrn;
            interface.as_ref()
        }
    }
}

mod test {
    create_external_interface! {
        pub Test + TestMod => {
            pub fn test() -> ();
            pub fn test2(test: i32) -> i32;
        }
    }

    create_external_interface! {
        pub(crate) Test2 + Test2Mod => {
            pub(super) fn get_str(index: i32) -> *const u8;
        }
    }
}
