#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::squirreldatatypes::*;

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SQRESULT {
    SQRESULT_ERROR = -1,
    SQRESULT_NULL = 0,
    SQRESULT_NOTNULL = 1,
}
pub type SQFunction =
    ::std::option::Option<unsafe extern "C" fn(sqvm: *mut HSquirrelVM) -> SQRESULT>;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum eSQReturnType {
    Float = 1,
    Vector = 3,
    Integer = 5,
    Boolean = 6,
    Entity = 13,
    String = 33,
    Default = 32,
    Arrays = 37,
    Asset = 40,
    Table = 38,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CompileBufferState {
    pub buffer: *const SQChar,
    pub bufferPlusLength: *const SQChar,
    pub bufferAgain: *const SQChar,
}
#[test]
fn bindgen_test_layout_CompileBufferState() {
    const UNINIT: ::std::mem::MaybeUninit<CompileBufferState> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<CompileBufferState>(),
        24usize,
        concat!("Size of: ", stringify!(CompileBufferState))
    );
    assert_eq!(
        ::std::mem::align_of::<CompileBufferState>(),
        8usize,
        concat!("Alignment of ", stringify!(CompileBufferState))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CompileBufferState),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bufferPlusLength) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CompileBufferState),
            "::",
            stringify!(bufferPlusLength)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bufferAgain) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CompileBufferState),
            "::",
            stringify!(bufferAgain)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SQFuncRegistration {
    pub squirrelFuncName: *const ::std::os::raw::c_char,
    pub cppFuncName: *const ::std::os::raw::c_char,
    pub helpText: *const ::std::os::raw::c_char,
    pub returnTypeString: *const ::std::os::raw::c_char,
    pub argTypes: *const ::std::os::raw::c_char,
    pub unknown1: u32,
    pub devLevel: u32,
    pub shortNameMaybe: *const ::std::os::raw::c_char,
    pub unknown2: u32,
    pub returnType: eSQReturnType,
    pub externalBufferPointer: *mut u32,
    pub externalBufferSize: u64,
    pub unknown3: u64,
    pub unknown4: u64,
    pub funcPtr: SQFunction,
}
#[test]
fn bindgen_test_layout_SQFuncRegistration() {
    const UNINIT: ::std::mem::MaybeUninit<SQFuncRegistration> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQFuncRegistration>(),
        104usize,
        concat!("Size of: ", stringify!(SQFuncRegistration))
    );
    assert_eq!(
        ::std::mem::align_of::<SQFuncRegistration>(),
        8usize,
        concat!("Alignment of ", stringify!(SQFuncRegistration))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).squirrelFuncName) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(squirrelFuncName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cppFuncName) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(cppFuncName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).helpText) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(helpText)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).returnTypeString) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(returnTypeString)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).argTypes) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(argTypes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown1) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(unknown1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).devLevel) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(devLevel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).shortNameMaybe) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(shortNameMaybe)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown2) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(unknown2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).returnType) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(returnType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).externalBufferPointer) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(externalBufferPointer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).externalBufferSize) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(externalBufferSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown3) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(unknown3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown4) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(unknown4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).funcPtr) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFuncRegistration),
            "::",
            stringify!(funcPtr)
        )
    );
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ScriptContext {
    SERVER = 0,
    CLIENT = 1,
    UI = 2,
}
pub type RegisterSquirrelFuncType = unsafe extern "C" fn(
    sqvm: *mut CSquirrelVM,
    funcReg: *mut SQFuncRegistration,
    unknown: ::std::os::raw::c_char,
) -> i64;
pub type sq_defconstType =
    unsafe extern "C" fn(sqvm: *mut CSquirrelVM, name: *const SQChar, value: ::std::os::raw::c_int);
pub type sq_compilebufferType = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    compileBuffer: *mut CompileBufferState,
    file: *const ::std::os::raw::c_char,
    a1: ::std::os::raw::c_int,
    bShouldThrowError: SQBool,
) -> SQRESULT;
pub type sq_callType = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    iArgs: SQInteger,
    bShouldReturn: SQBool,
    bThrowError: SQBool,
) -> SQRESULT;
pub type sq_raiseerrorType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pError: *const SQChar) -> SQInteger;
pub type sq_compilefileType = unsafe extern "C" fn(
    sqvm: *mut CSquirrelVM,
    path: *const ::std::os::raw::c_char,
    name: *const ::std::os::raw::c_char,
    a4: ::std::os::raw::c_int,
) -> bool;
pub type sq_newarrayType = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger);
pub type sq_arrayappendType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> SQRESULT;
pub type sq_newtableType = unsafe extern "C" fn(sqvm: *mut HSquirrelVM) -> SQRESULT;
pub type sq_newslotType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, idx: SQInteger, bStatic: SQBool) -> SQRESULT;
pub type sq_pushroottableType = unsafe extern "C" fn(sqvm: *mut HSquirrelVM);
pub type sq_pushstringType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pStr: *const SQChar, iLength: SQInteger);
pub type sq_pushintegerType = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, i: SQInteger);
pub type sq_pushfloatType = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, f: SQFloat);
pub type sq_pushboolType = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, b: SQBool);
pub type sq_pushassetType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, str_: *const SQChar, iLength: SQInteger);
pub type sq_pushvectorType = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pVec: *const SQFloat);
pub type sq_pushobjectType = unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pVec: *mut SQObject);
pub type sq_getstringType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> *const SQChar;
pub type sq_getintegerType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> SQInteger;
pub type sq_getfloatType =
    unsafe extern "C" fn(arg1: *mut HSquirrelVM, iStackpos: SQInteger) -> SQFloat;
pub type sq_getboolType =
    unsafe extern "C" fn(arg1: *mut HSquirrelVM, iStackpos: SQInteger) -> SQBool;
pub type sq_getType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> SQRESULT;
pub type sq_getassetType = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    iStackpos: SQInteger,
    pResult: *mut *const ::std::os::raw::c_char,
) -> SQRESULT;
pub type sq_getuserdataType = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    iStackpos: SQInteger,
    pData: *mut *mut ::std::os::raw::c_void,
    pTypeId: *mut u64,
) -> SQRESULT;
pub type sq_getvectorType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> *mut SQFloat;
pub type sq_getthisentityType = unsafe extern "C" fn(
    arg1: *mut HSquirrelVM,
    ppEntity: *mut *mut ::std::os::raw::c_void,
) -> SQBool;
pub type sq_getobjectType =
    unsafe extern "C" fn(arg1: *mut HSquirrelVM, iStackPos: SQInteger, pOutObj: *mut SQObject);
pub type sq_stackinfosType = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    iLevel: ::std::os::raw::c_int,
    pOutObj: *mut SQStackInfos,
    iCallStackSize: ::std::os::raw::c_int,
) -> ::std::os::raw::c_longlong;
pub type sq_createuserdataType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iSize: SQInteger) -> *mut ::std::os::raw::c_void;
pub type sq_setuserdatatypeidType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger, iTypeId: u64) -> SQRESULT;
pub type sq_getentityfrominstanceType =
    unsafe extern "C" fn(
        sqvm: *mut CSquirrelVM,
        pInstance: *mut SQObject,
        ppEntityConstant: *mut *mut ::std::os::raw::c_char,
    ) -> *mut super::class_types::cplayer::CPlayer;
pub type sq_GetEntityConstantType = unsafe extern "C" fn() -> *mut *mut ::std::os::raw::c_char;
pub type sq_getfunctionType = unsafe extern "C" fn(
    sqvm: *mut HSquirrelVM,
    name: *const ::std::os::raw::c_char,
    returnObj: *mut SQObject,
    signature: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int;
pub type sq_pushnewstructinstanceType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, fieldCount: ::std::os::raw::c_int) -> SQRESULT;
pub type sq_sealstructslotType =
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, slotIndex: ::std::os::raw::c_int) -> SQRESULT;
pub type RegisterSquirrelFuncType_External = unsafe extern "C" fn(
    context: ScriptContext,
    funcReg: *mut SQFuncRegistration,
    unknown: ::std::os::raw::c_char,
) -> i64;
