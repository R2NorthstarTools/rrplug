#![allow(non_upper_case_globals)]

use super::{engine::EngineCallbacks, squrriel::SQFUNCTIONS};
use crate::{
    bindings::{
        plugin_abi::{PluginEngineData, PluginLoadDLL, PluginLoadDLL_ENGINE, SquirrelFunctions},
        squirrelclasstypes::{
            self, SQFuncRegistration, ScriptContext, ScriptContext_CLIENT, ScriptContext_SERVER,
            ScriptContext_UI,
        },
        squirreldatatypes::CSquirrelVM,
    },
    wrappers::squrriel::FUNCTION_SQ_REGISTER,
};
use once_cell::sync::Lazy;
use std::ffi::CString;

pub static mut ENGINE_CALLBACKS: Lazy<std::sync::Mutex<EngineCallbacks>> =
    Lazy::new(|| std::sync::Mutex::new(EngineCallbacks::default()));
static mut EXTERNAL_BUFFER: Lazy<Vec<u32>> = Lazy::new(|| vec![0_u32; 1000]);

#[no_mangle]
#[export_name = "PLUGIN_INIT_SQVM_CLIENT"]
fn plugin_init_sqvm_client(funcs: *const SquirrelFunctions) {
    let funcs = match unsafe { funcs.as_ref() } {
        Some(funcs) => funcs,
        None => {
            log::error!("failed to get SquirrelFunctions from ptr in PLUGIN_INIT_SQVM_CLIENT");
            return;
        }
    };

    unsafe { SQFUNCTIONS.client = Some(funcs.into()) }
}

#[no_mangle]
#[export_name = "PLUGIN_INIT_SQVM_SERVER"]
fn plugin_init_sqvm_server(funcs: *const SquirrelFunctions) {
    let funcs = match unsafe { funcs.as_ref() } {
        Some(funcs) => funcs,
        None => {
            log::error!("failed to get SquirrelFunctions from ptr in PLUGIN_INIT_SQVM_SERVER");
            return;
        }
    };

    unsafe { SQFUNCTIONS.server = Some(funcs.into()) }
}

#[no_mangle]
#[export_name = "PLUGIN_INFORM_SQVM_CREATED"]
extern "C" fn plugin_inform_sqvm_created(context: ScriptContext, sqvm: *mut CSquirrelVM) {
    log::info!("PLUGIN_INFORM_SQVM_CREATED called {}", context);

    let mut locked_register_functions = loop {
        match unsafe { FUNCTION_SQ_REGISTER.try_lock() } {
            Ok(locked_sq_functions) => break locked_sq_functions,
            Err(err) => log::error!(
                "failed to get functions marked for REGISTER: {err:?}; retrying in a bit"
            ),
        }
    };

    let sq_functions = unsafe {
        match context {
            ScriptContext_SERVER => SQFUNCTIONS.server.as_ref().unwrap(),
            ScriptContext_CLIENT => SQFUNCTIONS.client.as_ref().unwrap(),
            ScriptContext_UI => SQFUNCTIONS.client.as_ref().unwrap(),
            _ => {
                log::error!("invalid ScriptContext");
                return;
            }
        }
    };

    let sq_register_func = sq_functions.register_squirrel_func;

    let to_cstring = |s: &str| CString::new(s).unwrap();

    let unkown_char = CString::new("whar").unwrap();

    for get_info_func in locked_register_functions.iter_mut() {
        let mut buffer = vec![0_u32; 10000]; // how large?
        // let capacity = unsafe { EXTERNAL_BUFFER.capacity() };
        // let ptr = unsafe { EXTERNAL_BUFFER.as_mut_ptr() };
        let capacity = buffer.capacity();
        let ptr = buffer.as_mut_ptr();

        let (cpp_func_name, sq_func_name, types, func) = get_info_func();

        log::info!("registing function {sq_func_name} with {types}");

        let returntype = "int";

        let esq_returntype = match returntype {
            "bool" => squirrelclasstypes::eSQReturnType_Boolean,
            "float" => squirrelclasstypes::eSQReturnType_Float,
            "vector" => squirrelclasstypes::eSQReturnType_Vector,
            "int" => squirrelclasstypes::eSQReturnType_Integer,
            "entity" => squirrelclasstypes::eSQReturnType_Entity,
            "string" => squirrelclasstypes::eSQReturnType_String,
            "array" => squirrelclasstypes::eSQReturnType_Arrays,
            "asset" => squirrelclasstypes::eSQReturnType_Asset,
            "table" => squirrelclasstypes::eSQReturnType_Table,
            _ => squirrelclasstypes::eSQReturnType_Default,
        };

        let func_short = to_cstring(sq_func_name);
        let sq_func_name = to_cstring(sq_func_name);
        let help_test = to_cstring("what help");
        let cpp_func_name = to_cstring(cpp_func_name);
        let returntype = to_cstring(returntype);
        let types = to_cstring(types);

        let mut sqfunction_registration = SQFuncRegistration {
            squirrelFuncName: sq_func_name.as_ptr(),
            cppFuncName: cpp_func_name.as_ptr(),
            helpText: help_test.as_ptr(),
            returnTypeString: returntype.as_ptr(),
            argTypes: types.as_ptr(),
            unknown1: 0,
            devLevel: 0,
            shortNameMaybe: func_short.as_ptr(),
            unknown2: 0,
            returnType: esq_returntype,
            externalBufferPointer: ptr,
            externalBufferSize: capacity.try_into().unwrap(),
            unknown3: 0,
            unknown4: 0,
            funcPtr: func,
        };

        unsafe {
            sq_register_func(
                sqvm,
                &mut sqfunction_registration,
                *unkown_char.as_ptr(),
            );
        }
    }
}

#[no_mangle]
#[export_name = "PLUGIN_INFORM_DLL_LOAD"]
extern "C" fn plugin_inform_dll_load(dll: PluginLoadDLL, data: *const ::std::os::raw::c_void) {
    match dll {
        PluginLoadDLL_ENGINE => unsafe {
            let engine_dll: *const PluginEngineData = std::mem::transmute(data);
            let engine_dll = *engine_dll;
            log::debug!(
                "PLUGIN_INFORM_DLL_LOAD got a engine dll with data: {:?}",
                engine_dll
            );

            match ENGINE_CALLBACKS.try_lock() {
                Ok(engine_callbacks) => engine_callbacks.call_callbacks(engine_dll),
                Err(err) => log::error!("calling dll load callbacks failed: {err:?}"),
            }
        },
        _ => log::warn!("PLUGIN_INFORM_DLL_LOAD called with unknown PluginLoadDLL type {dll}"),
    }
}
