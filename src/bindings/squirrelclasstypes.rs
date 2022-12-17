#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::squirreldatatypes::*;

pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize) -> !;
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
pub const SQRESULT_SQRESULT_ERROR: SQRESULT = -1;
pub const SQRESULT_SQRESULT_NULL: SQRESULT = 0;
pub const SQRESULT_SQRESULT_NOTNULL: SQRESULT = 1;
pub type SQRESULT = ::std::os::raw::c_int;
pub type SQFunction =
    *const unsafe extern "C" fn(sqvm: *mut HSquirrelVM) -> SQRESULT; // this type was inaccurate
pub const eSQReturnType_Float: eSQReturnType = 1;
pub const eSQReturnType_Vector: eSQReturnType = 3;
pub const eSQReturnType_Integer: eSQReturnType = 5;
pub const eSQReturnType_Boolean: eSQReturnType = 6;
pub const eSQReturnType_Entity: eSQReturnType = 13;
pub const eSQReturnType_String: eSQReturnType = 33;
pub const eSQReturnType_Default: eSQReturnType = 32;
pub const eSQReturnType_Arrays: eSQReturnType = 37;
pub const eSQReturnType_Asset: eSQReturnType = 40;
pub const eSQReturnType_Table: eSQReturnType = 38;
pub type eSQReturnType = ::std::os::raw::c_int;
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
pub const ScriptContext_SERVER: ScriptContext = 0;
pub const ScriptContext_CLIENT: ScriptContext = 1;
pub const ScriptContext_UI: ScriptContext = 2;
pub type ScriptContext = ::std::os::raw::c_int;
pub type SquirrelMessage_External_Pop =
    ::std::option::Option<unsafe extern "C" fn(sqvm: *mut HSquirrelVM)>;
pub type sq_schedule_call_externalType = ::std::option::Option<
    unsafe extern "C" fn(
        context: ScriptContext,
        funcname: *const ::std::os::raw::c_char,
        function: SquirrelMessage_External_Pop,
    ),
>;
pub type RegisterSquirrelFuncType = ::std::option::Option<
    unsafe extern "C" fn(
        sqvm: *mut CSquirrelVM,
        funcReg: *mut SQFuncRegistration,
        unknown: ::std::os::raw::c_char,
    ) -> i64,
>;
pub type sq_defconstType = ::std::option::Option<
    unsafe extern "C" fn(sqvm: *mut CSquirrelVM, name: *const SQChar, value: ::std::os::raw::c_int),
>;
pub type sq_compilebufferType = ::std::option::Option<
    unsafe extern "C" fn(
        sqvm: *mut HSquirrelVM,
        compileBuffer: *mut CompileBufferState,
        file: *const ::std::os::raw::c_char,
        a1: ::std::os::raw::c_int,
        bShouldThrowError: SQBool,
    ) -> SQRESULT,
>;
pub type sq_callType = ::std::option::Option<
    unsafe extern "C" fn(
        sqvm: *mut HSquirrelVM,
        iArgs: SQInteger,
        bShouldReturn: SQBool,
        bThrowError: SQBool,
    ) -> SQRESULT,
>;
pub type sq_raiseerrorType = ::std::option::Option<
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pError: *const SQChar) -> SQInteger,
>;
pub type sq_newarrayType =
    ::std::option::Option<unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger)>;
pub type sq_arrayappendType = ::std::option::Option<
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> SQRESULT,
>;
pub type sq_newtableType =
    ::std::option::Option<unsafe extern "C" fn(sqvm: *mut HSquirrelVM) -> SQRESULT>;
pub type sq_newslotType = ::std::option::Option<
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, idx: SQInteger, bStatic: SQBool) -> SQRESULT,
>;
pub type sq_pushroottableType = ::std::option::Option<unsafe extern "C" fn(sqvm: *mut HSquirrelVM)>;
pub type sq_pushstringType = ::std::option::Option<
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pStr: *const SQChar, iLength: SQInteger),
>;
pub type sq_pushintegerType =
    ::std::option::Option<unsafe extern "C" fn(sqvm: *mut HSquirrelVM, i: SQInteger)>;
pub type sq_pushfloatType =
    ::std::option::Option<unsafe extern "C" fn(sqvm: *mut HSquirrelVM, f: SQFloat)>;
pub type sq_pushboolType =
    ::std::option::Option<unsafe extern "C" fn(sqvm: *mut HSquirrelVM, b: SQBool)>;
pub type sq_pushassetType = ::std::option::Option<
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, str_: *const SQChar, iLength: SQInteger),
>;
pub type sq_pushvectorType =
    ::std::option::Option<unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pVec: *const SQFloat)>;
pub type sq_pushobjectType =
    ::std::option::Option<unsafe extern "C" fn(sqvm: *mut HSquirrelVM, pVec: *mut SQObjectType)>;
pub type sq_getstringType = ::std::option::Option<
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> *const SQChar,
>;
pub type sq_getintegerType = ::std::option::Option<
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> SQInteger,
>;
pub type sq_getfloatType = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut HSquirrelVM, iStackpos: SQInteger) -> SQFloat,
>;
pub type sq_getboolType = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut HSquirrelVM, iStackpos: SQInteger) -> SQBool,
>;
pub type sq_getType = ::std::option::Option<
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> SQRESULT,
>;
pub type sq_getassetType = ::std::option::Option<
    unsafe extern "C" fn(
        sqvm: *mut HSquirrelVM,
        iStackpos: SQInteger,
        pResult: *mut *const ::std::os::raw::c_char,
    ) -> SQRESULT,
>;
pub type sq_getuserdataType = ::std::option::Option<
    unsafe extern "C" fn(
        sqvm: *mut HSquirrelVM,
        iStackpos: SQInteger,
        pData: *mut *mut ::std::os::raw::c_void,
        pTypeId: *mut u64,
    ) -> SQRESULT,
>;
pub type sq_getvectorType = ::std::option::Option<
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger) -> *mut SQFloat,
>;
pub type sq_getthisentityType = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut HSquirrelVM,
        ppEntity: *mut *mut ::std::os::raw::c_void,
    ) -> SQBool,
>;
pub type sq_getobjectType = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut HSquirrelVM, iStackPos: SQInteger, pOutObj: *mut SQObjectType),
>;
pub type sq_createuserdataType = ::std::option::Option<
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iSize: SQInteger) -> *mut ::std::os::raw::c_void,
>;
pub type sq_setuserdatatypeidType = ::std::option::Option<
    unsafe extern "C" fn(sqvm: *mut HSquirrelVM, iStackpos: SQInteger, iTypeId: u64) -> SQRESULT,
>;
pub type sq_getentityfrominstanceType = ::std::option::Option<
    unsafe extern "C" fn(
        sqvm: *mut CSquirrelVM,
        pInstance: *mut SQObjectType,
        ppEntityConstant: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type sq_GetEntityConstantType =
    ::std::option::Option<unsafe extern "C" fn() -> *mut *mut ::std::os::raw::c_char>;
pub type sq_getfunctionType = ::std::option::Option<
    unsafe extern "C" fn(
        sqvm: *mut HSquirrelVM,
        name: *const ::std::os::raw::c_char,
        returnObj: *mut SQObjectType,
        signature: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
>;
pub type RegisterSquirrelFuncType_External = ::std::option::Option<
    unsafe extern "C" fn(
        context: ScriptContext,
        funcReg: *mut SQFuncRegistration,
        unknown: ::std::os::raw::c_char,
    ) -> i64,
>;
