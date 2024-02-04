//! various macros for interating with c++

#[cfg(doc)]
use crate::high::squirrel_traits::SQVMName;

/// internal rrplug macro
///
/// examples of uses are in [`crate::bindings::class_types`]
#[macro_export]
macro_rules! offset_struct {
    ( $v:vis struct $struct_name:ident { $( $name:ident : $t:ty where offset($offset:literal)),*, } ) => {

        $v union $struct_name {
            $(
                $v $name: std::mem::ManuallyDrop<$crate::bindings::OffsetStructField::<$t,$offset>>,
            )*
        }
    };
}

/// internal rrplug macro
///
/// used by [`crate::impl_vmethods`]
#[macro_export]
macro_rules! impl_vmethod {
    ( pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty, for $class:ty where offset($offset:literal) ) => {
        impl $class {
            $crate::impl_vmethod!{ GEN pub fn $name( $($arg_name: $arg),* ) -> $output, for $class where offset( $offset ) }
        }
    };
    ( pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty, for WRAPPER $class:ty where offset($offset:literal) ) => {
        impl $class {
            $crate::impl_vmethod!{ GEN pub fn $name( $($arg_name: $arg),* ) -> $output, for $class where offset( $offset ) }
        }
    };
    ( pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty, for OFFSET $class:ty where offset($offset:literal) ) => {
        impl $class {
            $crate::impl_vmethod!{ GEN pub fn $name( $($arg_name: $arg),* ) -> $output, for OFFSET $class where offset( $offset ) }
        }
    };

    ( GEN pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty, for $class:ty where offset($offset:literal) ) => {
        #[doc = "# Safety" ]
        #[doc = "this is a wrapper to a vtable function" ]
        #[doc = "" ]
        #[doc = "calling this function without knowing how it works may or may create ub" ]
        #[doc = "this is a auto doc so idk how it works" ]
        pub unsafe fn $name( &self, $($arg_name: $arg),* ) -> $output {
            use std::ffi::c_void;

            let func = (*(self.vtable as *const [usize;u32::MAX as usize]))[$offset];
            (std::mem::transmute::<_,unsafe extern "C" fn(*const c_void, $($arg,)*) -> $output>(func))
            (
                self as *const _ as *const c_void,
                $( $arg_name, )*
            )
        }
    };
    ( GEN pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty, for WRAPPER $class:ty where offset($offset:literal) ) => {
        #[doc = "# Safety" ]
        #[doc = "this is a wrapper to a vtable function" ]
        #[doc = "" ]
        #[doc = "calling this function without knowing how it works may or may create ub" ]
        #[doc = "this is a auto doc so idk how it works" ]
        pub unsafe fn $name( &self, $($arg_name: $arg),* ) -> $output {
            use std::ffi::c_void;

            let func = (**(self.class as *const *const [usize;u32::MAX as usize]))[$offset];
            (std::mem::transmute::<_,unsafe extern "C" fn(*const c_void, $($arg,)*) -> $output>(func))
            (
                self.class as *const c_void,
                $( $arg_name, )*
            )
        }
    };
    ( GEN pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty, for OFFSET $class:ty where offset($offset:literal) ) => {
        #[doc = "# Safety" ]
        #[doc = "this is a wrapper to a vtable function" ]
        #[doc = "" ]
        #[doc = "Calling this function without knowing how it works may or may not create ub." ]
        #[doc = "This is a auto doc so idk how it works." ]
        pub unsafe fn $name( &self, $($arg_name: $arg),* ) -> $output {
            use std::ffi::c_void;

            let func = (*(**self.vtable as *const [usize;u32::MAX as usize]))[$offset];
            (std::mem::transmute::<_,unsafe extern "C" fn(*const c_void, $($arg,)*) -> $output>(func))
            (
                self as *const _ as *const c_void,
                $( $arg_name, )*
            )
        }
    };
}

/// internal rrplug macro
///
/// examples of uses are in [`crate::bindings::class_types`]
#[macro_export]
macro_rules! impl_vmethods {
    ( impl $class:ty { $( pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty where offset($offset:literal) );*; } ) => {
        impl $class {
            $(
                $crate::impl_vmethod!{ GEN pub fn $name( $($arg_name: $arg),* ) -> $output, for $class where offset( $offset ) }
            )*
        }
    };
    ( impl WRAPPER $class:ty { $( pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty where offset($offset:literal) );*; } ) => {
        impl $class {
            $(
                $crate::impl_vmethod!{ GEN pub fn $name( $($arg_name: $arg),* ) -> $output, for WRAPPER $class where offset( $offset ) }
            )*
        }
    };
    ( impl OFFSET $class:ty { $( pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty where offset($offset:literal) );*; } ) => {
        impl $class {
            $(
                $crate::impl_vmethod!{ GEN pub fn $name( $($arg_name: $arg),* ) -> $output, for OFFSET $class where offset( $offset ) }
            )*
        }
    };
}

/// utility macro to get functions and globals from dlls with offsets
///
/// the generated struct has to be init in `on_dll_load`
///
/// # Example
///
/// ```
/// # use rrplug::prelude::*;
/// # use rrplug::offset_functions;
///
/// offset_functions! {
///     ENGINE_FUNCTIONS + EngineFunctions for WhichDll::Engine => {
///         client_array = *const rrplug::bindings::class_types::client::CClient where offset(0x12A53F90);
///     }
/// }
///
/// // init
/// fn on_dll_load(engine_data: Option<&EngineData>, dll_ptr: &DLLPointer) {
///     unsafe { EngineFunctions::try_init(dll_ptr, &ENGINE_FUNCTIONS) };
/// }
/// ```
#[macro_export]
macro_rules! offset_functions {
    ( $static_name:ident + $struct_name:ident for $dll:expr => { $($name:ident = $t:ty where offset($addr:literal);)* } ) => {
        #[doc = "offset_functions or smth idk this auto generated btw so no real docs here lmao"]
        pub static $static_name: $crate::exports::OnceCell<$struct_name> = $crate::exports::OnceCell::new();

        #[doc(hidden)]
        pub struct $struct_name {
            $(pub $name: $t,)*
        }

        #[allow(clippy::missing_safety_doc,clippy::useless_transmute)]
        impl $struct_name {
            pub unsafe fn try_init(dll: &$crate::mid::engine::DLLPointer, static_var: &$crate::exports::OnceCell<Self>) {
                use $crate::mid::engine::WhichDll;

                if &$dll != dll.which_dll() {
                    return
                }

                _ = static_var.set( unsafe {
                    Self {
                        $(
                            $name: std::mem::transmute(dll.offset( $addr )), // transmute is used since it's easier but a lot more unsafe so yeah
                        )*
                    }
                });

            }
        }

        unsafe impl Sync for $struct_name {}
        unsafe impl Send for $struct_name {}
    }
}

/// macro to implement [`SQVMName`]
///
/// # Example
/// ```
/// # use rrplug::prelude::*;
/// # use rrplug::impl_sqvm_name;
/// struct SQStruct;
///
/// impl_sqvm_name!(SQStruct => "SQStruct");
/// ```
#[macro_export]
macro_rules! impl_sqvm_name {
    ($struct:ident => $name:literal ) => {
        impl $crate::high::squirrel_traits::SQVMName for $struct {
            fn get_sqvm_name() -> String {
                $name.into()
            }
        }
    };
}

#[cfg(test)]
mod test {
    #![allow(dead_code)]

    struct Test;

    impl_sqvm_name!(Test => "Test");

    offset_functions! {
        ENGINE_FUNCTIONS + EngineFunctions for WhichDll::Engine => {
            client_array = *const crate::bindings::class_types::client::CClient where offset(0x12A53F90);
        }
    }

    offset_functions! {
        SOME_FUNCTIONS + SomeFunctions for WhichDll::Other("some.dll") => {
            client_array = *const crate::bindings::class_types::client::CClient where offset(0xdeadbeef);
        }
    }
}
