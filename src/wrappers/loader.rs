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
use std::{ffi::CString, mem, ptr::addr_of_mut};

pub static mut ENGINE_CALLBACKS: Lazy<std::sync::Mutex<EngineCallbacks>> =
    Lazy::new(|| std::sync::Mutex::new(EngineCallbacks::default()));

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

    for get_info_func in locked_register_functions.iter_mut() {
        let (cpp_func_name, sq_func_name, types, func) = dbg!(get_info_func());

        log::info!("Registering {context} function {sq_func_name} with types: {types}"); // TODO: context int to str

        let returntype = "void";

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
            _ => {
                log::info!("undefined return type choosing eSQReturnType_Default");
                squirrelclasstypes::eSQReturnType_Default
            }
        };

        let sq_func_name = to_cstring(sq_func_name);
        let help_test = to_cstring("what help");
        let cpp_func_name = to_cstring(cpp_func_name);
        let returntype = to_cstring(returntype);
        let types = to_cstring(types);

        let sq_func_name_ptr = sq_func_name.as_ptr();
        let cpp_func_name_ptr = cpp_func_name.as_ptr();
        let help_test_ptr = help_test.as_ptr();
        let returntype_ptr = returntype.as_ptr();
        let types_ptr = types.as_ptr();

        let mut un_init = Box::new(mem::MaybeUninit::<SQFuncRegistration>::zeroed());
        let struct_ptr = un_init.as_mut_ptr();

        debug_assert!(!sq_func_name_ptr.is_null());
        debug_assert!(!cpp_func_name_ptr.is_null());
        debug_assert!(!help_test_ptr.is_null());
        debug_assert!(!returntype_ptr.is_null());
        debug_assert!(!types_ptr.is_null());
        debug_assert!(!struct_ptr.is_null());
        debug_assert!(!sqvm.is_null());

        unsafe {
            addr_of_mut!((*struct_ptr).squirrelFuncName).write(sq_func_name_ptr);
            addr_of_mut!((*struct_ptr).squirrelFuncName).write(sq_func_name_ptr);
            addr_of_mut!((*struct_ptr).cppFuncName).write(cpp_func_name_ptr);
            addr_of_mut!((*struct_ptr).helpText).write(help_test_ptr);
            addr_of_mut!((*struct_ptr).returnTypeString).write(returntype_ptr);
            addr_of_mut!((*struct_ptr).returnType).write(esq_returntype);
            addr_of_mut!((*struct_ptr).argTypes).write(types_ptr);
            addr_of_mut!((*struct_ptr).funcPtr).write(func);
        };

        debug_assert!(!sq_func_name_ptr.is_null());
        debug_assert!(!cpp_func_name_ptr.is_null());
        debug_assert!(!help_test_ptr.is_null());
        debug_assert!(!returntype_ptr.is_null());
        debug_assert!(!types_ptr.is_null());
        debug_assert!(!struct_ptr.is_null());
        debug_assert!(!sqvm.is_null());
        debug_assert!(func.is_some());

        unsafe {
            sq_register_func(sqvm, struct_ptr, 1);
        }

        unsafe { un_init.assume_init_drop() }
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
