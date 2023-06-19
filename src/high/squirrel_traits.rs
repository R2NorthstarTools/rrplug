#![allow(clippy::not_unsafe_ptr_arg_deref)] // maybe remove this later

use std::mem::MaybeUninit;

use super::vector::Vector3;
use crate::{
    bindings::{
        squirreldatatypes::{HSquirrelVM, SQObject},
        unwraped::SquirrelFunctionsUnwraped,
    },
    mid::squirrel::{
        get_sq_array, get_sq_bool, get_sq_float, get_sq_int, get_sq_object, get_sq_string,
        get_sq_vector, push_sq_array, push_sq_bool, push_sq_float, push_sq_int, push_sq_object,
        push_sq_string, push_sq_vector,
    },
};

// Push Traits

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

// Get Traits

macro_rules! get_from_sqvm {
    ( $( $function:ident::<$t:ty> );*; ) => { $(

        impl GetFromSquirrelVm for $t {
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
}
// get_sq_array::<Vec<Vector3>>(|obj| obj.asVector.into()); // not a thing
// atuaclly might just be a *const f32 so transmute::<_,*const f32>(obj._VAL)
