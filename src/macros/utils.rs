#[macro_export]
macro_rules! engine_functions {
    ( $static_name:ident + $struct_name:ident for $dll:expr => { $($name:ident = $t:ty, at $addr:literal;)* } ) => {
        pub static $static_name: once_cell::sync::OnceCell<$struct_name> = once_cell::sync::OnceCell::new();

        #[doc(hidden)]
        pub struct $struct_name {
            $(pub $name: $t,)*
        }

        #[allow(clippy::missing_safety_doc,clippy::useless_transmute)]
        impl $struct_name {
            pub unsafe fn try_init(dll: &$crate::mid::engine::DLLPointer, static_var: &once_cell::sync::OnceCell<Self>) {
                use $crate::mid::engine::WhichDll;

                // match (dll.which_dll()) {
                //     $dll if true => {},
                //     (_) => return,
                // }

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
            client_array = *const crate::bindings::entity::CBaseClient, at 0x12A53F90;
        }
    }

    engine_functions! {
        SOME_FUNCTIONS + SomeFunctions for WhichDll::Other("some.dll") => {
            client_array = *const crate::bindings::entity::CBaseClient, at 0xdeadbeef;
        }
    }
}
