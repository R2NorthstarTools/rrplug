#[macro_export]
macro_rules! offset_struct {
    ( $v:vis struct $struct_name:ident { $( $name:ident : $t:ty where offset($offset:literal)),*, } ) => {

        $v struct $struct_name {
            $(
                $v $name: $crate::bindings::OffsetSructField::<$t,$offset>,
            )*
        }
    };
}

#[macro_export]
macro_rules! impl_vmethod {
    ( pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty, for $class:ty, offset $offset:literal ) => {
        impl $class {
            #[allow(clippy::missing_safety_doc)]
            pub unsafe fn $name( &self, $($arg_name: $arg),* ) -> $output {
                (std::mem::transmute::<_,unsafe extern "C" fn(*const c_void, $($arg,)*) -> $output>(self.vtable_adr.add($offset)))(
                    std::mem::transmute(self.vtable_adr),
                    $( $arg_name, )*
                )
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vmethods {
    ( $( pub fn $name:ident( $( $arg_name:ident : $arg:ty),* ) -> $output:ty, for $class:ty, offset $offset:literal );*; ) => {
        $(
            $crate::impl_vmethod!{ pub fn $name( $($arg_name: $arg),* ) -> $output, for $class, offset $offset }
        )*
    };
}
