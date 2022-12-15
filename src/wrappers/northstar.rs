//! wrappers for structs that are passed to the plugin

use log::SetLoggerError;
use once_cell::unsync::Lazy;
use std::ffi::CString;
use std::fmt::Display;
use std::sync::Mutex;

use super::engine::EngineCallbacks;
use super::errors::SqFunctionError;
use super::squrriel::FUNCTION_SQ_REGISTER;
use crate::bindings::plugin_abi::{PluginEngineData, PluginInitFuncs, PluginNorthstarData};
use crate::bindings::squirrelclasstypes::{
    eSQReturnType_Arrays, eSQReturnType_Asset, eSQReturnType_Boolean, eSQReturnType_Default,
    eSQReturnType_Entity, eSQReturnType_Float, eSQReturnType_Integer, eSQReturnType_String,
    eSQReturnType_Table, eSQReturnType_Vector, SQFuncRegistration, SQFunction,
    ScriptContext_CLIENT, ScriptContext_SERVER, ScriptContext_UI,
};
use crate::nslog;

static mut EXTERNAL_BUFFER: Lazy<Vec<u32>> = Lazy::new(|| vec![0_u32; 1000]);

pub type SQFuncInfo = fn() -> (&'static str, &'static str, &'static str, SQFunction);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScriptVmType {
    Server,
    Client,
    Ui,
    UiClient,
}

impl Display for ScriptVmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({self:?})")
    }
}

impl ScriptVmType {
    pub fn to_int(&self) -> i32 {
        match self {
            Self::Server => ScriptContext_SERVER,
            Self::Client => ScriptContext_CLIENT,
            Self::Ui => ScriptContext_UI,
            Self::UiClient => ScriptContext_UI,
        }
    }
}

pub struct PluginData {
    plugin_init_funcs: PluginInitFuncs,
    plugin_northstar_data: PluginNorthstarData,
    engine_callbacks: &'static mut Option<Mutex<EngineCallbacks>>,
}

impl PluginData {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new(
        plugin_init_funcs: *const PluginInitFuncs,
        plugin_northstar_data: *const PluginNorthstarData,
        engine_callbacks: &'static mut Option<Mutex<EngineCallbacks>>,
    ) -> Self {
        Self {
            plugin_init_funcs: *plugin_init_funcs,
            plugin_northstar_data: *plugin_northstar_data,
            engine_callbacks,
        }
    }

    /// logging is already initialized in the entry marco by default
    pub fn try_init_logger(&self) -> Result<(), SetLoggerError> {
        nslog::try_init(
            self.plugin_init_funcs.logger,
            self.plugin_northstar_data.pluginHandle,
        )
    }

    /// logging is already initialized in the entry marco by default
    pub fn init_logger(&self) {
        self.try_init_logger().unwrap();
    }

    pub fn get_northstar_version(&self) -> i8 {
        unsafe { *self.plugin_northstar_data.version }
    }

    pub fn get_plugin_handle(&self) -> i32 {
        self.plugin_northstar_data.pluginHandle
    }

    pub fn add_engine_load_callback(&self, callback: Box<dyn Fn(PluginEngineData)>) {
        let mut engine_callbacks = match self.engine_callbacks.as_ref().unwrap().try_lock() {
            Ok(engine_callbacks) => engine_callbacks,
            Err(err) => {
                log::error!("failed to add engine callbacks because of {err:?}");
                return;
            }
        };
        engine_callbacks.add_callback(callback);
    }

    pub fn register_sq_functions(&self, get_info_func: SQFuncInfo) -> Result<(), SqFunctionError> {
        let to_cstring = |s: &str| CString::new(s).unwrap();

        let capacity = unsafe { EXTERNAL_BUFFER.capacity() };
        let ptr = unsafe { EXTERNAL_BUFFER.as_mut_ptr() };

        let (cpp_func_name, sq_func_name, types, func) = get_info_func();

        log::warn!("registing function {sq_func_name} with {types}");

        let returntype = "int";

        let esq_returntype = match returntype {
            "bool" => eSQReturnType_Boolean,
            "float" => eSQReturnType_Float,
            "vector" => eSQReturnType_Vector,
            "int" => eSQReturnType_Integer,
            "entity" => eSQReturnType_Entity,
            "string" => eSQReturnType_String,
            "array" => eSQReturnType_Arrays,
            "asset" => eSQReturnType_Asset,
            "table" => eSQReturnType_Table,
            _ => eSQReturnType_Default,
        };

        let sqfunction_registration = SQFuncRegistration {
            squirrelFuncName: to_cstring(sq_func_name).as_ptr(),
            cppFuncName: to_cstring(cpp_func_name).as_ptr(),
            helpText: to_cstring(sq_func_name).as_ptr(),
            returnTypeString: to_cstring("void").as_ptr(),
            argTypes: to_cstring(types).as_ptr(),
            unknown1: 0,
            devLevel: 0,
            shortNameMaybe: to_cstring(sq_func_name).as_ptr(),
            unknown2: 0,
            returnType: esq_returntype,
            externalBufferPointer: ptr,
            externalBufferSize: capacity.try_into().unwrap(),
            unknown3: 0,
            unknown4: 0,
            funcPtr: func,
        };

        match unsafe { FUNCTION_SQ_REGISTER.try_lock() } {
            Ok(mut sq_function_vec) => {
                sq_function_vec.push(sqfunction_registration);
                Ok(())
            }
            Err(_) => Err(SqFunctionError::LockedSqFunctionVec),
        }
    }
}
