#![allow(non_upper_case_globals)]

use crate::{
    bindings::{
        plugin_abi::{PluginEngineData, PluginLoadDLL, PluginLoadDLL_ENGINE, SquirrelFunctions},
        squirrelclasstypes::{
            SQFuncRegistration, ScriptContext, ScriptContext_CLIENT, ScriptContext_SERVER,
            ScriptContext_UI,
        },
        squirreldatatypes::CSquirrelVM,
    },
    wrappers::squrriel::FUNCTION_SQ_REGISTER,
};

use super::{engine::EngineCallbacks, squrriel::SQFUNCTIONS};

pub static mut ENGINE_CALLBACKS: Option<std::sync::Mutex<EngineCallbacks>> = None;

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
extern "C" fn plugin_inform_sqvm_created(context: ScriptContext, sqvm: *const CSquirrelVM) {
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

    let sqvm = sqvm.cast_mut();

    for func in locked_register_functions.iter_mut() {
        unsafe {
            sq_register_func(sqvm, func as *mut SQFuncRegistration, 0);
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

            match ENGINE_CALLBACKS.as_ref().unwrap().try_lock() {
                Ok(engine_callbacks) => engine_callbacks.call_callbacks(engine_dll),
                Err(err) => log::error!("calling dll load callbacks failed: {err:?}"),
            }
        },
        _ => log::warn!("PLUGIN_INFORM_DLL_LOAD called with unknown PluginLoadDLL type {dll}"),
    }
}
