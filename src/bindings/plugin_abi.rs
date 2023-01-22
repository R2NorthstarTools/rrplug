#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::{squirrelclasstypes::*, squirreldatatypes::CSquirrelVM};

pub const GameState_LOADING: GameState = 0;
pub const GameState_MAINMENU: GameState = 1;
pub const GameState_LOBBY: GameState = 2;
pub const GameState_INGAME: GameState = 3;
pub type GameState = ::std::os::raw::c_int;
pub const PluginLoadDLL_ENGINE: PluginLoadDLL = 0;
pub const PluginLoadDLL_CLIENT: PluginLoadDLL = 1;
pub const PluginLoadDLL_SERVER: PluginLoadDLL = 2;
pub type PluginLoadDLL = ::std::os::raw::c_int;
pub const ObjectType_CONCOMMANDS: ObjectType = 0;
pub const ObjectType_CONVAR: ObjectType = 1;
pub type ObjectType = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SquirrelFunctions {
    pub RegisterSquirrelFunc: RegisterSquirrelFuncType,
    pub __sq_defconst: sq_defconstType,
    
    pub __sq_compilebuffer: sq_compilebufferType,
    pub __sq_call: sq_callType,
    pub __sq_raiseerror: sq_raiseerrorType,

    pub __sq_newarray: sq_newarrayType,
    pub __sq_arrayappend: sq_arrayappendType,

    pub __sq_newtable: sq_newtableType,
    pub __sq_newslot: sq_newslotType,

    pub __sq_pushroottable: sq_pushroottableType,
    pub __sq_pushstring: sq_pushstringType,
    pub __sq_pushinteger: sq_pushintegerType,
    pub __sq_pushfloat: sq_pushfloatType,
    pub __sq_pushbool: sq_pushboolType,
    pub __sq_pushasset: sq_pushassetType,
    pub __sq_pushvector: sq_pushvectorType,
    pub __sq_pushobject: sq_pushobjectType,
    pub __sq_getthisentity: sq_getthisentityType,
    pub __sq_getobject: sq_getobjectType,

    pub __sq_stackinfos: sq_stackinfosType,

    pub __sq_getstring: sq_getstringType,
    pub __sq_getinteger: sq_getintegerType,
    pub __sq_getfloat: sq_getfloatType,
    pub __sq_getbool: sq_getboolType,
    pub __sq_get: sq_getType,
    pub __sq_getasset: sq_getassetType,
    pub __sq_getuserdata: sq_getuserdataType,
    pub __sq_getvector: sq_getvectorType,
    pub __sq_createuserdata: sq_createuserdataType,
    pub __sq_setuserdatatypeid: sq_setuserdatatypeidType,
    pub __sq_getfunction: sq_getfunctionType,

    pub __sq_schedule_call_external: sq_schedule_call_externalType,
    pub __sq_getentityfrominstance: sq_getentityfrominstanceType,
    pub __sq_GetEntityConstant_CBaseEntity: sq_GetEntityConstantType,
}
#[test]
fn bindgen_test_layout_SquirrelFunctions() {
    const UNINIT: ::std::mem::MaybeUninit<SquirrelFunctions> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SquirrelFunctions>(),
        272usize,
        concat!("Size of: ", stringify!(SquirrelFunctions))
    );
    assert_eq!(
        ::std::mem::align_of::<SquirrelFunctions>(),
        8usize,
        concat!("Alignment of ", stringify!(SquirrelFunctions))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).RegisterSquirrelFunc) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(RegisterSquirrelFunc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_defconst) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_defconst)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_compilebuffer) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_compilebuffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_call) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_call)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_raiseerror) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_raiseerror)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_newarray) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_newarray)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_arrayappend) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_arrayappend)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_newtable) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_newtable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_newslot) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_newslot)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_pushroottable) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_pushroottable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_pushstring) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_pushstring)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_pushinteger) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_pushinteger)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_pushfloat) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_pushfloat)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_pushbool) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_pushbool)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_pushasset) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_pushasset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_pushvector) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_pushvector)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_pushobject) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_pushobject)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_getthisentity) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_getthisentity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_getobject) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_getobject)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_stackinfos) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_stackinfos)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_getstring) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_getstring)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_getinteger) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_getinteger)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_getfloat) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_getfloat)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_getbool) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_getbool)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_get) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_get)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_getasset) as usize - ptr as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_getasset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_getuserdata) as usize - ptr as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_getuserdata)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_getvector) as usize - ptr as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_getvector)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_createuserdata) as usize - ptr as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_createuserdata)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_setuserdatatypeid) as usize - ptr as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_setuserdatatypeid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_getfunction) as usize - ptr as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_getfunction)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_schedule_call_external) as usize - ptr as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_schedule_call_external)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sq_getentityfrominstance) as usize - ptr as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_getentityfrominstance)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).__sq_GetEntityConstant_CBaseEntity) as usize - ptr as usize
        },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(SquirrelFunctions),
            "::",
            stringify!(__sq_GetEntityConstant_CBaseEntity)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MessageSource {
    pub file: *const ::std::os::raw::c_char,
    pub func: *const ::std::os::raw::c_char,
    pub line: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_MessageSource() {
    const UNINIT: ::std::mem::MaybeUninit<MessageSource> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<MessageSource>(),
        24usize,
        concat!("Size of: ", stringify!(MessageSource))
    );
    assert_eq!(
        ::std::mem::align_of::<MessageSource>(),
        8usize,
        concat!("Alignment of ", stringify!(MessageSource))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).file) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MessageSource),
            "::",
            stringify!(file)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).func) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MessageSource),
            "::",
            stringify!(func)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).line) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MessageSource),
            "::",
            stringify!(line)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LogMsg {
    pub level: ::std::os::raw::c_int,
    pub timestamp: u64,
    pub msg: *const ::std::os::raw::c_char,
    pub source: MessageSource,
    pub pluginHandle: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_LogMsg() {
    const UNINIT: ::std::mem::MaybeUninit<LogMsg> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<LogMsg>(),
        56usize,
        concat!("Size of: ", stringify!(LogMsg))
    );
    assert_eq!(
        ::std::mem::align_of::<LogMsg>(),
        8usize,
        concat!("Alignment of ", stringify!(LogMsg))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).level) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(LogMsg),
            "::",
            stringify!(level)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).timestamp) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(LogMsg),
            "::",
            stringify!(timestamp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).msg) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(LogMsg),
            "::",
            stringify!(msg)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).source) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(LogMsg),
            "::",
            stringify!(source)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pluginHandle) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(LogMsg),
            "::",
            stringify!(pluginHandle)
        )
    );
}
pub type loggerfunc_t = ::std::option::Option<unsafe extern "C" fn(msg: *mut LogMsg)>;
pub type PLUGIN_RELAY_INVITE_TYPE =
    ::std::option::Option<unsafe extern "C" fn(invite: *const ::std::os::raw::c_char)>;
pub type CreateObjectFunc =
    ::std::option::Option<unsafe extern "C" fn(type_: ObjectType) -> *mut ::std::os::raw::c_void>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PluginNorthstarData {
    pub version: *const ::std::os::raw::c_char,
    pub northstarModule: *mut ::std::os::raw::c_void,
    pub pluginHandle: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_PluginNorthstarData() {
    const UNINIT: ::std::mem::MaybeUninit<PluginNorthstarData> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<PluginNorthstarData>(),
        24usize,
        concat!("Size of: ", stringify!(PluginNorthstarData))
    );
    assert_eq!(
        ::std::mem::align_of::<PluginNorthstarData>(),
        8usize,
        concat!("Alignment of ", stringify!(PluginNorthstarData))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).version) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginNorthstarData),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).northstarModule) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginNorthstarData),
            "::",
            stringify!(northstarModule)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pluginHandle) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginNorthstarData),
            "::",
            stringify!(pluginHandle)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PluginInitFuncs {
    pub logger: loggerfunc_t,
    pub relayInviteFunc: PLUGIN_RELAY_INVITE_TYPE,
    pub createObject: CreateObjectFunc,
}
#[test]
fn bindgen_test_layout_PluginInitFuncs() {
    const UNINIT: ::std::mem::MaybeUninit<PluginInitFuncs> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<PluginInitFuncs>(),
        24usize,
        concat!("Size of: ", stringify!(PluginInitFuncs))
    );
    assert_eq!(
        ::std::mem::align_of::<PluginInitFuncs>(),
        8usize,
        concat!("Alignment of ", stringify!(PluginInitFuncs))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).logger) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginInitFuncs),
            "::",
            stringify!(logger)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).relayInviteFunc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginInitFuncs),
            "::",
            stringify!(relayInviteFunc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).createObject) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginInitFuncs),
            "::",
            stringify!(createObject)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PluginEngineData {
    pub ConCommandConstructor: *mut ::std::os::raw::c_void,
    pub conVarMalloc: *mut ::std::os::raw::c_void,
    pub conVarRegister: *mut ::std::os::raw::c_void,
    pub ConVar_Vtable: *mut ::std::os::raw::c_void,
    pub IConVar_Vtable: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_PluginEngineData() {
    const UNINIT: ::std::mem::MaybeUninit<PluginEngineData> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<PluginEngineData>(),
        40usize,
        concat!("Size of: ", stringify!(PluginEngineData))
    );
    assert_eq!(
        ::std::mem::align_of::<PluginEngineData>(),
        8usize,
        concat!("Alignment of ", stringify!(PluginEngineData))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ConCommandConstructor) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginEngineData),
            "::",
            stringify!(ConCommandConstructor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).conVarMalloc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginEngineData),
            "::",
            stringify!(conVarMalloc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).conVarRegister) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginEngineData),
            "::",
            stringify!(conVarRegister)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ConVar_Vtable) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginEngineData),
            "::",
            stringify!(ConVar_Vtable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IConVar_Vtable) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginEngineData),
            "::",
            stringify!(IConVar_Vtable)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PluginGameStatePresence {
    pub id: *const ::std::os::raw::c_char,
    pub name: *const ::std::os::raw::c_char,
    pub description: *const ::std::os::raw::c_char,
    pub password: *const ::std::os::raw::c_char,
    pub is_server: bool,
    pub is_local: bool,
    pub state: GameState,
    pub map: *const ::std::os::raw::c_char,
    pub map_displayname: *const ::std::os::raw::c_char,
    pub playlist: *const ::std::os::raw::c_char,
    pub playlist_displayname: *const ::std::os::raw::c_char,
    pub current_players: ::std::os::raw::c_int,
    pub max_players: ::std::os::raw::c_int,
    pub own_score: ::std::os::raw::c_int,
    pub other_highest_score: ::std::os::raw::c_int,
    pub max_score: ::std::os::raw::c_int,
    pub timestamp_end: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_PluginGameStatePresence() {
    const UNINIT: ::std::mem::MaybeUninit<PluginGameStatePresence> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<PluginGameStatePresence>(),
        96usize,
        concat!("Size of: ", stringify!(PluginGameStatePresence))
    );
    assert_eq!(
        ::std::mem::align_of::<PluginGameStatePresence>(),
        8usize,
        concat!("Alignment of ", stringify!(PluginGameStatePresence))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).description) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(description)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).password) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(password)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).is_server) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(is_server)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).is_local) as usize - ptr as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(is_local)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).state) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).map) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(map)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).map_displayname) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(map_displayname)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).playlist) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(playlist)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).playlist_displayname) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(playlist_displayname)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).current_players) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(current_players)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_players) as usize - ptr as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(max_players)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).own_score) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(own_score)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).other_highest_score) as usize - ptr as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(other_highest_score)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_score) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(max_score)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).timestamp_end) as usize - ptr as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(PluginGameStatePresence),
            "::",
            stringify!(timestamp_end)
        )
    );
}
#[doc = " <summary> Async communication within the plugin system\n Due to the asynchronous nature of plugins, combined with the limitations of multi-compiler support\n and the custom memory allocator used by r2, is it difficult to safely get data across DLL boundaries\n from Northstar to plugin unless Northstar can get memory-clear signal back.\n To do this, we use a request-response system\n This means that if a plugin wants a piece of data, it will send a request to Northstar in the form of an\n PLUGIN_REQUESTS_[DATA]_DATA call. The first argument to this call is a function pointer to call to return the data\n Northstar will then, when possible, construct the requested data and call the function\n This ensures that the process blocks until the data is ingested, and means it can safely be deleted afterwards without risk of dangling\n pointers On the plugin side, the data should be ingested into a class guarded by mutexes The provided Plugin Library will handle all\n this automatically.\n </summary>"]
pub type PLUGIN_INIT_TYPE = ::std::option::Option<
    unsafe extern "C" fn(funcs: *mut PluginInitFuncs, data: *mut PluginNorthstarData),
>;
pub type PLUGIN_INIT_SQVM_TYPE =
    ::std::option::Option<unsafe extern "C" fn(funcs: *mut SquirrelFunctions)>;
pub type PLUGIN_INFORM_SQVM_CREATED_TYPE =
    ::std::option::Option<unsafe extern "C" fn(context: ScriptContext, sqvm: *mut CSquirrelVM)>;
pub type PLUGIN_INFORM_SQVM_DESTROYED_TYPE =
    ::std::option::Option<unsafe extern "C" fn(context: ScriptContext)>;
pub type PLUGIN_PUSH_PRESENCE_TYPE =
    ::std::option::Option<unsafe extern "C" fn(data: *mut PluginGameStatePresence)>;
pub type PLUGIN_INFORM_DLL_LOAD_TYPE = ::std::option::Option<
    unsafe extern "C" fn(dll: PluginLoadDLL, data: *mut ::std::os::raw::c_void),
>;