/// marco used to generate the entry for your plugin
///
/// takes in a struct that implements the [`crate::plugin::Plugin`] trait.
#[macro_export]
macro_rules! entry {
    ( $plugin:ident ) => {
        #[allow(unused_imports)]
        #[doc(hidden)]
        pub(crate) use exports::PLUGIN;
        #[doc(hidden)]
        pub(crate) mod exports {
            use super::$plugin;

            use $crate::plugin::Plugin;
            use $crate::bindings::{plugin_abi, squirrelclasstypes, squirreldatatypes};
            use $crate::log;
            use $crate::{high,mid};
            use mid::squirrel::SQFUNCTIONS;
            use high::{northstar::ScriptVmType,engine::EngineData};

            use std::ffi::CString;

            pub static PLUGIN: $crate::OnceCell<$plugin> = $crate::OnceCell::new();

            #[no_mangle]
            #[export_name = "PLUGIN_INIT"]
            extern "C" fn plugin_init(
                plugin_init_funcs: *const plugin_abi::PluginInitFuncs,
                plugin_northstar_data: *const plugin_abi::PluginNorthstarData,
            ) {
                let plugin_data = unsafe {
                    $crate::high::northstar::PluginData::new(
                        plugin_init_funcs,
                        plugin_northstar_data,
                    )
                };

                plugin_data.init_logger();
                log::info!("plugin logging initialized");

                let plugin: $plugin = $crate::plugin::Plugin::new(&plugin_data);

                if PLUGIN.set(plugin).is_err() {
                    panic!("PLUGIN failed initialization")
                }

                log::info!( "plugin static initialized : {}", PLUGIN.get().is_some() );

                std::thread::spawn(move || PLUGIN.wait().main());
            }

            #[no_mangle]
            #[export_name = "PLUGIN_INIT_SQVM_CLIENT"]
            fn plugin_init_sqvm_client(funcs: *const plugin_abi::SquirrelFunctions) {
                let funcs = match unsafe { funcs.as_ref() } {
                    Some(funcs) => funcs,
                    None => {
                        log::error!(
                            "failed to get SquirrelFunctions from ptr in PLUGIN_INIT_SQVM_CLIENT"
                        );
                        panic!("failed to get SquirrelFunctions from ptr in PLUGIN_INIT_SQVM_CLIENT")
                    }
                };

                SQFUNCTIONS.client.set((*funcs).into()).expect("SQFUNCTIONS.client should be initialized once");
                log::debug!("Client SquirrelFunctions acquired!");
            }

            #[no_mangle]
            #[export_name = "PLUGIN_INIT_SQVM_SERVER"]
            fn plugin_init_sqvm_server(funcs: *const plugin_abi::SquirrelFunctions) {
                let funcs = match unsafe { funcs.as_ref() } {
                    Some(funcs) => funcs,
                    None => {
                        log::error!(
                            "failed to get SquirrelFunctions from ptr in PLUGIN_INIT_SQVM_SERVER"
                        );
                        panic!("failed to get SquirrelFunctions from ptr in PLUGIN_INIT_SQVM_SERVER")
                    }
                };



                SQFUNCTIONS.server.set((*funcs).into()).expect("SQFUNCTIONS.server should be initialized once");
                log::debug!("Server SquirrelFunctions acquired!");
            }

            #[no_mangle]
            #[export_name = "PLUGIN_INFORM_SQVM_CREATED"]
            extern "C" fn plugin_inform_sqvm_created(
                context: squirrelclasstypes::ScriptContext,
                sqvm: *mut squirreldatatypes::CSquirrelVM,
            ) {
                let context = std::convert::Into::<high::northstar::ScriptVmType>::into(context);
                log::info!("PLUGIN_INFORM_SQVM_CREATED called {}", context);

                let mut locked_register_functions = high::squirrel::FUNCTION_SQ_REGISTER.lock();

                let sq_functions = match context {
                    ScriptVmType::Server => SQFUNCTIONS.server.get(),
                    ScriptVmType::Client => SQFUNCTIONS.client.get(),
                    ScriptVmType::Ui => SQFUNCTIONS.client.get(),
                    _ => {
                        log::error!("invalid ScriptContext");
                        return;
                    }
                }.expect("SQFUNCTIONS should be initialized at this point");

                let sq_register_func = sq_functions.register_squirrel_func;

                for func_info in
                    locked_register_functions
                        .iter_mut()
                        .map(|f| f())
                        .filter(|info| info.vm.is_right_vm(&context))
                {
                    log::info!("Registering {context} function {} with types: {}", func_info.sq_func_name, func_info.types);

                    let esq_returntype = match func_info.return_type.split('<').collect::<Vec<&str>>()[0] {
                        "bool" => squirrelclasstypes::eSQReturnType::Boolean,
                        "float" => squirrelclasstypes::eSQReturnType::Float,
                        "vector" => squirrelclasstypes::eSQReturnType::Vector,
                        "int" => squirrelclasstypes::eSQReturnType::Integer,
                        "entity" => squirrelclasstypes::eSQReturnType::Entity,
                        "string" => squirrelclasstypes::eSQReturnType::String,
                        "array" => squirrelclasstypes::eSQReturnType::Arrays,
                        "asset" => squirrelclasstypes::eSQReturnType::Asset,
                        "table" => squirrelclasstypes::eSQReturnType::Table,
                        "void" => squirrelclasstypes::eSQReturnType::Default,
                        "var" => squirrelclasstypes::eSQReturnType::Default,
                        _ => squirrelclasstypes::eSQReturnType::Default,
                    };

                    // shouldn't be unwraping here but I will say : why did you name your function like this?
                    let sq_func_name = CString::new(func_info.sq_func_name).unwrap();
                    let cpp_func_name = CString::new("what help").unwrap();
                    let help_test = CString::new(func_info.cpp_func_name).unwrap();
                    let returntype = CString::new(func_info.return_type).unwrap();
                    let types = CString::new(func_info.types).unwrap();

                    let mut reg = std::mem::MaybeUninit::<
                        squirrelclasstypes::SQFuncRegistration,
                    >::zeroed();
                    let struct_ptr = reg.as_mut_ptr();

                    unsafe {
                        std::ptr::addr_of_mut!((*struct_ptr).squirrelFuncName).write(sq_func_name.as_ptr());
                        std::ptr::addr_of_mut!((*struct_ptr).cppFuncName).write(cpp_func_name.as_ptr());
                        std::ptr::addr_of_mut!((*struct_ptr).helpText).write(help_test.as_ptr());
                        std::ptr::addr_of_mut!((*struct_ptr).returnTypeString).write(returntype.as_ptr());
                        std::ptr::addr_of_mut!((*struct_ptr).returnType).write(esq_returntype);
                        std::ptr::addr_of_mut!((*struct_ptr).argTypes).write(types.as_ptr());
                        std::ptr::addr_of_mut!((*struct_ptr).funcPtr).write(func_info.function);
                    };

                    debug_assert!(!struct_ptr.is_null());
                    debug_assert!(!sqvm.is_null());

                    unsafe {
                        sq_register_func(sqvm, struct_ptr, 1);
                    }
                }

                let handle =
                    high::squirrel::CSquirrelVMHandle::<<$plugin as Plugin>::SaveType>::new(
                        sqvm, context,
                    );

                PLUGIN.wait().on_sqvm_created(&handle);
            }

            #[no_mangle]
            #[export_name = "PLUGIN_INFORM_SQVM_DESTROYED"]
            extern "C" fn plugin_inform_sqvm_destroyed(context: squirrelclasstypes::ScriptContext) {
                let context = std::convert::Into::<ScriptVmType>::into(context);
                PLUGIN.wait().on_sqvm_destroyed(context);
            }

            #[no_mangle]
            #[export_name = "PLUGIN_INFORM_DLL_LOAD"]
            extern "C" fn plugin_inform_dll_load(
                dll: plugin_abi::PluginLoadDLL,
                data: *mut ::std::os::raw::c_void,
                dll_ptr: *mut ::std::os::raw::c_void,
            ) {
                match dll {
                    plugin_abi::PluginLoadDLL::ENGINE => unsafe {
                        let engine_dll: *const plugin_abi::PluginEngineData = std::mem::transmute(data);
                        let engine_result = match engine_dll.as_ref() {
                            Some(engine_dll) => {
                                match mid::engine::ENGINE_DATA
                                    .set(EngineData::new(&*engine_dll))
                                {
                                    // maybe use as_ref later
                                    Ok(_) => high::northstar::EngineLoadType::Engine(
                                        mid::engine::ENGINE_DATA.wait(),
                                    ),
                                    Err(_) => high::northstar::EngineLoadType::EngineFailed,
                                }
                            }
                            None => high::northstar::EngineLoadType::EngineFailed,
                        };
                        PLUGIN.wait().on_engine_load(&engine_result, dll_ptr)
                    },
                    plugin_abi::PluginLoadDLL::SERVER => PLUGIN
                    .wait()
                        .on_engine_load(&high::northstar::EngineLoadType::Server, dll_ptr),
                    plugin_abi::PluginLoadDLL::CLIENT => PLUGIN
                    .wait()
                        .on_engine_load(&high::northstar::EngineLoadType::Client, dll_ptr)
                }
            }

            #[no_mangle]
            #[export_name = "PLUGIN_RUNFRAME"]
            unsafe extern "C" fn plugin_runframe() {
                PLUGIN.wait().runframe();
            }
        }
    };
}

#[cfg(test)]
mod test_entry {
    use crate::prelude::*;

    pub struct Test;

    impl Plugin for Test {
        type SaveType = Save;

        fn new(_plugin_data: &PluginData) -> Self {
            Self {}
        }

        fn main(&self) {}
    }

    entry!(Test);

    #[test]
    fn test_init() {
        // todo: somehow test all the functions
    }
}
