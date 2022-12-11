#![allow(non_upper_case_globals)]

use crate::{
    bindings::{
        plugin_abi::{PluginEngineData, PluginLoadDLL, PluginLoadDLL_ENGINE, SquirrelFunctions},
        squirrelclasstypes::{
            ScriptContext, ScriptContext_CLIENT, ScriptContext_SERVER, ScriptContext_UI,
        },
        squirreldatatypes::CSquirrelVM,
    },
    wrappers::squrrielvm::ScriptVm,
};

use super::{
    engine::EngineCallbacks,
    squrrielvm::{SqFunctions, SquirrelVMCallbacks},
};

pub static mut ENGINE_CALLBACKS: Option<std::sync::Mutex<EngineCallbacks>> = None;
pub static mut SQVM_CALLBACKS: Option<std::sync::Mutex<SquirrelVMCallbacks>> = None;

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
    let funcs = SqFunctions::Client(funcs);

    match unsafe { SQVM_CALLBACKS.as_ref().unwrap().try_lock() } {
        Ok(sqvm_callbacks) => sqvm_callbacks.call_callbacks_init(funcs),
        Err(err) => log::error!("calling sqvm client init callbacks failed: {err:?}"),
    }
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
    let funcs = SqFunctions::Server( funcs );

    match unsafe { SQVM_CALLBACKS.as_ref().unwrap().try_lock() } {
        Ok(sqvm_callbacks) => sqvm_callbacks.call_callbacks_init(funcs),
        Err(err) => log::error!("calling sqvm server init callbacks failed: {err:?}"),
    }
}

#[no_mangle]
#[export_name = "PLUGIN_INFORM_SQVM_CREATED"]
extern "C" fn plugin_inform_sqvm_created(context: ScriptContext, sqvm: *const CSquirrelVM) {
    log::warn!("PLUGIN_INFORM_SQVM_CREATED called");
    let sqvm = match context {
        ScriptContext_SERVER => {
            let sqvm = match unsafe { sqvm.cast_mut().as_mut() } {
                Some(funcs) => funcs,
                None => {
                    log::error!("failed to get CSquirrelVM from ptr in PLUGIN_INFORM_SQVM_CREATED SERVER");
                    return;
                }
            };
            log::debug!(
                "PLUGIN_INFORM_SQVM_CREATED got SERVER ScriptContext {:?}",
                sqvm
            );

            ScriptVm::Server(sqvm)
        }
        ScriptContext_CLIENT => {
            let sqvm = match unsafe { sqvm.cast_mut().as_mut() } {
                Some(funcs) => funcs,
                None => {
                    log::error!("failed to get CSquirrelVM from ptr in PLUGIN_INFORM_SQVM_CREATED CLIENT");
                    return;
                }
            };
            log::debug!(
                "PLUGIN_INFORM_SQVM_CREATED got CLIENT ScriptContext {:?}",
                sqvm
            );

            ScriptVm::Client(sqvm)
        }
        ScriptContext_UI => {
            let sqvm = match unsafe { sqvm.cast_mut().as_mut() } {
                Some(funcs) => funcs,
                None => {
                    log::error!("failed to get CSquirrelVM from ptr in PLUGIN_INFORM_SQVM_CREATED UI");
                    return;
                }
            };
            log::debug!("PLUGIN_INFORM_SQVM_CREATED got UI ScriptContext {:?}", sqvm);

            ScriptVm::Ui(sqvm)
        }
        _ => {
            log::warn!("PLUGIN_INFORM_SQVM_CREATED called with unknown ScriptContext {context}");
            return;
        }
    };

    match unsafe { SQVM_CALLBACKS.as_ref().unwrap().try_lock() } {
        Ok(sqvm_callbacks) => sqvm_callbacks.call_callbacks_created(sqvm),
        Err(err) => log::error!("calling sqvm created callbacks failed: {err:?}"),
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