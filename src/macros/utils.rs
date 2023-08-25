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

#[macro_export]
macro_rules! impl_vmethod {
    ( pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty, for $class:ty where offset($offset:literal) ) => {
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
        pub unsafe fn $name( &self, $($arg_name: $arg),* ) -> $output {
            use std::ffi::c_void;


            let func = (**(self.vtable_adr as *const *const [usize;u32::MAX as usize]))[$offset];
            (std::mem::transmute::<_,unsafe extern "C" fn(*const c_void, $($arg,)*) -> $output>(func))
            (
                self.vtable_adr as *const c_void,
                $( $arg_name, )*
            )
        }
    };
    ( GEN pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty, for OFFSET $class:ty where offset($offset:literal) ) => {
        #[doc = "# Safety" ]
        #[doc = "this is a wrapper to a vtable function in a offset struct" ]
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

#[macro_export]
macro_rules! impl_vmethods {
    ( impl $class:ty { $( pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty where offset($offset:literal) );*; } ) => {
        impl $class {
            $(
                $crate::impl_vmethod!{ GEN pub fn $name( $($arg_name: $arg),* ) -> $output, for $class where offset( $offset ) }
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

#[macro_export]
macro_rules! engine_functions {
    ( $static_name:ident + $struct_name:ident for $dll:expr => { $($name:ident = $t:ty where offset($addr:literal);)* } ) => {
        pub static $static_name: $crate::OnceCell<$struct_name> = $crate::OnceCell::new();

        #[doc(hidden)]
        pub struct $struct_name {
            $(pub $name: $t,)*
        }

        #[allow(clippy::missing_safety_doc,clippy::useless_transmute)]
        impl $struct_name {
            pub unsafe fn try_init(dll: &$crate::mid::engine::DLLPointer, static_var: &$crate::OnceCell<Self>) {
                use $crate::mid::engine::WhichDll;

                if &$dll != dll.which_dll() {
                    return
                }

                _ = static_var.set( Self {
                    $(
                        $name: std::mem::transmute(dll.offset( $addr )),
                    )*
                });

            }
        }

        unsafe impl Sync for $struct_name {}
        unsafe impl Send for $struct_name {}
    }
}

#[cfg(test)]
mod test {
    #![allow(dead_code)]

    engine_functions! {
        ENGINE_FUNCTIONS + EngineFunctions for WhichDll::Engine => {
            client_array = *const crate::bindings::class_types::client::CBaseClient where offset(0x12A53F90);
        }
    }

    engine_functions! {
        SOME_FUNCTIONS + SomeFunctions for WhichDll::Other("some.dll") => {
            client_array = *const crate::bindings::class_types::client::CBaseClient where offset(0xdeadbeef);
        }
    }
}
