#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type releasehookType = ::std::option::Option<
    unsafe extern "C" fn(val: *mut ::std::os::raw::c_void, size: ::std::os::raw::c_int),
>;
pub type SQFloat = f32;
pub type SQInteger = ::std::os::raw::c_long;
pub type SQUnsignedInteger = ::std::os::raw::c_ulong;
pub type SQChar = ::std::os::raw::c_char;
pub type SQBool = SQUnsignedInteger;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SQObjectType {
    _RT_NULL = 1,
    _RT_INTEGER = 2,
    _RT_FLOAT = 4,
    _RT_BOOL = 8,
    _RT_STRING = 16,
    _RT_TABLE = 32,
    _RT_ARRAY = 64,
    _RT_USERDATA = 128,
    _RT_CLOSURE = 256,
    _RT_NATIVECLOSURE = 512,
    _RT_GENERATOR = 1024,
    OT_USERPOINTER = 2048,
    _RT_THREAD = 4096,
    _RT_FUNCPROTO = 8192,
    _RT_CLASS = 16384,
    _RT_INSTANCE = 32768,
    _RT_WEAKREF = 65536,
    OT_VECTOR = 262144,
    SQOBJECT_CANBEFALSE = 16777216,
    OT_NULL = 16777217,
    OT_BOOL = 16777224,
    SQOBJECT_DELEGABLE = 33554432,
    SQOBJECT_NUMERIC = 67108864,
    OT_INTEGER = 83886082,
    OT_FLOAT = 83886084,
    SQOBJECT_REF_COUNTED = 134217728,
    OT_STRING = 134217744,
    OT_ARRAY = 134217792,
    OT_CLOSURE = 134217984,
    OT_NATIVECLOSURE = 134218240,
    OT_ASSET = 134218752,
    OT_THREAD = 134221824,
    OT_FUNCPROTO = 134225920,
    OT_CLAAS = 134234112,
    OT_STRUCT = 136314880,
    OT_WEAKREF = 134283264,
    OT_TABLE = 167772192,
    OT_USERDATA = 167772288,
    OT_INSTANCE = 167804928,
    OT_ENTITY = 171966464,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SQObjectValue {
    pub asString: *mut SQString,
    pub asTable: *mut SQTable,
    pub asClosure: *mut SQClosure,
    pub asFuncProto: *mut SQFunctionProto,
    pub asStructDef: *mut SQStructDef,
    pub as64Integer: ::std::os::raw::c_longlong,
    pub asNativeClosure: *mut SQNativeClosure,
    pub asArray: *mut SQArray,
    pub asThread: *mut HSquirrelVM,
    pub asFloat: f32,
    pub asInteger: ::std::os::raw::c_int,
    pub asUserdata: *mut SQUserData,
    pub asStructInstance: *mut SQStructInstance,
}
#[test]
fn bindgen_test_layout_SQObjectValue() {
    const UNINIT: ::std::mem::MaybeUninit<SQObjectValue> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQObjectValue>(),
        8usize,
        concat!("Size of: ", stringify!(SQObjectValue))
    );
    assert_eq!(
        ::std::mem::align_of::<SQObjectValue>(),
        8usize,
        concat!("Alignment of ", stringify!(SQObjectValue))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).asString) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(asString)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).asTable) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(asTable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).asClosure) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(asClosure)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).asFuncProto) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(asFuncProto)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).asStructDef) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(asStructDef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).as64Integer) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(as64Integer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).asNativeClosure) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(asNativeClosure)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).asArray) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(asArray)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).asThread) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(asThread)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).asFloat) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(asFloat)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).asInteger) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(asInteger)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).asUserdata) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(asUserdata)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).asStructInstance) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObjectValue),
            "::",
            stringify!(asStructInstance)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SQVector {
    pub _Type: SQObjectType,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[test]
fn bindgen_test_layout_SQVector() {
    const UNINIT: ::std::mem::MaybeUninit<SQVector> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQVector>(),
        16usize,
        concat!("Size of: ", stringify!(SQVector))
    );
    assert_eq!(
        ::std::mem::align_of::<SQVector>(),
        4usize,
        concat!("Alignment of ", stringify!(SQVector))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQVector),
            "::",
            stringify!(_Type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SQVector),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQVector),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SQVector),
            "::",
            stringify!(z)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SQObject {
    pub _Type: SQObjectType,
    pub structNumber: ::std::os::raw::c_int,
    pub _VAL: SQObjectValue,
}
#[test]
fn bindgen_test_layout_SQObject() {
    const UNINIT: ::std::mem::MaybeUninit<SQObject> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQObject>(),
        16usize,
        concat!("Size of: ", stringify!(SQObject))
    );
    assert_eq!(
        ::std::mem::align_of::<SQObject>(),
        8usize,
        concat!("Alignment of ", stringify!(SQObject))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObject),
            "::",
            stringify!(_Type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).structNumber) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObject),
            "::",
            stringify!(structNumber)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._VAL) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQObject),
            "::",
            stringify!(_VAL)
        )
    );
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct SQString {
    pub vftable: *mut ::std::os::raw::c_void,
    pub uiRef: ::std::os::raw::c_int,
    pub padding: ::std::os::raw::c_int,
    pub _next_maybe: *mut SQString,
    pub sharedState: *mut SQSharedState,
    pub length: ::std::os::raw::c_int,
    pub gap_24: [::std::os::raw::c_uchar; 4usize],
    pub _hash: [::std::os::raw::c_char; 8usize],
    pub _val: [::std::os::raw::c_char; 1usize],
}
#[test]
fn bindgen_test_layout_SQString() {
    const UNINIT: ::std::mem::MaybeUninit<SQString> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQString>(),
        56usize,
        concat!("Size of: ", stringify!(SQString))
    );
    assert_eq!(
        ::std::mem::align_of::<SQString>(),
        8usize,
        concat!("Alignment of ", stringify!(SQString))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vftable) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQString),
            "::",
            stringify!(vftable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uiRef) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQString),
            "::",
            stringify!(uiRef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).padding) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SQString),
            "::",
            stringify!(padding)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._next_maybe) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQString),
            "::",
            stringify!(_next_maybe)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sharedState) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SQString),
            "::",
            stringify!(sharedState)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SQString),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_24) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(SQString),
            "::",
            stringify!(gap_24)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._hash) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SQString),
            "::",
            stringify!(_hash)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._val) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SQString),
            "::",
            stringify!(_val)
        )
    );
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct SQTable {
    pub vftable: *mut ::std::os::raw::c_void,
    pub gap_08: [::std::os::raw::c_uchar; 4usize],
    pub uiRef: ::std::os::raw::c_int,
    pub gap_10: [::std::os::raw::c_uchar; 8usize],
    pub pointer_18: *mut ::std::os::raw::c_void,
    pub pointer_20: *mut ::std::os::raw::c_void,
    pub _sharedState: *mut ::std::os::raw::c_void,
    pub field_30: ::std::os::raw::c_longlong,
    pub _nodes: *mut tableNode,
    pub _numOfNodes: ::std::os::raw::c_int,
    pub size: ::std::os::raw::c_int,
    pub field_48: ::std::os::raw::c_int,
    pub _usedNodes: ::std::os::raw::c_int,
    pub _gap_50: [::std::os::raw::c_uchar; 20usize],
    pub field_64: ::std::os::raw::c_int,
    pub _gap_68: [::std::os::raw::c_uchar; 80usize],
}
#[test]
fn bindgen_test_layout_SQTable() {
    const UNINIT: ::std::mem::MaybeUninit<SQTable> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQTable>(),
        184usize,
        concat!("Size of: ", stringify!(SQTable))
    );
    assert_eq!(
        ::std::mem::align_of::<SQTable>(),
        8usize,
        concat!("Alignment of ", stringify!(SQTable))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vftable) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(vftable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_08) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(gap_08)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uiRef) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(uiRef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_10) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(gap_10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_18) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(pointer_18)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_20) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(pointer_20)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._sharedState) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(_sharedState)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).field_30) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(field_30)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._nodes) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(_nodes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._numOfNodes) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(_numOfNodes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).size) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).field_48) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(field_48)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._usedNodes) as usize - ptr as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(_usedNodes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._gap_50) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(_gap_50)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).field_64) as usize - ptr as usize },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(field_64)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._gap_68) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(SQTable),
            "::",
            stringify!(_gap_68)
        )
    );
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub struct SQClosure {
    pub vftable: *mut ::std::os::raw::c_void,
    pub gap_08: [::std::os::raw::c_uchar; 4usize],
    pub uiRef: ::std::os::raw::c_int,
    pub pointer_10: *mut ::std::os::raw::c_void,
    pub pointer_18: *mut ::std::os::raw::c_void,
    pub pointer_20: *mut ::std::os::raw::c_void,
    pub sharedState: *mut ::std::os::raw::c_void,
    pub obj_30: SQObject,
    pub _function: SQObject,
    pub _outervalues: *mut SQObject,
    pub gap_58: [::std::os::raw::c_uchar; 8usize],
    pub gap_60: [::std::os::raw::c_uchar; 96usize],
    pub objectPointer_C0: *mut SQObject,
    pub gap_C8: [::std::os::raw::c_uchar; 16usize],
}
#[test]
fn bindgen_test_layout_SQClosure() {
    const UNINIT: ::std::mem::MaybeUninit<SQClosure> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQClosure>(),
        216usize,
        concat!("Size of: ", stringify!(SQClosure))
    );
    assert_eq!(
        ::std::mem::align_of::<SQClosure>(),
        8usize,
        concat!("Alignment of ", stringify!(SQClosure))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vftable) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(vftable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_08) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(gap_08)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uiRef) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(uiRef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_10) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(pointer_10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_18) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(pointer_18)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_20) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(pointer_20)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sharedState) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(sharedState)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).obj_30) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(obj_30)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._function) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(_function)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._outervalues) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(_outervalues)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_58) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(gap_58)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_60) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(gap_60)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).objectPointer_C0) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(objectPointer_C0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_C8) as usize - ptr as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(SQClosure),
            "::",
            stringify!(gap_C8)
        )
    );
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub struct SQFunctionProto {
    pub vftable: *mut ::std::os::raw::c_void,
    pub gap_08: [::std::os::raw::c_uchar; 4usize],
    pub uiRef: ::std::os::raw::c_int,
    pub gap_10: [::std::os::raw::c_uchar; 8usize],
    pub pointer_18: *mut ::std::os::raw::c_void,
    pub pointer_20: *mut ::std::os::raw::c_void,
    pub sharedState: *mut ::std::os::raw::c_void,
    pub pointer_30: *mut ::std::os::raw::c_void,
    pub _fileNameType: SQObjectType,
    pub _fileName: *mut SQString,
    pub _funcNameType: SQObjectType,
    pub _funcName: *mut SQString,
    pub obj_58: SQObject,
    pub gap_68: [::std::os::raw::c_uchar; 12usize],
    pub _stacksize: ::std::os::raw::c_int,
    pub gap_78: [::std::os::raw::c_uchar; 48usize],
    pub nParameters: ::std::os::raw::c_int,
    pub gap_AC: [::std::os::raw::c_uchar; 60usize],
    pub nDefaultParams: ::std::os::raw::c_int,
    pub gap_EC: [::std::os::raw::c_uchar; 200usize],
}
#[test]
fn bindgen_test_layout_SQFunctionProto() {
    const UNINIT: ::std::mem::MaybeUninit<SQFunctionProto> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQFunctionProto>(),
        440usize,
        concat!("Size of: ", stringify!(SQFunctionProto))
    );
    assert_eq!(
        ::std::mem::align_of::<SQFunctionProto>(),
        8usize,
        concat!("Alignment of ", stringify!(SQFunctionProto))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vftable) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(vftable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_08) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(gap_08)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uiRef) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(uiRef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_10) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(gap_10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_18) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(pointer_18)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_20) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(pointer_20)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sharedState) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(sharedState)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_30) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(pointer_30)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._fileNameType) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(_fileNameType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._fileName) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(_fileName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._funcNameType) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(_funcNameType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._funcName) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(_funcName)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).obj_58) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(obj_58)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_68) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(gap_68)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._stacksize) as usize - ptr as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(_stacksize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_78) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(gap_78)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nParameters) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(nParameters)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_AC) as usize - ptr as usize },
        172usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(gap_AC)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nDefaultParams) as usize - ptr as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(nDefaultParams)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_EC) as usize - ptr as usize },
        236usize,
        concat!(
            "Offset of field: ",
            stringify!(SQFunctionProto),
            "::",
            stringify!(gap_EC)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SQStructDef {
    pub vtable: *mut ::std::os::raw::c_void,
    pub uiRef: ::std::os::raw::c_int,
    pub padding_C: [::std::os::raw::c_uchar; 4usize],
    pub unknown: [::std::os::raw::c_uchar; 24usize],
    pub sharedState: *mut SQSharedState,
    pub _nameType: SQObjectType,
    pub _name: *mut SQString,
    pub gap_38: [::std::os::raw::c_uchar; 16usize],
    pub _variableNamesType: SQObjectType,
    pub _variableNames: *mut SQTable,
    pub gap_: [::std::os::raw::c_uchar; 32usize],
}
#[test]
fn bindgen_test_layout_SQStructDef() {
    const UNINIT: ::std::mem::MaybeUninit<SQStructDef> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQStructDef>(),
        128usize,
        concat!("Size of: ", stringify!(SQStructDef))
    );
    assert_eq!(
        ::std::mem::align_of::<SQStructDef>(),
        8usize,
        concat!("Alignment of ", stringify!(SQStructDef))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vtable) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructDef),
            "::",
            stringify!(vtable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uiRef) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructDef),
            "::",
            stringify!(uiRef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).padding_C) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructDef),
            "::",
            stringify!(padding_C)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructDef),
            "::",
            stringify!(unknown)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sharedState) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructDef),
            "::",
            stringify!(sharedState)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._nameType) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructDef),
            "::",
            stringify!(_nameType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._name) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructDef),
            "::",
            stringify!(_name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_38) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructDef),
            "::",
            stringify!(gap_38)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._variableNamesType) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructDef),
            "::",
            stringify!(_variableNamesType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._variableNames) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructDef),
            "::",
            stringify!(_variableNames)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructDef),
            "::",
            stringify!(gap_)
        )
    );
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct SQNativeClosure {
    pub vftable: *mut ::std::os::raw::c_void,
    pub uiRef: ::std::os::raw::c_int,
    pub gap_C: [::std::os::raw::c_uchar; 4usize],
    pub value_10: ::std::os::raw::c_longlong,
    pub value_18: ::std::os::raw::c_longlong,
    pub value_20: ::std::os::raw::c_longlong,
    pub sharedState: *mut SQSharedState,
    pub unknown_30: ::std::os::raw::c_char,
    pub padding_34: [::std::os::raw::c_uchar; 7usize],
    pub value_38: ::std::os::raw::c_longlong,
    pub value_40: ::std::os::raw::c_longlong,
    pub value_48: ::std::os::raw::c_longlong,
    pub value_50: ::std::os::raw::c_longlong,
    pub value_58: ::std::os::raw::c_longlong,
    pub _nameType: SQObjectType,
    pub _name: *mut SQString,
    pub value_70: ::std::os::raw::c_longlong,
    pub value_78: ::std::os::raw::c_longlong,
    pub justInCaseGap_80: [::std::os::raw::c_uchar; 300usize],
}
#[test]
fn bindgen_test_layout_SQNativeClosure() {
    const UNINIT: ::std::mem::MaybeUninit<SQNativeClosure> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQNativeClosure>(),
        432usize,
        concat!("Size of: ", stringify!(SQNativeClosure))
    );
    assert_eq!(
        ::std::mem::align_of::<SQNativeClosure>(),
        8usize,
        concat!("Alignment of ", stringify!(SQNativeClosure))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vftable) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(vftable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uiRef) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(uiRef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_C) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(gap_C)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_10) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(value_10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_18) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(value_18)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_20) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(value_20)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sharedState) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(sharedState)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown_30) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(unknown_30)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).padding_34) as usize - ptr as usize },
        49usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(padding_34)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_38) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(value_38)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_40) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(value_40)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_48) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(value_48)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_50) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(value_50)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_58) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(value_58)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._nameType) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(_nameType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._name) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(_name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_70) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(value_70)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value_78) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(value_78)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).justInCaseGap_80) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(SQNativeClosure),
            "::",
            stringify!(justInCaseGap_80)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SQArray {
    pub vftable: *mut ::std::os::raw::c_void,
    pub uiRef: ::std::os::raw::c_int,
    pub gap_24: [::std::os::raw::c_uchar; 36usize],
    pub _values: *mut SQObject,
    pub _usedSlots: ::std::os::raw::c_int,
    pub _allocated: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_SQArray() {
    const UNINIT: ::std::mem::MaybeUninit<SQArray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQArray>(),
        64usize,
        concat!("Size of: ", stringify!(SQArray))
    );
    assert_eq!(
        ::std::mem::align_of::<SQArray>(),
        8usize,
        concat!("Alignment of ", stringify!(SQArray))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vftable) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQArray),
            "::",
            stringify!(vftable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uiRef) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQArray),
            "::",
            stringify!(uiRef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_24) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SQArray),
            "::",
            stringify!(gap_24)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._values) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SQArray),
            "::",
            stringify!(_values)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._usedSlots) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SQArray),
            "::",
            stringify!(_usedSlots)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._allocated) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(SQArray),
            "::",
            stringify!(_allocated)
        )
    );
}
#[repr(C)]
#[repr(align(8))]
// #[derive(Copy, Clone)]
pub struct HSquirrelVM {
    pub vftable: *mut ::std::os::raw::c_void,
    pub uiRef: ::std::os::raw::c_int,
    pub gap_8: [::std::os::raw::c_uchar; 12usize],
    pub _toString: *mut ::std::os::raw::c_void,
    pub _roottable_pointer: *mut ::std::os::raw::c_void,
    pub pointer_28: *mut ::std::os::raw::c_void,
    pub ci: *mut CallInfo,
    pub _callstack: *mut CallInfo,
    pub _callsstacksize: ::std::os::raw::c_int,
    pub _stackbase: ::std::os::raw::c_int,
    pub _stackOfCurrentFunction: *mut SQObject,
    pub sharedState: *mut SQSharedState,
    pub pointer_58: *mut ::std::os::raw::c_void,
    pub pointer_60: *mut ::std::os::raw::c_void,
    pub _top: ::std::os::raw::c_int,
    pub _stack: *mut SQObject,
    pub gap_78: [::std::os::raw::c_uchar; 8usize],
    pub _vargvstack: *mut SQObject,
    pub gap_88: [::std::os::raw::c_uchar; 8usize],
    pub temp_reg: SQObject,
    pub gapA0: [::std::os::raw::c_uchar; 8usize],
    pub pointer_A8: *mut ::std::os::raw::c_void,
    pub gap_B0: [::std::os::raw::c_uchar; 8usize],
    pub _roottable_object: SQObject,
    pub _lasterror: SQObject,
    pub _errorHandler: SQObject,
    pub field_E8: ::std::os::raw::c_longlong,
    pub traps: ::std::os::raw::c_int,
    pub gap_F4: [::std::os::raw::c_uchar; 12usize],
    pub _nnativecalls: ::std::os::raw::c_int,
    pub _suspended: ::std::os::raw::c_int,
    pub _suspended_root: ::std::os::raw::c_int,
    pub _callstacksize: ::std::os::raw::c_int,
    pub _suspended_target: ::std::os::raw::c_int,
    pub trapAmount: ::std::os::raw::c_int,
    pub _suspend_varargs: ::std::os::raw::c_int,
    pub unknown_field_11C: ::std::os::raw::c_int,
    pub object_120: SQObject,
}
impl std::fmt::Debug for HSquirrelVM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HSquirrelVM").field("vftable", &self.vftable).field("uiRef", &self.uiRef).field("gap_8", &self.gap_8).field("_toString", &self._toString).field("_roottable_pointer", &self._roottable_pointer).field("pointer_28", &self.pointer_28).field("ci", &self.ci).field("_callstack", &self._callstack).field("_callsstacksize", &self._callsstacksize).field("_stackbase", &self._stackbase).field("_stackOfCurrentFunction", &self._stackOfCurrentFunction).field("sharedState", &self.sharedState).field("pointer_58", &self.pointer_58).field("pointer_60", &self.pointer_60).field("_top", &self._top).field("_stack", &self._stack).field("gap_78", &self.gap_78).field("_vargvstack", &self._vargvstack).field("gap_88", &self.gap_88).field("temp_reg", &"unparsable").field("gapA0", &self.gapA0).field("pointer_A8", &self.pointer_A8).field("gap_B0", &self.gap_B0).field("_roottable_object", &"unparsable").field("_lasterror", &"unparsable").field("_errorHandler", &"unparsable").field("field_E8", &self.field_E8).field("traps", &self.traps).field("gap_F4", &self.gap_F4).field("_nnativecalls", &self._nnativecalls).field("_suspended", &self._suspended).field("_suspended_root", &self._suspended_root).field("_callstacksize", &self._callstacksize).field("_suspended_target", &self._suspended_target).field("trapAmount", &self.trapAmount).field("_suspend_varargs", &self._suspend_varargs).field("unknown_field_11C", &self.unknown_field_11C).field("object_120", &"unparsable").finish()
    }
}
#[test]
fn bindgen_test_layout_HSquirrelVM() {
    const UNINIT: ::std::mem::MaybeUninit<HSquirrelVM> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<HSquirrelVM>(),
        304usize,
        concat!("Size of: ", stringify!(HSquirrelVM))
    );
    assert_eq!(
        ::std::mem::align_of::<HSquirrelVM>(),
        8usize,
        concat!("Alignment of ", stringify!(HSquirrelVM))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vftable) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(vftable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uiRef) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(uiRef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_8) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(gap_8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._toString) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_toString)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._roottable_pointer) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_roottable_pointer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_28) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(pointer_28)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ci) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(ci)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._callstack) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_callstack)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._callsstacksize) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_callsstacksize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._stackbase) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_stackbase)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._stackOfCurrentFunction) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_stackOfCurrentFunction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sharedState) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(sharedState)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_58) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(pointer_58)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_60) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(pointer_60)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._top) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_top)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._stack) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_stack)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_78) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(gap_78)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._vargvstack) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_vargvstack)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_88) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(gap_88)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).temp_reg) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(temp_reg)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gapA0) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(gapA0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_A8) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(pointer_A8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_B0) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(gap_B0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._roottable_object) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_roottable_object)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._lasterror) as usize - ptr as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_lasterror)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._errorHandler) as usize - ptr as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_errorHandler)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).field_E8) as usize - ptr as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(field_E8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).traps) as usize - ptr as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(traps)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_F4) as usize - ptr as usize },
        244usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(gap_F4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._nnativecalls) as usize - ptr as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_nnativecalls)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._suspended) as usize - ptr as usize },
        260usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_suspended)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._suspended_root) as usize - ptr as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_suspended_root)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._callstacksize) as usize - ptr as usize },
        268usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_callstacksize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._suspended_target) as usize - ptr as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_suspended_target)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).trapAmount) as usize - ptr as usize },
        276usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(trapAmount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._suspend_varargs) as usize - ptr as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(_suspend_varargs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown_field_11C) as usize - ptr as usize },
        284usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(unknown_field_11C)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).object_120) as usize - ptr as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(HSquirrelVM),
            "::",
            stringify!(object_120)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SQStructInstance {
    pub vftable: *mut ::std::os::raw::c_void,
    pub uiRef: ::std::os::raw::c_int,
    pub gap_C: [::std::os::raw::c_uchar; 4usize],
    pub unknown_10: ::std::os::raw::c_longlong,
    pub pointer_18: *mut ::std::os::raw::c_void,
    pub unknown_20: ::std::os::raw::c_longlong,
    pub _sharedState: *mut SQSharedState,
    pub size: ::std::os::raw::c_uint,
    pub gap_34: [::std::os::raw::c_uchar; 4usize],
    pub data: [SQObject; 1usize],
}
#[test]
fn bindgen_test_layout_SQStructInstance() {
    const UNINIT: ::std::mem::MaybeUninit<SQStructInstance> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQStructInstance>(),
        72usize,
        concat!("Size of: ", stringify!(SQStructInstance))
    );
    assert_eq!(
        ::std::mem::align_of::<SQStructInstance>(),
        8usize,
        concat!("Alignment of ", stringify!(SQStructInstance))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vftable) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructInstance),
            "::",
            stringify!(vftable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uiRef) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructInstance),
            "::",
            stringify!(uiRef)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_C) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructInstance),
            "::",
            stringify!(gap_C)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown_10) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructInstance),
            "::",
            stringify!(unknown_10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pointer_18) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructInstance),
            "::",
            stringify!(pointer_18)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown_20) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructInstance),
            "::",
            stringify!(unknown_20)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._sharedState) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructInstance),
            "::",
            stringify!(_sharedState)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).size) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructInstance),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_34) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructInstance),
            "::",
            stringify!(gap_34)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SQStructInstance),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SQSharedState {
    pub gap_0: [::std::os::raw::c_uchar; 72usize],
    pub unknown: *mut ::std::os::raw::c_void,
    pub gap_50: [::std::os::raw::c_uchar; 16344usize],
    pub _unknownTableType00: SQObjectType,
    pub _unknownTableValue00: ::std::os::raw::c_longlong,
    pub gap_4038: [::std::os::raw::c_uchar; 16usize],
    pub _stringTable: *mut StringTable,
    pub gap_4050: [::std::os::raw::c_uchar; 32usize],
    pub _unknownTableType0: SQObjectType,
    pub _unknownTableValue0: ::std::os::raw::c_longlong,
    pub _unknownObjectType1: SQObjectType,
    pub _unknownObjectValue1: ::std::os::raw::c_longlong,
    pub gap_4090: [::std::os::raw::c_uchar; 8usize],
    pub _unknownArrayType2: SQObjectType,
    pub _unknownArrayValue2: ::std::os::raw::c_longlong,
    pub _gobalsArrayType: SQObjectType,
    pub _globalsArray: *mut SQStructInstance,
    pub gap_40B8: [::std::os::raw::c_uchar; 16usize],
    pub _nativeClosuresType: SQObjectType,
    pub _nativeClosures: *mut SQTable,
    pub _typedConstantsType: SQObjectType,
    pub _typedConstants: *mut SQTable,
    pub _untypedConstantsType: SQObjectType,
    pub _untypedConstants: *mut SQTable,
    pub _globalsMaybeType: SQObjectType,
    pub _globals: *mut SQTable,
    pub _functionsType: SQObjectType,
    pub _functions: *mut SQTable,
    pub _structsType: SQObjectType,
    pub _structs: *mut SQTable,
    pub _typeDefsType: SQObjectType,
    pub _typeDefs: *mut SQTable,
    pub unknownTableType: SQObjectType,
    pub unknownTable: *mut SQTable,
    pub _squirrelFilesType: SQObjectType,
    pub _squirrelFiles: *mut SQTable,
    pub gap_4158: [::std::os::raw::c_uchar; 80usize],
    pub _nativeClosures2Type: SQObjectType,
    pub _nativeClosures2: *mut SQTable,
    pub _entityTypesMaybeType: SQObjectType,
    pub _entityTypesMaybe: *mut SQTable,
    pub unknownTable2Type: SQObjectType,
    pub unknownTable2: *mut SQTable,
    pub gap_41D8: [::std::os::raw::c_uchar; 72usize],
    pub _compilerKeywordsType: SQObjectType,
    pub _compilerKeywords: *mut SQTable,
    pub _currentThreadMaybe: *mut HSquirrelVM,
    pub gap_4238: [::std::os::raw::c_uchar; 8usize],
    pub unknownTable3Type: SQObjectType,
    pub unknownTable3: *mut SQTable,
    pub gap_4250: [::std::os::raw::c_uchar; 16usize],
    pub unknownThreadType: SQObjectType,
    pub unknownThread: *mut SQTable,
    pub _tableNativeFunctionsType: SQObjectType,
    pub _tableNativeFunctions: *mut SQTable,
    pub _unknownTableType4: SQObjectType,
    pub _unknownObjectValue4: ::std::os::raw::c_longlong,
    pub _unknownObjectType5: SQObjectType,
    pub _unknownObjectValue5: ::std::os::raw::c_longlong,
    pub _unknownObjectType6: SQObjectType,
    pub _unknownObjectValue6: ::std::os::raw::c_longlong,
    pub _unknownObjectType7: SQObjectType,
    pub _unknownObjectValue7: ::std::os::raw::c_longlong,
    pub _unknownObjectType8: SQObjectType,
    pub _unknownObjectValue8: ::std::os::raw::c_longlong,
    pub _unknownObjectType9: SQObjectType,
    pub _unknownObjectValue9: ::std::os::raw::c_longlong,
    pub _unknownObjectType10: SQObjectType,
    pub _unknownObjectValue10: ::std::os::raw::c_longlong,
    pub _unknownObjectType11: SQObjectType,
    pub _unknownObjectValue11: ::std::os::raw::c_longlong,
    pub _unknownObjectType12: SQObjectType,
    pub _unknownObjectValue12: ::std::os::raw::c_longlong,
    pub _unknownObjectType13: SQObjectType,
    pub _unknownObjectValue13: ::std::os::raw::c_longlong,
    pub _unknownObjectType14: SQObjectType,
    pub _unknownObjectValue14: ::std::os::raw::c_longlong,
    pub _unknownObjectType15: SQObjectType,
    pub _unknownObjectValue15: ::std::os::raw::c_longlong,
    pub gap_4340: [::std::os::raw::c_uchar; 16usize],
    pub printFunction: *mut ::std::os::raw::c_void,
    pub gap_4358: [::std::os::raw::c_uchar; 16usize],
    pub logEntityFunction: *mut ::std::os::raw::c_void,
    pub gap_4370: [::std::os::raw::c_uchar; 40usize],
    pub _waitStringType: SQObjectType,
    pub _waitStringValue: *mut SQString,
    pub _SpinOffAndWaitForStringType: SQObjectType,
    pub _SpinOffAndWaitForStringValue: *mut SQString,
    pub _SpinOffAndWaitForSoloStringType: SQObjectType,
    pub _SpinOffAndWaitForSoloStringValue: *mut SQString,
    pub _SpinOffStringType: SQObjectType,
    pub _SpinOffStringValue: *mut SQString,
    pub _SpinOffDelayedStringType: SQObjectType,
    pub _SpinOffDelayedStringValue: *mut SQString,
    pub gap_43E8: [::std::os::raw::c_uchar; 8usize],
    pub enableDebugInfo: bool,
    pub gap_43F1: [::std::os::raw::c_uchar; 23usize],
}
#[test]
fn bindgen_test_layout_SQSharedState() {
    const UNINIT: ::std::mem::MaybeUninit<SQSharedState> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SQSharedState>(),
        17416usize,
        concat!("Size of: ", stringify!(SQSharedState))
    );
    assert_eq!(
        ::std::mem::align_of::<SQSharedState>(),
        8usize,
        concat!("Alignment of ", stringify!(SQSharedState))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_0) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknown) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(unknown)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_50) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_50)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownTableType00) as usize - ptr as usize },
        16424usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownTableType00)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownTableValue00) as usize - ptr as usize },
        16432usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownTableValue00)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_4038) as usize - ptr as usize },
        16440usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_4038)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._stringTable) as usize - ptr as usize },
        16456usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_stringTable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_4050) as usize - ptr as usize },
        16464usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_4050)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownTableType0) as usize - ptr as usize },
        16496usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownTableType0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownTableValue0) as usize - ptr as usize },
        16504usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownTableValue0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectType1) as usize - ptr as usize },
        16512usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectType1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue1) as usize - ptr as usize },
        16520usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_4090) as usize - ptr as usize },
        16528usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_4090)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownArrayType2) as usize - ptr as usize },
        16536usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownArrayType2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownArrayValue2) as usize - ptr as usize },
        16544usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownArrayValue2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._gobalsArrayType) as usize - ptr as usize },
        16552usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_gobalsArrayType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._globalsArray) as usize - ptr as usize },
        16560usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_globalsArray)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_40B8) as usize - ptr as usize },
        16568usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_40B8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._nativeClosuresType) as usize - ptr as usize },
        16584usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_nativeClosuresType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._nativeClosures) as usize - ptr as usize },
        16592usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_nativeClosures)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._typedConstantsType) as usize - ptr as usize },
        16600usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_typedConstantsType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._typedConstants) as usize - ptr as usize },
        16608usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_typedConstants)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._untypedConstantsType) as usize - ptr as usize },
        16616usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_untypedConstantsType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._untypedConstants) as usize - ptr as usize },
        16624usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_untypedConstants)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._globalsMaybeType) as usize - ptr as usize },
        16632usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_globalsMaybeType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._globals) as usize - ptr as usize },
        16640usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_globals)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._functionsType) as usize - ptr as usize },
        16648usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_functionsType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._functions) as usize - ptr as usize },
        16656usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_functions)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._structsType) as usize - ptr as usize },
        16664usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_structsType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._structs) as usize - ptr as usize },
        16672usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_structs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._typeDefsType) as usize - ptr as usize },
        16680usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_typeDefsType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._typeDefs) as usize - ptr as usize },
        16688usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_typeDefs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknownTableType) as usize - ptr as usize },
        16696usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(unknownTableType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknownTable) as usize - ptr as usize },
        16704usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(unknownTable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._squirrelFilesType) as usize - ptr as usize },
        16712usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_squirrelFilesType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._squirrelFiles) as usize - ptr as usize },
        16720usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_squirrelFiles)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_4158) as usize - ptr as usize },
        16728usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_4158)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._nativeClosures2Type) as usize - ptr as usize },
        16808usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_nativeClosures2Type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._nativeClosures2) as usize - ptr as usize },
        16816usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_nativeClosures2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._entityTypesMaybeType) as usize - ptr as usize },
        16824usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_entityTypesMaybeType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._entityTypesMaybe) as usize - ptr as usize },
        16832usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_entityTypesMaybe)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknownTable2Type) as usize - ptr as usize },
        16840usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(unknownTable2Type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknownTable2) as usize - ptr as usize },
        16848usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(unknownTable2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_41D8) as usize - ptr as usize },
        16856usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_41D8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._compilerKeywordsType) as usize - ptr as usize },
        16928usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_compilerKeywordsType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._compilerKeywords) as usize - ptr as usize },
        16936usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_compilerKeywords)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._currentThreadMaybe) as usize - ptr as usize },
        16944usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_currentThreadMaybe)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_4238) as usize - ptr as usize },
        16952usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_4238)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknownTable3Type) as usize - ptr as usize },
        16960usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(unknownTable3Type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknownTable3) as usize - ptr as usize },
        16968usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(unknownTable3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_4250) as usize - ptr as usize },
        16976usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_4250)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknownThreadType) as usize - ptr as usize },
        16992usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(unknownThreadType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).unknownThread) as usize - ptr as usize },
        17000usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(unknownThread)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._tableNativeFunctionsType) as usize - ptr as usize },
        17008usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_tableNativeFunctionsType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._tableNativeFunctions) as usize - ptr as usize },
        17016usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_tableNativeFunctions)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownTableType4) as usize - ptr as usize },
        17024usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownTableType4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue4) as usize - ptr as usize },
        17032usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectType5) as usize - ptr as usize },
        17040usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectType5)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue5) as usize - ptr as usize },
        17048usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue5)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectType6) as usize - ptr as usize },
        17056usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectType6)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue6) as usize - ptr as usize },
        17064usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue6)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectType7) as usize - ptr as usize },
        17072usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectType7)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue7) as usize - ptr as usize },
        17080usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue7)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectType8) as usize - ptr as usize },
        17088usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectType8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue8) as usize - ptr as usize },
        17096usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectType9) as usize - ptr as usize },
        17104usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectType9)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue9) as usize - ptr as usize },
        17112usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue9)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectType10) as usize - ptr as usize },
        17120usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectType10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue10) as usize - ptr as usize },
        17128usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectType11) as usize - ptr as usize },
        17136usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectType11)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue11) as usize - ptr as usize },
        17144usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue11)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectType12) as usize - ptr as usize },
        17152usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectType12)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue12) as usize - ptr as usize },
        17160usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue12)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectType13) as usize - ptr as usize },
        17168usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectType13)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue13) as usize - ptr as usize },
        17176usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue13)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectType14) as usize - ptr as usize },
        17184usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectType14)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue14) as usize - ptr as usize },
        17192usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue14)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectType15) as usize - ptr as usize },
        17200usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectType15)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unknownObjectValue15) as usize - ptr as usize },
        17208usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_unknownObjectValue15)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_4340) as usize - ptr as usize },
        17216usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_4340)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).printFunction) as usize - ptr as usize },
        17232usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(printFunction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_4358) as usize - ptr as usize },
        17240usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_4358)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).logEntityFunction) as usize - ptr as usize },
        17256usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(logEntityFunction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_4370) as usize - ptr as usize },
        17264usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_4370)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._waitStringType) as usize - ptr as usize },
        17304usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_waitStringType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._waitStringValue) as usize - ptr as usize },
        17312usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_waitStringValue)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr)._SpinOffAndWaitForStringType) as usize - ptr as usize
        },
        17320usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_SpinOffAndWaitForStringType)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr)._SpinOffAndWaitForStringValue) as usize - ptr as usize
        },
        17328usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_SpinOffAndWaitForStringValue)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr)._SpinOffAndWaitForSoloStringType) as usize - ptr as usize
        },
        17336usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_SpinOffAndWaitForSoloStringType)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr)._SpinOffAndWaitForSoloStringValue) as usize - ptr as usize
        },
        17344usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_SpinOffAndWaitForSoloStringValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._SpinOffStringType) as usize - ptr as usize },
        17352usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_SpinOffStringType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._SpinOffStringValue) as usize - ptr as usize },
        17360usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_SpinOffStringValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._SpinOffDelayedStringType) as usize - ptr as usize },
        17368usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_SpinOffDelayedStringType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._SpinOffDelayedStringValue) as usize - ptr as usize },
        17376usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(_SpinOffDelayedStringValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_43E8) as usize - ptr as usize },
        17384usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_43E8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).enableDebugInfo) as usize - ptr as usize },
        17392usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(enableDebugInfo)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_43F1) as usize - ptr as usize },
        17393usize,
        concat!(
            "Offset of field: ",
            stringify!(SQSharedState),
            "::",
            stringify!(gap_43F1)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tableNode {
    pub val: SQObject,
    pub key: SQObject,
    pub next: *mut tableNode,
}
#[test]
fn bindgen_test_layout_tableNode() {
    const UNINIT: ::std::mem::MaybeUninit<tableNode> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tableNode>(),
        40usize,
        concat!("Size of: ", stringify!(tableNode))
    );
    assert_eq!(
        ::std::mem::align_of::<tableNode>(),
        8usize,
        concat!("Alignment of ", stringify!(tableNode))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tableNode),
            "::",
            stringify!(val)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tableNode),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tableNode),
            "::",
            stringify!(next)
        )
    );
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub struct CallInfo {
    pub ip: ::std::os::raw::c_longlong,
    pub _literals: *mut SQObject,
    pub obj10: SQObject,
    pub closure: SQObject,
    pub _etraps: [::std::os::raw::c_int; 4usize],
    pub _root: ::std::os::raw::c_int,
    pub _vargs_size: ::std::os::raw::c_short,
    pub _vargs_base: ::std::os::raw::c_short,
    pub gap: [::std::os::raw::c_uchar; 16usize],
}
#[test]
fn bindgen_test_layout_CallInfo() {
    const UNINIT: ::std::mem::MaybeUninit<CallInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<CallInfo>(),
        88usize,
        concat!("Size of: ", stringify!(CallInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<CallInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(CallInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ip) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CallInfo),
            "::",
            stringify!(ip)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._literals) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CallInfo),
            "::",
            stringify!(_literals)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).obj10) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CallInfo),
            "::",
            stringify!(obj10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).closure) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CallInfo),
            "::",
            stringify!(closure)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._etraps) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(CallInfo),
            "::",
            stringify!(_etraps)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._root) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(CallInfo),
            "::",
            stringify!(_root)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._vargs_size) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(CallInfo),
            "::",
            stringify!(_vargs_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._vargs_base) as usize - ptr as usize },
        70usize,
        concat!(
            "Offset of field: ",
            stringify!(CallInfo),
            "::",
            stringify!(_vargs_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(CallInfo),
            "::",
            stringify!(gap)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StringTable {
    pub gap_0: [::std::os::raw::c_uchar; 12usize],
    pub _numofslots: ::std::os::raw::c_int,
    pub gap_10: [::std::os::raw::c_uchar; 200usize],
}
#[test]
fn bindgen_test_layout_StringTable() {
    const UNINIT: ::std::mem::MaybeUninit<StringTable> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<StringTable>(),
        216usize,
        concat!("Size of: ", stringify!(StringTable))
    );
    assert_eq!(
        ::std::mem::align_of::<StringTable>(),
        4usize,
        concat!("Alignment of ", stringify!(StringTable))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_0) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(StringTable),
            "::",
            stringify!(gap_0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._numofslots) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(StringTable),
            "::",
            stringify!(_numofslots)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gap_10) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(StringTable),
            "::",
            stringify!(gap_10)
        )
    );
}
#[repr(C)]
#[repr(align(8))]
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
#[repr(align(4))]
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
    pub object_10: SQObject,
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
#[derive(Debug,Clone,Copy)]
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
