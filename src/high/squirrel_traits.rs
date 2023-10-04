//! trait definitions and implementations to generalize interacting with squirrel

#![allow(clippy::not_unsafe_ptr_arg_deref)] // maybe remove this later

pub use rrplug_proc::{
    ConstSQVMName, GetFromSQObject, GetFromSquirrelVm, PushToSquirrelVm, SQVMName,
};
use std::mem::MaybeUninit;

use super::{squirrel::SQHandle, vector::Vector3};
use crate::{
    bindings::{
        class_types::cplayer::CPlayer,
        squirreldatatypes::{
            HSquirrelVM, SQArray, SQBool, SQClosure, SQFloat, SQFunctionProto, SQInteger,
            SQNativeClosure, SQObject, SQObjectType, SQString, SQStructInstance, SQTable,
        },
        unwraped::SquirrelFunctionsUnwraped,
    },
    call_sq_object_function,
    mid::squirrel::{
        get_sq_array, get_sq_bool, get_sq_float, get_sq_int, get_sq_object, get_sq_string,
        get_sq_vector, push_sq_array, push_sq_bool, push_sq_float, push_sq_int, push_sq_object,
        push_sq_string, push_sq_vector,
    },
};

// Push Trait

macro_rules! push_to_sqvm {
    ( $( $function:ident::<$t:ty> );*; ) => { $(

        impl PushToSquirrelVm for $t {
            #[inline]
            fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
                $function(sqvm, sqfunctions, self)
            }
        }
    )* }
}

/// trait to used to generalize pushing to the sq stack
pub trait PushToSquirrelVm {
    /// pushes the value to the stack
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped);
}

push_to_sqvm! {
    push_sq_string::<String>;
    push_sq_int::<i32>;
    push_sq_float::<f32>;
    push_sq_bool::<bool>;
    push_sq_vector::<Vector3>;
    push_sq_object::<MaybeUninit<SQObject>>;
}

impl<T> PushToSquirrelVm for Vec<T>
where
    T: PushToSquirrelVm,
{
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
        push_sq_array(sqvm, sqfunctions, self);
    }
}

// Get Trait

macro_rules! get_from_sqvm {
    ( $( $function:ident::<$t:ty> );*; ) => { $(

        impl GetFromSquirrelVm for $t {
            #[inline]
            fn get_from_sqvm(
                sqvm: *mut HSquirrelVM,
                sqfunctions: &SquirrelFunctionsUnwraped,
                stack_pos: i32,
            ) -> Self {
                $function(sqvm, sqfunctions, stack_pos)
            }
        }
    )* };

    ( $( ($($ty_name: ident : $var_name:ident),*) );*; ) => { $(
        impl<$($ty_name: PushToSquirrelVm,)*> GetFromSquirrelVm for Box<dyn Fn($($ty_name,)*)> {
            fn get_from_sqvm(
                sqvm: *mut HSquirrelVM,
                sqfunctions: &'static SquirrelFunctionsUnwraped,
                stack_pos: i32,
            ) -> Self {
                Box::new(move |$($var_name: $ty_name,)*| { _ =
                    call_sq_object_function!(
                        sqvm,
                        sqfunctions,
                        SQHandle::<SQClosure>::get_from_sqvm(sqvm, sqfunctions, stack_pos),
                        $($var_name),*
                    );
                })
            }
        }
    )* }
}

/// trait to get values out of the squrriel stack
pub trait GetFromSquirrelVm {
    /// tries to get the value out of the squirrel stack but it cannot fail
    /// so this can panic
    fn get_from_sqvm(
        sqvm: *mut HSquirrelVM,
        sqfunctions: &'static SquirrelFunctionsUnwraped,
        stack_pos: i32,
    ) -> Self;
}

get_from_sqvm! {
    get_sq_string::<String>;
    get_sq_int::<i32>;
    get_sq_float::<f32>;
    get_sq_bool::<bool>;
    get_sq_vector::<Vector3>;
    get_sq_object::<MaybeUninit<SQObject>>;
}

get_from_sqvm! {
    (T1: v2);
    (T1: v1, T2: v2);
    (T1: v1, T2: v2, T3: v3);
    (T1: v1, T2: v2, T3: v3, T4: v4);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6, T7: v7);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6, T7: v7, T8: v8);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6, T7: v7, T8: v8, T9: v9);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6, T7: v7, T8: v8, T9: v9, T10: v10);
}

impl<T> GetFromSquirrelVm for Vec<T>
where
    T: GetFromSQObject,
{
    fn get_from_sqvm(
        sqvm: *mut HSquirrelVM,
        _: &'static SquirrelFunctionsUnwraped,
        stack_pos: i32,
    ) -> Self {
        get_sq_array(sqvm, stack_pos)
    }
}

impl GetFromSquirrelVm for &mut CPlayer {
    fn get_from_sqvm(
        sqvm: *mut HSquirrelVM,
        sqfunctions: &SquirrelFunctionsUnwraped,
        stack_pos: i32,
    ) -> Self {
        unsafe {
            let sqvm = sqvm.as_mut().expect("the sqvm was invalid");
            let cs_sqvm = sqvm
                .sharedState
                .as_ref()
                .expect("shared state was invalid")
                .cSquirrelVM;

            let mut obj = MaybeUninit::<SQObject>::uninit();
            (sqfunctions.sq_getobject)(sqvm, stack_pos, obj.as_mut_ptr());

            (sqfunctions.sq_getentityfrominstance)(
                cs_sqvm,
                obj.as_mut_ptr(),
                (sqfunctions.sq_get_entity_constant_cbase_entity)(),
            )
            .as_mut()
            .expect("entity was supposed to be valid")
        }
    }
}

impl GetFromSquirrelVm for SQHandle<SQClosure> {
    fn get_from_sqvm(
        sqvm: *mut HSquirrelVM,
        sqfunctions: &SquirrelFunctionsUnwraped,
        stack_pos: i32,
    ) -> Self {
        unsafe {
            let mut obj = std::mem::MaybeUninit::<SQObject>::uninit();
            (sqfunctions.sq_getobject)(sqvm, stack_pos, obj.as_mut_ptr());
            Self::new(obj.assume_init()).expect("the SQObject wasn't a closure")
        }
    }
}

// exists for dynamic returns of some functions
impl GetFromSquirrelVm for () {
    fn get_from_sqvm(_: *mut HSquirrelVM, _: &SquirrelFunctionsUnwraped, _: i32) -> Self {}
}

// Get From SQObject Trait

/// gets the value out of a sqobject
///
/// most implementations don't check the type
///
/// so this can panic if it's not the correct type
pub trait GetFromSQObject {
    /// gets the value out of a sqobject
    ///
    /// halts if the type is incorrect
    fn get_from_sqobject(obj: &SQObject) -> Self;
}

impl GetFromSQObject for String {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        unsafe {
            std::ffi::CStr::from_ptr(
                (&obj._VAL.asString.as_ref().unwrap_unchecked()._val) as *const i8,
            )
            .to_string_lossy()
            .into()
        }
    }
}
impl GetFromSQObject for i32 {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        unsafe { obj._VAL.asInteger }
    }
}
impl GetFromSQObject for f32 {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        unsafe { obj._VAL.asFloat }
    }
}
impl GetFromSQObject for bool {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        unsafe { obj._VAL.asInteger != 0 }
    }
}
impl GetFromSQObject for Vector3 {
    #[inline]
    fn get_from_sqobject(obj: &SQObject) -> Self {
        (obj as *const SQObject).into()
    }
}

// sqvm name

macro_rules! sqvm_name {
    ($( ($($ty_name:ident : $var_name:ident),*) );*;)  => {
        $(
            impl<$($ty_name: SQVMName,)*> SQVMName for Box<dyn Fn($($ty_name,)*)> {
                fn get_sqvm_name() -> String {
                    let mut name = "void functionref(".to_string();

                    $(
                        if name != "void functionref(" { // bad solution but this will run only once for each use
                            name.push(',');
                            name.push(' ');
                        }
                        name.push_str(&$ty_name::get_sqvm_name());
                    )*

                    name.push(')');

                    name
                }
            }
        )*
    };

    ( $( $t:ty = $sqty:literal );*; ) => {
        $(
            impl SQVMName for $t {
                #[inline]
                fn get_sqvm_name() -> String {
                     $sqty.to_string()
                }
            }
        )*
    };
}

/// the sqvm name of a type in rust
///
/// used to map a rust function into a sq native function
pub trait SQVMName {
    /// the name on the sqvm of a type
    ///
    /// the default is "var" which is any type
    fn get_sqvm_name() -> String;
}

sqvm_name! {
    String = "string";
    i32 = "int";
    f32 = "float";
    bool = "bool";
    Vector3 = "vector";
    &mut CPlayer = "entity";
    SQHandle<SQClosure> = "var";
    () = "void";
    std::ffi::c_void = "void"; // just for a proc macro
}

sqvm_name! {
    (T1: v2);
    (T1: v1, T2: v2);
    (T1: v1, T2: v2, T3: v3);
    (T1: v1, T2: v2, T3: v3, T4: v4);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6, T7: v7);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6, T7: v7, T8: v8);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6, T7: v7, T8: v8, T9: v9);
    (T1: v1, T2: v2, T3: v3, T4: v4, T5: v5, T6: v6, T7: v7, T8: v8, T9: v9, T10: v10);
}

impl<T: SQVMName> SQVMName for Vec<T> {
    fn get_sqvm_name() -> String {
        format!("array<{}>", T::get_sqvm_name())
    }
}
// Markers

macro_rules! is_sq_object {
    ( $( $object:ty,RT: $rt:expr,OT: $ot:expr );*; ) => { $(

        impl IsSQObject for $object {
            const OT_TYPE: SQObjectType = $ot;
            const RT_TYPE: SQObjectType = $rt;
        }
    )* }
}

/// trait to define SQObject types
pub trait IsSQObject {
    /// ot type
    const OT_TYPE: SQObjectType;
    /// return type
    const RT_TYPE: SQObjectType;
}

is_sq_object! {
    SQTable, RT: SQObjectType::RT_TABLE, OT: SQObjectType::OT_TABLE;
    SQString, RT: SQObjectType::RT_STRING, OT: SQObjectType::OT_STRING;
    SQFunctionProto, RT: SQObjectType::RT_FUNCPROTO, OT: SQObjectType::OT_FUNCPROTO;
    SQClosure, RT: SQObjectType::RT_CLOSURE, OT: SQObjectType::OT_CLOSURE;
    SQStructInstance, RT: SQObjectType::RT_INSTANCE, OT: SQObjectType::OT_INSTANCE;
    SQNativeClosure, RT: SQObjectType::RT_NATIVECLOSURE, OT: SQObjectType::OT_NATIVECLOSURE;
    SQArray, RT: SQObjectType::RT_ARRAY, OT: SQObjectType::OT_ARRAY;
    SQFloat, RT: SQObjectType::RT_FLOAT, OT: SQObjectType::OT_FLOAT;
    SQInteger, RT: SQObjectType::RT_INTEGER, OT: SQObjectType::OT_INTEGER;
    SQBool, RT: SQObjectType::RT_BOOL, OT: SQObjectType::OT_BOOL;
} // not a thing? SQStructDef, RT: SQObjectType::, OT: SQObjectType::;

// TODO: so here is the idea
// have add_sqfunction be generic over extern "C" fn s and have traits to diffrenciate client/server/ui sqfunctions
// the generic would cover mutitple implementation
// but with this version the user would have to specifically ask for a sqvm and sqfunctions
// now that I writing this the biggest problem is the return ...
// but since it's a int we could have a C struct with a i32 and it would be transparent
// this would allow the user to return anything that can become that sturct
// so this is figured out :)
// also the input could be generic over *mut sqvm
// but then it would have to be a tuple :pain:
// maybe a combination of this and proc macro would be better?
