//! macros and traits for handling external interfaces

use std::{mem::MaybeUninit, ptr::NonNull};

use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HMODULE,
        System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
    },
};

use crate::{
    bindings::plugin_abi::CreateInterface, errors::InterfaceGetterError, mid::utils::try_cstring,
};

/// creates a type definition for the external trait
///
/// a implementation of ['SourceInterface`] is auto generated by this macro
///
/// # Example
/// ```
/// # use rrplug::create_external_interface;
/// # use rrplug::interfaces::external::*;
/// create_external_interface! {
///     pub ExternalInterface + module_to_contain_it => {
///         pub fn smth() -> ();
///         pub fn add(num1: i32, num2: i32) -> i32;
///     }
/// }
///
/// let interface = unsafe { ExternalInterface::from_dll_name("some_dll.dll", "ExternalInterface") };
/// ```
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
                #[inline]
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

        #[doc = "auto generated docs; idk how to make macros capture docs so no docs :("]
        #[doc = "# Safety"]
        #[doc = "idk"]
        #[deny(unsafe_op_in_unsafe_fn)]
        #[allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]
        impl $interface_name {
            $(
                #[doc = "auto generated docs; idk how to make macros capture docs so no docs :("]
                #[doc = "# Safety"]
                #[doc = "idk"]
                $func_vis unsafe fn $name( &self, $($arg_name: $arg,)* ) -> $output {
                    use $crate::interfaces::external::SourceInterface;
                    use std::ffi::c_void;
                    unsafe { (std::mem::transmute::<_,unsafe extern "C" fn(*const c_void, $($arg),*) -> $output>(self.get_func($mod_name::Counter::$name as usize)))(
                        self as *const Self as *const c_void, $($arg_name),*
                    ) }
                }
            )*
        }
    };
}
/// trait for interacting with external interfaces
///
/// the `get_vtable` function has to be provided and everything else will just work
///
/// the implementation is auto generated by [`create_external_interface`]
pub trait SourceInterface<Rtrn = Self> {
    /// function to return the vtable of the interface to be used by `get_func`
    fn get_vtable(&self) -> NonNull<fn()>;

    /// used internally by the [`create_external_interface`]
    #[inline]
    fn get_func(&self, index: usize) -> fn() {
        unsafe { *self.get_vtable().as_ptr().add(index) }
    }

    /// acquires the interface from a dll pointer
    ///
    /// # Safety
    /// the `dll_ptr` has to be a valid module and the `interface_name` has to match the interface type
    unsafe fn from_dll_ptr(
        dll_ptr: HMODULE,
        interface_name: &str,
    ) -> Result<&'static Rtrn, InterfaceGetterError<'_>> {
        let mut status = MaybeUninit::uninit();
        unsafe {
            let create_interface = std::mem::transmute::<_, CreateInterface>(
                GetProcAddress(dll_ptr, PCSTR("CreateInterface\0".as_ptr())).ok_or(
                    InterfaceGetterError::NullCreateInterface(dll_ptr.0 as usize),
                )?,
            );

            let cstring_interface_name = try_cstring(interface_name)?;
            let interface = create_interface(cstring_interface_name.as_ptr(), status.as_mut_ptr())
                as *const Rtrn;
            interface
                .as_ref()
                .ok_or(InterfaceGetterError::InterfaceNotFound(interface_name))
        }
    }

    /// acquires the interface from a dll name
    ///
    /// # Safety
    ///
    /// the `interface_name` has to match the interface type
    unsafe fn from_dll_name<'a>(
        dll_name: &str,
        interface_name: &'a str,
    ) -> Result<&'static Rtrn, InterfaceGetterError<'a>> {
        let mut status = MaybeUninit::uninit();
        unsafe {
            let dll_name = try_cstring(dll_name)?;
            let create_interface = std::mem::transmute::<_, CreateInterface>(
                GetProcAddress(
                    GetModuleHandleA(PCSTR(dll_name.as_ptr() as *const u8))?,
                    PCSTR("CreateInterface\0".as_ptr()),
                )
                .ok_or(InterfaceGetterError::NullCreateInterface(0x0))?,
            );

            let cstring_interface_name = try_cstring(interface_name)?;
            let interface = create_interface(cstring_interface_name.as_ptr(), status.as_mut_ptr())
                as *const Rtrn;
            interface
                .as_ref()
                .ok_or(InterfaceGetterError::InterfaceNotFound(interface_name))
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
