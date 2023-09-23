#![allow(non_camel_case_types)] // whar

use super::class_types::cplayer::CPlayer;
use crate::bindings::{
    plugin_abi::SquirrelFunctions,
    squirrelclasstypes::{
        CompileBufferState, SQFuncRegistration, ScriptContext, SquirrelMessage_External_Pop,
        SQRESULT,
    },
    squirreldatatypes::{
        CSquirrelVM, HSquirrelVM, SQBool, SQChar, SQFloat, SQInteger, SQObject, SQStackInfos,
    },
};

pub type sq_schedule_call_externalType_unwraped = unsafe extern "C" fn(
    context: ScriptContext,
    funcname: *const ::std::os::raw::c_char,
    function: SquirrelMessage_External_Pop,
    userdata: *mut ::std::os::raw::c_void,
);
pub type register_squirrel_func_unwraped = unsafe extern "C" fn(
    sqvm: *mut CSquirrelVM,
    funcReg: *mut SQFuncRegistration,
    unknown: ::std::os::raw::c_char,
) -> i64;
pub type sq_defconstType_unwraped =
    unsafe extern "C" fn(sqvm: *mut CSquirrelVM, name: *const SQChar, value: ::std::os::raw::c_int);
pub type sq_compilebufferType_unwraped = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    compileBuffer: *mut CompileBufferState,
    file: *const ::std::os::raw::c_char,
    a1: ::std::os::raw::c_int,
    bShouldThrowError: SQBool,
) -> SQRESULT;
pub type sq_callType_unwraped = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    iArgs: SQInteger,
    bShouldReturn: SQBool,
    bThrowError: SQBool,
) -> SQRESULT;
pub type sq_raiseerrorType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pError: *const SQChar) -> SQInteger;
pub type sq_compilefileType_unwraped = unsafe extern "C" fn(
    sqvm: *mut CSquirrelVM,
    path: *const ::std::os::raw::c_char,
    name: *const ::std::os::raw::c_char,
    a4: ::std::os::raw::c_int,
) -> bool;
pub type sq_newarrayType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger);
pub type sq_arrayappendType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> SQRESULT;
pub type sq_newtableType_unwraped = unsafe extern "C" fn(sqvm: *mut HSquirrelVM) -> SQRESULT;
pub type sq_newslotType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, idx: SQInteger, bStatic: SQBool) -> SQRESULT;
pub type sq_pushroottableType_unwraped = unsafe extern "C" fn(sqvm: *mut HSquirrelVM);
pub type sq_pushstringType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pStr: *const SQChar, iLength: SQInteger);
pub type sq_pushintegerType_unwraped = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, i: SQInteger);
pub type sq_pushfloatType_unwraped = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, f: SQFloat);
pub type sq_pushboolType_unwraped = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, b: SQBool);
pub type sq_pushassetType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, str_: *const SQChar, iLength: SQInteger);
pub type sq_pushvectorType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pVec: *const SQFloat);
pub type sq_pushobjectType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pVec: *mut SQObject);
pub type sq_getstringType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> *const SQChar;
pub type sq_getintegerType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> SQInteger;
pub type sq_getfloatType_unwraped =
    unsafe extern "C" fn(arg1: *mut HSquirrelVM, iStackpos: SQInteger) -> SQFloat;
pub type sq_getboolType_unwraped =
    unsafe extern "C" fn(arg1: *mut HSquirrelVM, iStackpos: SQInteger) -> SQBool;
pub type sq_getType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> SQRESULT;
pub type sq_getassetType_unwraped = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    iStackpos: SQInteger,
    pResult: *mut *const ::std::os::raw::c_char,
) -> SQRESULT;
pub type sq_getuserdataType_unwraped = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    iStackpos: SQInteger,
    pData: *mut *mut ::std::os::raw::c_void,
    pTypeId: *mut u64,
) -> SQRESULT;
pub type sq_getvectorType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> *mut SQFloat;
pub type sq_getthisentityType_unwraped = unsafe extern "C" fn(
    arg1: *mut HSquirrelVM,
    ppEntity: *mut *mut ::std::os::raw::c_void,
) -> SQBool;
pub type sq_getobjectType_unwraped =
    unsafe extern "C" fn(arg1: *mut HSquirrelVM, iStackPos: SQInteger, pOutObj: *mut SQObject);
pub type sq_stackinfosType_unwraped = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    iLevel: ::std::os::raw::c_int,
    pOutObj: *mut SQStackInfos,
    iCallStackSize: ::std::os::raw::c_int,
) -> ::std::os::raw::c_longlong;
pub type sq_createuserdataType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iSize: SQInteger) -> *mut ::std::os::raw::c_void;
pub type sq_setuserdatatypeidType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger, iTypeId: u64) -> SQRESULT;
pub type sq_getentityfrominstanceType_unwraped = unsafe extern "C" fn(
    sqvm: *mut CSquirrelVM,
    pInstance: *mut SQObject,
    ppEntityConstant: *mut *mut ::std::os::raw::c_char,
) -> *mut CPlayer;
pub type sq_GetEntityConstantType_unwraped =
    unsafe extern "C" fn() -> *mut *mut ::std::os::raw::c_char;
pub type sq_getfunctionType_unwraped = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    name: *const ::std::os::raw::c_char,
    returnObj: *mut SQObject,
    signature: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int;
pub type sq_pushnewstructinstanceType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, fieldCount: ::std::os::raw::c_int) -> SQRESULT;
pub type sq_sealstructslotType_unwraped =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, slotIndex: ::std::os::raw::c_int) -> SQRESULT;
pub type RegisterSquirrelFuncType_External_unwraped = unsafe extern "C" fn(
    context: ScriptContext,
    funcReg: *mut SQFuncRegistration,
    unknown: ::std::os::raw::c_char,
) -> i64;

#[derive(Debug)]
pub struct SquirrelFunctionsUnwraped {
    pub register_squirrel_func: register_squirrel_func_unwraped,
    pub sq_defconst: sq_defconstType_unwraped,
    pub sq_compilebuffer: sq_compilebufferType_unwraped,
    pub sq_call: sq_callType_unwraped,
    pub sq_raiseerror: sq_raiseerrorType_unwraped,
    pub sq_compilefile: sq_compilefileType_unwraped,
    pub sq_newarray: sq_newarrayType_unwraped,
    pub sq_arrayappend: sq_arrayappendType_unwraped,
    pub sq_newtable: sq_newtableType_unwraped,
    pub sq_newslot: sq_newslotType_unwraped,
    pub sq_pushroottable: sq_pushroottableType_unwraped,
    pub sq_pushstring: sq_pushstringType_unwraped,
    pub sq_pushinteger: sq_pushintegerType_unwraped,
    pub sq_pushfloat: sq_pushfloatType_unwraped,
    pub sq_pushbool: sq_pushboolType_unwraped,
    pub sq_pushasset: sq_pushassetType_unwraped,
    pub sq_pushvector: sq_pushvectorType_unwraped,
    pub sq_pushobject: sq_pushobjectType_unwraped,
    pub sq_getstring: sq_getstringType_unwraped,
    pub sq_getinteger: sq_getintegerType_unwraped,
    pub sq_getfloat: sq_getfloatType_unwraped,
    pub sq_getbool: sq_getboolType_unwraped,
    pub sq_get: sq_getType_unwraped,
    pub sq_getasset: sq_getassetType_unwraped,
    pub sq_getuserdata: sq_getuserdataType_unwraped,
    pub sq_getvector: sq_getvectorType_unwraped,
    pub sq_getthisentity: sq_getthisentityType_unwraped,
    pub sq_getobject: sq_getobjectType_unwraped,
    pub sq_stackinfos: sq_stackinfosType_unwraped,
    pub sq_createuserdata: sq_createuserdataType_unwraped,
    pub sq_setuserdatatypeid: sq_setuserdatatypeidType_unwraped,
    pub sq_getfunction: sq_getfunctionType_unwraped,
    pub sq_schedule_call_external: sq_schedule_call_externalType_unwraped,
    pub sq_getentityfrominstance: sq_getentityfrominstanceType_unwraped,
    pub sq_get_entity_constant_cbase_entity: sq_GetEntityConstantType_unwraped,
    pub sq_pushnewstructinstance: sq_pushnewstructinstanceType_unwraped,
    pub sq_sealstructslot: sq_sealstructslotType_unwraped,
}
impl From<SquirrelFunctions> for SquirrelFunctionsUnwraped {
    fn from(value: SquirrelFunctions) -> Self {
        // can produce ub but the plugins v2 contract should never be broken so ub wouldn't be a problem
        // also would allow for extra optimizations compared to normal unwrap
        // this was done so release builds wouldn't collapse here which quite weird imo
        // release was triping and producing panics on `Some` values wtf
        // if the contract does somehow break then I would just die
        // like hello?
        // don't panic on Some in release
        // actually might be caused by hyper optimzations producing ub
        // so basically this is safe

        // could be rewriten with a macro
        unsafe {
            SquirrelFunctionsUnwraped {
                register_squirrel_func: value.RegisterSquirrelFunc.unwrap_unchecked(),
                sq_defconst: value.__sq_defconst.unwrap_unchecked(),
                sq_compilebuffer: value.__sq_compilebuffer.unwrap_unchecked(),
                sq_call: value.__sq_call.unwrap_unchecked(),
                sq_raiseerror: value.__sq_raiseerror.unwrap_unchecked(),
                sq_compilefile: value.__sq_compilefile.unwrap_unchecked(),
                sq_newarray: value.__sq_newarray.unwrap_unchecked(),
                sq_arrayappend: value.__sq_arrayappend.unwrap_unchecked(),
                sq_newtable: value.__sq_newtable.unwrap_unchecked(),
                sq_newslot: value.__sq_newslot.unwrap_unchecked(),
                sq_pushroottable: value.__sq_pushroottable.unwrap_unchecked(),
                sq_pushstring: value.__sq_pushstring.unwrap_unchecked(),
                sq_pushinteger: value.__sq_pushinteger.unwrap_unchecked(),
                sq_pushfloat: value.__sq_pushfloat.unwrap_unchecked(),
                sq_pushbool: value.__sq_pushbool.unwrap_unchecked(),
                sq_pushasset: value.__sq_pushasset.unwrap_unchecked(),
                sq_pushvector: value.__sq_pushvector.unwrap_unchecked(),
                sq_pushobject: value.__sq_pushobject.unwrap_unchecked(),
                sq_getstring: value.__sq_getstring.unwrap_unchecked(),
                sq_getinteger: value.__sq_getinteger.unwrap_unchecked(),
                sq_getfloat: value.__sq_getfloat.unwrap_unchecked(),
                sq_getbool: value.__sq_getbool.unwrap_unchecked(),
                sq_get: value.__sq_get.unwrap_unchecked(),
                sq_getasset: value.__sq_getasset.unwrap_unchecked(),
                sq_getuserdata: value.__sq_getuserdata.unwrap_unchecked(),
                sq_getvector: value.__sq_getvector.unwrap_unchecked(),
                sq_getthisentity: value.__sq_getthisentity.unwrap_unchecked(),
                sq_getobject: value.__sq_getobject.unwrap_unchecked(),
                sq_stackinfos: value.__sq_stackinfos.unwrap_unchecked(),
                sq_createuserdata: value.__sq_createuserdata.unwrap_unchecked(),
                sq_setuserdatatypeid: value.__sq_setuserdatatypeid.unwrap_unchecked(),
                sq_getfunction: value.__sq_getfunction.unwrap_unchecked(),
                sq_schedule_call_external: value.__sq_schedule_call_external.unwrap_unchecked(),
                sq_getentityfrominstance: value.__sq_getentityfrominstance.unwrap_unchecked(),
                sq_get_entity_constant_cbase_entity: value
                    .__sq_GetEntityConstant_CBaseEntity
                    .unwrap_unchecked(),
                sq_pushnewstructinstance: value.__sq_pushnewstructinstance.unwrap_unchecked(),
                sq_sealstructslot: value.__sq_sealstructslot.unwrap_unchecked(),
            }
        }
    }
}
