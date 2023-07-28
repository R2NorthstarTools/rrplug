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
            pub unsafe fn try_init(dll: &DLLPointer, static_var: &once_cell::sync::OnceCell<Self>) {
                if dll.which_dll() != $dll {
                    return;
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