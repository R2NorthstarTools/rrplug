#![allow(clippy::not_unsafe_ptr_arg_deref)] // maybe remove this later

pub use rrplug_proc::{GetFromSQObject, GetFromSquirrelVm, PushToSquirrelVm};
use std::mem::MaybeUninit;

use super::{squirrel::SQHandle, vector::Vector3};
use crate::{
    bindings::{
        class_types::player::CPlayer,
        squirreldatatypes::{
            HSquirrelVM, SQArray, SQBool, SQClosure, SQFloat, SQFunctionProto, SQInteger,
            SQNativeClosure, SQObject, SQObjectType, SQString, SQStructInstance, SQTable,
        },
        unwraped::SquirrelFunctionsUnwraped,
    },
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

pub trait PushToSquirrelVm {
    fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped);
}

push_to_sqvm! {
    push_sq_string::<String>;
    push_sq_int::<i32>;
    push_sq_float::<f32>;
    push_sq_bool::<bool>;
    push_sq_vector::<Vector3>;
    push_sq_object::<MaybeUninit<SQObject>>;
    push_sq_array::<Vec<String>>;
    push_sq_array::<Vec<i32>>;
    push_sq_array::<Vec<f32>>;
    push_sq_array::<Vec<bool>>;
    push_sq_array::<Vec<Vector3>>;
    push_sq_array::<Vec<MaybeUninit<SQObject>>>;
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
    ( $( $function:ident::<$t:ty>($transformer:expr) );*; ) => { $(

        impl GetFromSquirrelVm for $t {
            #[inline]
            fn get_from_sqvm(
                sqvm: *mut HSquirrelVM,
                _sqfunctions: &SquirrelFunctionsUnwraped,
                stack_pos: i32,
            ) -> Self {
                $function(sqvm, stack_pos, $transformer)
            }
        }
    )* }
}

pub trait GetFromSquirrelVm {
    fn get_from_sqvm(
        sqvm: *mut HSquirrelVM,
        sqfunctions: &SquirrelFunctionsUnwraped,
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
    get_sq_array::<Vec<String>>(|obj| Some(unsafe{ std::ffi::CStr::from_ptr((&obj._VAL.asString.as_ref()?._val[0]) as *const i8).to_string_lossy().into() }));
    get_sq_array::<Vec<i32>>(|obj| Some(unsafe{ obj._VAL.asInteger }));
    get_sq_array::<Vec<f32>>(|obj| Some(unsafe{ obj._VAL.asFloat }));
    get_sq_array::<Vec<bool>>(|obj| Some(unsafe{ obj._VAL.asInteger != 0 }) );
    get_sq_array::<Vec<MaybeUninit<SQObject>>>(|obj| Some(std::mem::MaybeUninit::new(*obj))); // might not be sound since the stuff inside ptrs is not copied
    get_sq_array::<Vec<Vector3>>(|obj| Some((obj as *const SQObject).into()));
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
            let mut obj = std::mem::MaybeUninit::<SQObject>::uninit(); // TODO: import SQObject maybe?
            (sqfunctions.sq_getobject)(sqvm, stack_pos, obj.as_mut_ptr());
            Self::new(obj.assume_init()).expect("the SQObject wasn't a closure")
        }
    }
}
// Get From SQObject Trait

pub trait GetFromSQObject {
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

// Markers

macro_rules! is_sq_object {
    ( $( $object:ty,RT: $rt:expr,OT: $ot:expr );*; ) => { $(

        impl IsSQObject for $object {
            const OT_TYPE: SQObjectType = $ot;
            const RT_TYPE: SQObjectType = $rt;
        }
    )* }
}

pub trait IsSQObject {
    const OT_TYPE: SQObjectType;
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
}

// not a thing? SQStructDef, RT: SQObjectType::, OT: SQObjectType::;
