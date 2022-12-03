#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::squirreldatatypes::*;

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_CXX23: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;

pub type SQFloat = f32;
pub type SQInteger = ::std::os::raw::c_long;
pub type SQUnsignedInteger = ::std::os::raw::c_ulong;
pub type SQChar = ::std::os::raw::c_char;
pub type SQBool = SQUnsignedInteger;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SQStackInfos {
    pub _name: *mut ::std::os::raw::c_char,
    pub _sourceName: *mut ::std::os::raw::c_char,
    pub _line: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_SQStackInfos() {
    const UNINIT: ::std::mem::MaybeUninit<SQStackInfos> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQStackInfos>(),
        24usize,
        concat!("Size of: ", stringify!(SQStackInfos))
    );
    assert_eq!(
        ::std::mem::align_of::<SQStackInfos>(),
        8usize,
        concat!("Alignment of ", stringify!(SQStackInfos))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStackInfos),
            "::",
            stringify!(_name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._sourceName) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStackInfos),
            "::",
            stringify!(_sourceName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._line) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStackInfos),
            "::",
            stringify!(_line)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SQInstruction {
    pub op: ::std::os::raw::c_int,
    pub arg1: ::std::os::raw::c_int,
    pub output: ::std::os::raw::c_int,
    pub arg2: ::std::os::raw::c_short,
    pub arg3: ::std::os::raw::c_short,
}
#[test]
fn bindgen_test_layout_SQInstruction() {
    const UNINIT: ::std::mem::MaybeUninit<SQInstruction> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQInstruction>(),
        16usize,
        concat!("Size of: ", stringify!(SQInstruction))
    );
    assert_eq!(
        ::std::mem::align_of::<SQInstruction>(),
        4usize,
        concat!("Alignment of ", stringify!(SQInstruction))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).op) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQInstruction),
            "::",
            stringify!(op)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arg1) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SQInstruction),
            "::",
            stringify!(arg1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).output) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQInstruction),
            "::",
            stringify!(output)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arg2) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SQInstruction),
            "::",
            stringify!(arg2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arg3) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(SQInstruction),
            "::",
            stringify!(arg3)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SQLexer {
    pub gap_0: [::std::os::raw::c_uchar; 112usize],
}
#[test]
fn bindgen_test_layout_SQLexer() {
    const UNINIT: ::std::mem::MaybeUninit<SQLexer> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQLexer>(),
        112usize,
        concat!("Size of: ", stringify!(SQLexer))
    );
    assert_eq!(
        ::std::mem::align_of::<SQLexer>(),
        1usize,
        concat!("Alignment of ", stringify!(SQLexer))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_0) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQLexer),
            "::",
            stringify!(gap_0)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SQCompiler {
    pub gap_0: [::std::os::raw::c_uchar; 4usize],
    pub _token: ::std::os::raw::c_int,
    pub gap_8: [::std::os::raw::c_uchar; 8usize],
    pub object_10: SQObjectType,
    pub lexer: SQLexer,
    pub gap_90: [::std::os::raw::c_uchar; 752usize],
    pub sqvm: *mut HSquirrelVM,
    pub gap_288: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_SQCompiler() {
    const UNINIT: ::std::mem::MaybeUninit<SQCompiler> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQCompiler>(),
        912usize,
        concat!("Size of: ", stringify!(SQCompiler))
    );
    assert_eq!(
        ::std::mem::align_of::<SQCompiler>(),
        8usize,
        concat!("Alignment of ", stringify!(SQCompiler))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_0) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQCompiler),
            "::",
            stringify!(gap_0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._token) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SQCompiler),
            "::",
            stringify!(_token)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_8) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQCompiler),
            "::",
            stringify!(gap_8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).object_10) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQCompiler),
            "::",
            stringify!(object_10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lexer) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SQCompiler),
            "::",
            stringify!(lexer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_90) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(SQCompiler),
            "::",
            stringify!(gap_90)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sqvm) as usize - ptr as usize },
        896usize,
        concat!(
            "Offset of field: ",
            stringify!(SQCompiler),
            "::",
            stringify!(sqvm)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_288) as usize - ptr as usize },
        904usize,
        concat!(
            "Offset of field: ",
            stringify!(SQCompiler),
            "::",
            stringify!(gap_288)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CSquirrelVM {
    pub gap_0: [::std::os::raw::c_uchar; 8usize],
    pub sqvm: *mut HSquirrelVM,
    pub gap_10: [::std::os::raw::c_uchar; 44usize],
    pub loadEnumFromFileMaybe: ::std::os::raw::c_int,
    pub gap_40: [::std::os::raw::c_uchar; 200usize],
}
#[test]
fn bindgen_test_layout_CSquirrelVM() {
    const UNINIT: ::std::mem::MaybeUninit<CSquirrelVM> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<CSquirrelVM>(),
        264usize,
        concat!("Size of: ", stringify!(CSquirrelVM))
    );
    assert_eq!(
        ::std::mem::align_of::<CSquirrelVM>(),
        8usize,
        concat!("Alignment of ", stringify!(CSquirrelVM))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_0) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CSquirrelVM),
            "::",
            stringify!(gap_0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sqvm) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CSquirrelVM),
            "::",
            stringify!(sqvm)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_10) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CSquirrelVM),
            "::",
            stringify!(gap_10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).loadEnumFromFileMaybe) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(CSquirrelVM),
            "::",
            stringify!(loadEnumFromFileMaybe)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_40) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(CSquirrelVM),
            "::",
            stringify!(gap_40)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SQUserData {
    pub vftable: *mut ::std::os::raw::c_void,
    pub uiRef: ::std::os::raw::c_int,
    pub gap_12: [::std::os::raw::c_char; 4usize],
    pub unknown_10: ::std::os::raw::c_longlong,
    pub unknown_18: ::std::os::raw::c_longlong,
    pub unknown_20: ::std::os::raw::c_longlong,
    pub sharedState: ::std::os::raw::c_longlong,
    pub unknown_30: ::std::os::raw::c_longlong,
    pub size: ::std::os::raw::c_int,
    pub padding1: [::std::os::raw::c_char; 4usize],
    pub releaseHook: releasehookType,
    pub typeId: ::std::os::raw::c_longlong,
    pub data: [::std::os::raw::c_char; 1usize],
}
#[test]
fn bindgen_test_layout_SQUserData() {
    const UNINIT: ::std::mem::MaybeUninit<SQUserData> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQUserData>(),
        88usize,
        concat!("Size of: ", stringify!(SQUserData))
    );
    assert_eq!(
        ::std::mem::align_of::<SQUserData>(),
        8usize,
        concat!("Alignment of ", stringify!(SQUserData))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vftable) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(vftable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uiRef) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(uiRef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_12) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(gap_12)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown_10) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(unknown_10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown_18) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(unknown_18)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown_20) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(unknown_20)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sharedState) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(sharedState)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown_30) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(unknown_30)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).size) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).padding1) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(padding1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).releaseHook) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(releaseHook)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).typeId) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(typeId)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(SQUserData),
            "::",
            stringify!(data)
        )
    );
}
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
    ::std::option::Option<unsafe extern "C" fn(sqvm: *mut HSquirrelVM) -> SQRESULT>;
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
