#![allow(non_camel_case_types)] // whar

use std::sync::Mutex;

use crate::bindings::{
    plugin_abi::SquirrelFunctions,
    squirrelclasstypes::{
        CompileBufferState, SQFuncRegistration, ScriptContext, SquirrelMessage_External_Pop,
        SQRESULT,
    },
    squirreldatatypes::{
        CSquirrelVM, HSquirrelVM, SQBool, SQChar, SQFloat, SQInteger, SQObjectType,
    },
};

pub(crate) static mut FUNCTION_SQ_REGISTER: Mutex<Vec<SQFuncRegistration>> = Mutex::new(Vec::new());
pub(crate) static mut SQFUNCTIONS: SqFunctions = SqFunctions {
    client: None,
    server: None,
};

pub type sq_schedule_call_externalType_ref = unsafe extern "C" fn(
    context: ScriptContext,
    funcname: *const ::std::os::raw::c_char,
    function: SquirrelMessage_External_Pop,
);
pub type register_squirrel_func_type_ref = unsafe extern "C" fn(
    sqvm: *mut CSquirrelVM,
    funcReg: *mut SQFuncRegistration,
    unknown: ::std::os::raw::c_char,
) -> i64;
pub type sq_defconstType_ref =
    unsafe extern "C" fn(sqvm: *mut CSquirrelVM, name: *const SQChar, value: ::std::os::raw::c_int);
pub type sq_compilebufferType_ref = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    compileBuffer: *mut CompileBufferState,
    file: *const ::std::os::raw::c_char,
    a1: ::std::os::raw::c_int,
    bShouldThrowError: SQBool,
) -> SQRESULT;
pub type sq_callType_ref = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    iArgs: SQInteger,
    bShouldReturn: SQBool,
    bThrowError: SQBool,
) -> SQRESULT;
pub type sq_raiseerrorType_ref =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pError: *const SQChar) -> SQInteger;
pub type sq_newarrayType_ref = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger);
pub type sq_arrayappendType_ref =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> SQRESULT;
pub type sq_newtableType_ref = unsafe extern "C" fn(sqvm: *mut HSquirrelVM) -> SQRESULT;
pub type sq_newslotType_ref =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, idx: SQInteger, bStatic: SQBool) -> SQRESULT;
pub type sq_pushroottableType_ref = unsafe extern "C" fn(sqvm: *mut HSquirrelVM);
pub type sq_pushstringType_ref =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pStr: *const SQChar, iLength: SQInteger);
pub type sq_pushintegerType_ref = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, i: SQInteger);
pub type sq_pushfloatType_ref = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, f: SQFloat);
pub type sq_pushboolType_ref = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, b: SQBool);
pub type sq_pushassetType_ref =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, str_: *const SQChar, iLength: SQInteger);
pub type sq_pushvectorType_ref = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pVec: *const SQFloat);
pub type sq_pushobjectType_ref =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pVec: *mut SQObjectType);
pub type sq_getstringType_ref =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> *const SQChar;
pub type sq_getintegerType_ref =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> SQInteger;
pub type sq_getfloatType_ref =
    unsafe extern "C" fn(arg1: *mut HSquirrelVM, iStackpos: SQInteger) -> SQFloat;
pub type sq_getboolType_ref =
    unsafe extern "C" fn(arg1: *mut HSquirrelVM, iStackpos: SQInteger) -> SQBool;
pub type sq_getType_ref =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> SQRESULT;
pub type sq_getassetType_ref = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    iStackpos: SQInteger,
    pResult: *mut *const ::std::os::raw::c_char,
) -> SQRESULT;
pub type sq_getuserdataType_ref = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    iStackpos: SQInteger,
    pData: *mut *mut ::std::os::raw::c_void,
    pTypeId: *mut u64,
) -> SQRESULT;
pub type sq_getvectorType_ref =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> *mut SQFloat;
pub type sq_getthisentityType_ref = unsafe extern "C" fn(
    arg1: *mut HSquirrelVM,
    ppEntity: *mut *mut ::std::os::raw::c_void,
) -> SQBool;
pub type sq_getobjectType_ref =
    unsafe extern "C" fn(arg1: *mut HSquirrelVM, iStackPos: SQInteger, pOutObj: *mut SQObjectType);
pub type sq_createuserdataType_ref =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iSize: SQInteger) -> *mut ::std::os::raw::c_void;
pub type sq_setuserdatatypeidType_ref =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger, iTypeId: u64) -> SQRESULT;
pub type sq_getentityfrominstanceType_ref = unsafe extern "C" fn(
    sqvm: *mut CSquirrelVM,
    pInstance: *mut SQObjectType,
    ppEntityConstant: *mut *mut ::std::os::raw::c_char,
) -> *mut ::std::os::raw::c_void;
pub type sq_GetEntityConstantType_ref = unsafe extern "C" fn() -> *mut *mut ::std::os::raw::c_char;
pub type sq_getfunctionType_ref = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    name: *const ::std::os::raw::c_char,
    returnObj: *mut SQObjectType,
    signature: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int;

pub struct SquirrelFunctionsUnwraped {
    pub register_squirrel_func: &'static register_squirrel_func_type_ref,
    pub sq_defconst: &'static sq_defconstType_ref,
    pub sq_compilebuffer: &'static sq_compilebufferType_ref,
    pub sq_call: &'static sq_callType_ref,
    pub sq_raiseerror: &'static sq_raiseerrorType_ref,
    pub sq_newarray: &'static sq_newarrayType_ref,
    pub sq_arrayappend: &'static sq_arrayappendType_ref,
    pub sq_newtable: &'static sq_newtableType_ref,
    pub sq_newslot: &'static sq_newslotType_ref,
    pub sq_pushroottable: &'static sq_pushroottableType_ref,
    pub sq_pushstring: &'static sq_pushstringType_ref,
    pub sq_pushinteger: &'static sq_pushintegerType_ref,
    pub sq_pushfloat: &'static sq_pushfloatType_ref,
    pub sq_pushbool: &'static sq_pushboolType_ref,
    pub sq_pushasset: &'static sq_pushassetType_ref,
    pub sq_pushvector: &'static sq_pushvectorType_ref,
    pub sq_pushobject: &'static sq_pushobjectType_ref,
    pub sq_getstring: &'static sq_getstringType_ref,
    pub sq_getinteger: &'static sq_getintegerType_ref,
    pub sq_getfloat: &'static sq_getfloatType_ref,
    pub sq_getbool: &'static sq_getboolType_ref,
    pub sq_get: &'static sq_getType_ref,
    pub sq_getasset: &'static sq_getassetType_ref,
    pub sq_getuserdata: &'static sq_getuserdataType_ref,
    pub sq_getvector: &'static sq_getvectorType_ref,
    pub sq_createuserdata: &'static sq_createuserdataType_ref,
    pub sq_setuserdatatypeid: &'static sq_setuserdatatypeidType_ref,
    pub sq_getfunction: &'static sq_getfunctionType_ref,
    pub sq_schedule_call_external: &'static sq_schedule_call_externalType_ref,
}

pub(crate) struct SqFunctions {
    pub(crate) client: Option<SquirrelFunctionsUnwraped>,
    pub(crate) server: Option<SquirrelFunctionsUnwraped>,
}

impl From<&'static SquirrelFunctions> for SquirrelFunctionsUnwraped {
    fn from(value: &'static SquirrelFunctions) -> Self {
        SquirrelFunctionsUnwraped {
            register_squirrel_func: value.RegisterSquirrelFunc.as_ref().unwrap(),
            sq_defconst: value.__sq_defconst.as_ref().unwrap(),
            sq_compilebuffer: value.__sq_compilebuffer.as_ref().unwrap(),
            sq_call: value.__sq_call.as_ref().unwrap(),
            sq_raiseerror: value.__sq_raiseerror.as_ref().unwrap(),
            sq_newarray: value.__sq_newarray.as_ref().unwrap(),
            sq_arrayappend: value.__sq_arrayappend.as_ref().unwrap(),
            sq_newtable: value.__sq_newtable.as_ref().unwrap(),
            sq_newslot: value.__sq_newslot.as_ref().unwrap(),
            sq_pushroottable: value.__sq_pushroottable.as_ref().unwrap(),
            sq_pushstring: value.__sq_pushstring.as_ref().unwrap(),
            sq_pushinteger: value.__sq_pushinteger.as_ref().unwrap(),
            sq_pushfloat: value.__sq_pushfloat.as_ref().unwrap(),
            sq_pushbool: value.__sq_pushbool.as_ref().unwrap(),
            sq_pushasset: value.__sq_pushasset.as_ref().unwrap(),
            sq_pushvector: value.__sq_pushvector.as_ref().unwrap(),
            sq_pushobject: value.__sq_pushobject.as_ref().unwrap(),
            sq_getstring: value.__sq_getstring.as_ref().unwrap(),
            sq_getinteger: value.__sq_getinteger.as_ref().unwrap(),
            sq_getfloat: value.__sq_getfloat.as_ref().unwrap(),
            sq_getbool: value.__sq_getbool.as_ref().unwrap(),
            sq_get: value.__sq_get.as_ref().unwrap(),
            sq_getasset: value.__sq_getasset.as_ref().unwrap(),
            sq_getuserdata: value.__sq_getuserdata.as_ref().unwrap(),
            sq_getvector: value.__sq_getvector.as_ref().unwrap(),
            sq_createuserdata: value.__sq_createuserdata.as_ref().unwrap(),
            sq_setuserdatatypeid: value.__sq_setuserdatatypeid.as_ref().unwrap(),
            sq_getfunction: value.__sq_getfunction.as_ref().unwrap(),
            sq_schedule_call_external: value.__sq_schedule_call_external.as_ref().unwrap(),
        }
    }
}
