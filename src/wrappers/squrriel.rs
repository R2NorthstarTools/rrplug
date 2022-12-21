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
use super::northstar::FuncSQFuncInfo;

pub(crate) static mut FUNCTION_SQ_REGISTER: Mutex<Vec<FuncSQFuncInfo>> = Mutex::new(Vec::new());
pub static mut SQFUNCTIONS: SqFunctions = SqFunctions {
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
    pub register_squirrel_func: register_squirrel_func_type_ref,
    pub sq_defconst: sq_defconstType_ref,
    pub sq_compilebuffer: sq_compilebufferType_ref,
    pub sq_call: sq_callType_ref,
    pub sq_raiseerror: sq_raiseerrorType_ref,
    pub sq_newarray: sq_newarrayType_ref,
    pub sq_arrayappend: sq_arrayappendType_ref,
    pub sq_newtable: sq_newtableType_ref,
    pub sq_newslot: sq_newslotType_ref,
    pub sq_pushroottable: sq_pushroottableType_ref,
    pub sq_pushstring: sq_pushstringType_ref,
    pub sq_pushinteger: sq_pushintegerType_ref,
    pub sq_pushfloat: sq_pushfloatType_ref,
    pub sq_pushbool: sq_pushboolType_ref,
    pub sq_pushasset: sq_pushassetType_ref,
    pub sq_pushvector: sq_pushvectorType_ref,
    pub sq_pushobject: sq_pushobjectType_ref,
    pub sq_getstring: sq_getstringType_ref,
    pub sq_getinteger: sq_getintegerType_ref,
    pub sq_getfloat: sq_getfloatType_ref,
    pub sq_getbool: sq_getboolType_ref,
    pub sq_get: sq_getType_ref,
    pub sq_getasset: sq_getassetType_ref,
    pub sq_getuserdata: sq_getuserdataType_ref,
    pub sq_getvector: sq_getvectorType_ref,
    pub sq_createuserdata: sq_createuserdataType_ref,
    pub sq_setuserdatatypeid: sq_setuserdatatypeidType_ref,
    pub sq_getfunction: sq_getfunctionType_ref,
    pub sq_schedule_call_external: sq_schedule_call_externalType_ref,
}

pub struct SqFunctions {
    pub client: Option<SquirrelFunctionsUnwraped>,
    pub server: Option<SquirrelFunctionsUnwraped>,
}

impl From<SquirrelFunctions> for SquirrelFunctionsUnwraped {
    fn from(value: SquirrelFunctions) -> Self {
        SquirrelFunctionsUnwraped {
            register_squirrel_func: value.RegisterSquirrelFunc.unwrap(),
            sq_defconst: value.__sq_defconst.unwrap(),
            sq_compilebuffer: value.__sq_compilebuffer.unwrap(),
            sq_call: value.__sq_call.unwrap(),
            sq_raiseerror: value.__sq_raiseerror.unwrap(),
            sq_newarray: value.__sq_newarray.unwrap(),
            sq_arrayappend: value.__sq_arrayappend.unwrap(),
            sq_newtable: value.__sq_newtable.unwrap(),
            sq_newslot: value.__sq_newslot.unwrap(),
            sq_pushroottable: value.__sq_pushroottable.unwrap(),
            sq_pushstring: value.__sq_pushstring.unwrap(),
            sq_pushinteger: value.__sq_pushinteger.unwrap(),
            sq_pushfloat: value.__sq_pushfloat.unwrap(),
            sq_pushbool: value.__sq_pushbool.unwrap(),
            sq_pushasset: value.__sq_pushasset.unwrap(),
            sq_pushvector: value.__sq_pushvector.unwrap(),
            sq_pushobject: value.__sq_pushobject.unwrap(),
            sq_getstring: value.__sq_getstring.unwrap(),
            sq_getinteger: value.__sq_getinteger.unwrap(),
            sq_getfloat: value.__sq_getfloat.unwrap(),
            sq_getbool: value.__sq_getbool.unwrap(),
            sq_get: value.__sq_get.unwrap(),
            sq_getasset: value.__sq_getasset.unwrap(),
            sq_getuserdata: value.__sq_getuserdata.unwrap(),
            sq_getvector: value.__sq_getvector.unwrap(),
            sq_createuserdata: value.__sq_createuserdata.unwrap(),
            sq_setuserdatatypeid: value.__sq_setuserdatatypeid.unwrap(),
            sq_getfunction: value.__sq_getfunction.unwrap(),
            sq_schedule_call_external: value.__sq_schedule_call_external.unwrap(),
        }
    }
}
