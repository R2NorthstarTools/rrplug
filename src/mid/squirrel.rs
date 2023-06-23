#![allow(clippy::not_unsafe_ptr_arg_deref)] // maybe remove later

use std::{ffi::CStr, mem::MaybeUninit};

use once_cell::sync::OnceCell;

use crate::{
    bindings::{
        squirreldatatypes::{HSquirrelVM, SQObject},
        unwraped::SquirrelFunctionsUnwraped,
    },
    errors::CallError,
    high::{squirrel_traits::PushToSquirrelVm, vector::Vector3},
    to_sq_string,
};

/// functions that used to interact with the sqvm
///
/// client functions are both for ui and client vms
pub static SQFUNCTIONS: SqFunctions = SqFunctions {
    client: OnceCell::new(),
    server: OnceCell::new(),
};

/// functions that used to interact with the sqvm
///
/// client functions are both for ui and client vms
pub struct SqFunctions {
    pub client: OnceCell<SquirrelFunctionsUnwraped>,
    pub server: OnceCell<SquirrelFunctionsUnwraped>,
}

// maybe this will work in the future
// const fn get_sq_function<const T: i8>() -> &'static OnceCell<SquirrelFunctionsUnwraped> {
//     const SERVER: i8 = ScriptVmType::Server as i8;
//     const CLIENT: i8 = ScriptVmType::Client as i8;
//     const UI: i8 = ScriptVmType::Ui as i8;
//     match T {
//         SERVER => &SQFUNCTIONS.server,
//         CLIENT => &SQFUNCTIONS.client,
//         UI => &SQFUNCTIONS.client,
//         _ => unreachable!(),
//     }
// }

#[inline]
pub fn push_sq_array<T>(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    arguments: Vec<T>,
) where
    T: PushToSquirrelVm,
{
    unsafe { (sqfunctions.sq_newarray)(sqvm, 0) }

    for e in arguments.into_iter() {
        e.push_to_sqvm(sqvm, sqfunctions);
        unsafe { (sqfunctions.sq_arrayappend)(sqvm, -2) };
    }
}

#[inline]
pub fn push_sq_float(sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped, float: f32) {
    unsafe { (sqfunctions.sq_pushfloat)(sqvm, float) };
}

#[inline]
pub fn push_sq_int(sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped, int: i32) {
    unsafe { (sqfunctions.sq_pushinteger)(sqvm, int) };
}

#[inline]
pub fn push_sq_bool(sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped, boolen: bool) {
    unsafe { (sqfunctions.sq_pushbool)(sqvm, boolen as u32) };
}

#[inline]
pub fn push_sq_string(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    string: impl Into<String>,
) {
    // boxing this would be a good idea and leaking; altough we don't need to?
    let cstring = to_sq_string!(string.into());
    // its impossble for it to crash since we replace null with space if it does it must be reported
    unsafe { (sqfunctions.sq_pushstring)(sqvm, cstring.as_ptr(), -1) }; // why -1?
}

#[inline]
pub fn push_sq_vector(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    vector: Vector3,
) {
    unsafe { (sqfunctions.sq_pushvector)(sqvm, (&vector).into()) };
}

#[inline]
pub fn push_sq_object(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    mut object: MaybeUninit<SQObject>,
) {
    unsafe { (sqfunctions.sq_pushobject)(sqvm, object.as_mut_ptr()) };
}

#[inline]
pub fn get_sq_array<T, F>(sqvm: *mut HSquirrelVM, stack_pos: i32, transformer: F) -> Vec<T>
where
    T: PushToSquirrelVm,
    F: Fn(&SQObject) -> Option<T>,
{
    unsafe {
        let sqvm_ref = sqvm.as_ref().expect("ok how is this sqvm invalid");

        let array = sqvm_ref
            ._stackOfCurrentFunction
            .add(stack_pos as usize)
            .as_ref()
            .expect("the stack pos may be invalid")
            ._VAL
            .asArray
            .as_ref()
            .expect("the sq object may be invalid");

        (0..array._usedSlots as usize)
            .map(|i| array._values.add(i))
            .filter_map(|obj| obj.as_ref())
            .filter_map(transformer)
            .collect()
    }
}

#[inline]
pub fn get_sq_float(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    stack_pos: i32,
) -> f32 {
    unsafe { (sqfunctions.sq_getfloat)(sqvm, stack_pos) }
}

#[inline]
pub fn get_sq_int(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    stack_pos: i32,
) -> i32 {
    unsafe { (sqfunctions.sq_getinteger)(sqvm, stack_pos) }
}

#[inline]
pub fn get_sq_bool(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    stack_pos: i32,
) -> bool {
    unsafe { (sqfunctions.sq_getbool)(sqvm, stack_pos) != 0 }
}

#[inline]
pub fn get_sq_string(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    stack_pos: i32,
) -> String {
    unsafe {
        CStr::from_ptr((sqfunctions.sq_getstring)(sqvm, stack_pos))
            .to_string_lossy()
            .into()
    }
}

#[inline]
pub fn get_sq_vector(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    stack_pos: i32,
) -> Vector3 {
    unsafe { (sqfunctions.sq_getvector)(sqvm, stack_pos).into() }
}

#[inline]
pub fn get_sq_object(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    stack_pos: i32,
) -> MaybeUninit<SQObject> {
    let mut obj: MaybeUninit<SQObject> = MaybeUninit::uninit();
    unsafe {
        (sqfunctions.sq_getobject)(sqvm, stack_pos, obj.as_mut_ptr());
    };

    obj
}

pub fn get_sq_function_object(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    function_name: impl Into<String>,
) -> Result<SQObject, CallError> {
    let mut obj = MaybeUninit::<SQObject>::zeroed();
    let ptr = obj.as_mut_ptr();

    let function_name = to_sq_string!(function_name.into());

    let result = unsafe {
        (sqfunctions.sq_getfunction)(sqvm, function_name.as_ptr(), ptr, std::ptr::null())
    };

    if result != 0 {
        Err(CallError::FunctionNotFound(
            function_name.to_string_lossy().into(),
        )) // totaly safe :clueless:
    } else {
        Ok(unsafe { obj.assume_init() })
    }
}
