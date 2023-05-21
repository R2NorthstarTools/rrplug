/// marco used to generate the entry for your plugin
///
/// takes in a struct that implements the [`crate::plugin::Plugin`] trait.
#[macro_export]
macro_rules! entry {
    ( $func:ty ) => {
        #[doc(hidden)]
        use $crate::bindings::{plugin_abi, squirrelclasstypes, squirreldatatypes};
        use $crate::log;
        #[doc(hidden)]
        use $crate::wrappers::{northstar, squirrel};

        static PLUGIN: $crate::OnceCell<$func> = $crate::OnceCell::new();

        #[no_mangle]
        #[export_name = "PLUGIN_INIT"]
        extern "C" fn plugin_init(
            plugin_init_funcs: *const plugin_abi::PluginInitFuncs,
            plugin_northstar_data: *const plugin_abi::PluginNorthstarData,
        ) {
            let mut plugin: $func = $crate::plugin::Plugin::new();

            let plugin_data = unsafe {
                $crate::wrappers::northstar::PluginData::new(
                    plugin_init_funcs,
                    plugin_northstar_data,
                )
            };

            plugin_data.init_logger();
            log::info!("plugin logging initialized");

            plugin.initialize(&plugin_data);

            _ = PLUGIN.set(plugin);

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
                    return;
                }
            };

            _ = squirrel::SQFUNCTIONS.client.set((*funcs).into())
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
                    return;
                }
            };

            _ = squirrel::SQFUNCTIONS.server.set((*funcs).into())
        }

        #[no_mangle]
        #[export_name = "PLUGIN_INFORM_SQVM_CREATED"]
        extern "C" fn plugin_inform_sqvm_created(
            context: squirrelclasstypes::ScriptContext,
            sqvm: *mut squirreldatatypes::CSquirrelVM,
        ) {
            let context = std::convert::Into::<northstar::ScriptVmType>::into(context);
            log::info!("PLUGIN_INFORM_SQVM_CREATED called {}", context);

            let mut locked_register_functions = squirrel::FUNCTION_SQ_REGISTER.lock();

            let sq_functions = match context {
                northstar::ScriptVmType::Server => squirrel::SQFUNCTIONS.server.wait(),
                northstar::ScriptVmType::Client => squirrel::SQFUNCTIONS.client.wait(),
                northstar::ScriptVmType::Ui => squirrel::SQFUNCTIONS.client.wait(),
                _ => {
                    log::error!("invalid ScriptContext");
                    return;
                }
            };

            let sq_register_func = sq_functions.register_squirrel_func;

            for (cpp_func_name, sq_func_name, types, returntype, _, func) in
                locked_register_functions
                    .iter_mut()
                    .map(|f| f())
                    .filter(|info| info.4.is_right_vm(&context))
            {
                log::info!("Registering {context} function {sq_func_name} with types: {types}"); // TODO: context int to str

                let esq_returntype = match returntype.split('<').collect::<Vec<&str>>()[0] {
                    "bool" => squirrelclasstypes::eSQReturnType_Boolean,
                    "float" => squirrelclasstypes::eSQReturnType_Float,
                    "vector" => squirrelclasstypes::eSQReturnType_Vector,
                    "int" => squirrelclasstypes::eSQReturnType_Integer,
                    "entity" => squirrelclasstypes::eSQReturnType_Entity,
                    "string" => squirrelclasstypes::eSQReturnType_String,
                    "array" => squirrelclasstypes::eSQReturnType_Arrays,
                    "asset" => squirrelclasstypes::eSQReturnType_Asset,
                    "table" => squirrelclasstypes::eSQReturnType_Table,
                    "void" => squirrelclasstypes::eSQReturnType_Default,
                    "var" => squirrelclasstypes::eSQReturnType_Default,
                    _ => squirrelclasstypes::eSQReturnType_Default,
                };

                // shouldn't be unwraping here but I will say : why did you name your function like this?
                let sq_func_name = Box::new(std::ffi::CString::new(sq_func_name).unwrap());
                let help_test = Box::new(std::ffi::CString::new("what help").unwrap());
                let cpp_func_name = Box::new(std::ffi::CString::new(cpp_func_name).unwrap());
                let returntype = Box::new(std::ffi::CString::new(returntype).unwrap());
                let types = Box::new(std::ffi::CString::new(types).unwrap());

                let sq_func_name_ptr = Box::leak(sq_func_name).as_ptr();
                let cpp_func_name_ptr = Box::leak(cpp_func_name).as_ptr();
                let help_test_ptr = Box::leak(help_test).as_ptr();
                let returntype_ptr = Box::leak(returntype).as_ptr();
                let types_ptr = Box::leak(types).as_ptr();

                let reg = Box::new(std::mem::MaybeUninit::<
                    squirrelclasstypes::SQFuncRegistration,
                >::zeroed());
                let struct_ptr = Box::leak(reg).as_mut_ptr();

                unsafe {
                    std::ptr::addr_of_mut!((*struct_ptr).squirrelFuncName).write(sq_func_name_ptr);
                    std::ptr::addr_of_mut!((*struct_ptr).cppFuncName).write(cpp_func_name_ptr);
                    std::ptr::addr_of_mut!((*struct_ptr).helpText).write(help_test_ptr);
                    std::ptr::addr_of_mut!((*struct_ptr).returnTypeString).write(returntype_ptr);
                    std::ptr::addr_of_mut!((*struct_ptr).returnType).write(esq_returntype);
                    std::ptr::addr_of_mut!((*struct_ptr).argTypes).write(types_ptr);
                    std::ptr::addr_of_mut!((*struct_ptr).funcPtr).write(func);
                };

                debug_assert!(!sq_func_name_ptr.is_null());
                debug_assert!(!cpp_func_name_ptr.is_null());
                debug_assert!(!help_test_ptr.is_null());
                debug_assert!(!returntype_ptr.is_null());
                debug_assert!(!types_ptr.is_null());
                debug_assert!(!struct_ptr.is_null());
                debug_assert!(!sqvm.is_null());

                unsafe {
                    sq_register_func(sqvm, struct_ptr, 1);

                    _ = Box::from_raw(struct_ptr);
                    _ = *std::ffi::CStr::from_ptr(sq_func_name_ptr);
                    _ = *std::ffi::CStr::from_ptr(cpp_func_name_ptr);
                    _ = *std::ffi::CStr::from_ptr(help_test_ptr);
                    _ = *std::ffi::CStr::from_ptr(returntype_ptr);
                    _ = *std::ffi::CStr::from_ptr(types_ptr);
                }
            }

            let handle =
                $crate::wrappers::squirrel::CSquirrelVMHandle::<<$func as Plugin>::SaveType>::new(
                    sqvm, context,
                );

            PLUGIN.wait().on_sqvm_created(&handle);
        }

        #[no_mangle]
        #[export_name = "PLUGIN_INFORM_SQVM_DESTROYED"]
        extern "C" fn plugin_inform_sqvm_destroyed(context: squirrelclasstypes::ScriptContext) {
            let context = std::convert::Into::<northstar::ScriptVmType>::into(context);
            PLUGIN.wait().on_sqvm_destroyed(context);
        }

        #[no_mangle]
        #[export_name = "PLUGIN_INFORM_DLL_LOAD"]
        extern "C" fn plugin_inform_dll_load(
            dll: plugin_abi::PluginLoadDLL,
            data: *const ::std::os::raw::c_void,
        ) {
            match dll {
                plugin_abi::PluginLoadDLL_ENGINE => unsafe {
                    let engine_dll: *const plugin_abi::PluginEngineData = std::mem::transmute(data);
                    let engine_result = match engine_dll.as_ref() {
                        Some(engine_dll) => {
                            match $crate::wrappers::engine::ENGINE_DATA
                                .set($crate::wrappers::engine::EngineData::new(&*engine_dll))
                            {
                                // maybe use as_ref later
                                Ok(_) => northstar::EngineLoadType::Engine(
                                    $crate::wrappers::engine::ENGINE_DATA.wait(),
                                ),
                                Err(_) => northstar::EngineLoadType::EngineFailed,
                            }
                        }
                        None => northstar::EngineLoadType::EngineFailed,
                    };
                    PLUGIN.wait().on_engine_load(&engine_result)
                },
                plugin_abi::PluginLoadDLL_SERVER => PLUGIN
                    .wait()
                    .on_engine_load(&northstar::EngineLoadType::Server),
                plugin_abi::PluginLoadDLL_CLIENT => PLUGIN
                    .wait()
                    .on_engine_load(&northstar::EngineLoadType::Client),
                _ => log::warn!(
                    "PLUGIN_INFORM_DLL_LOAD called with unknown PluginLoadDLL type {dll}"
                ),
            }
        }

        #[no_mangle]
        #[export_name = "PLUGIN_RECEIVE_PRESENCE"]
        extern "C" fn plugin_receive_presence(
            presence: *const plugin_abi::PluginGameStatePresence,
        ) {
            match $crate::wrappers::presence::GamePresence::new(presence) {
                Ok(presence) => PLUGIN.wait().on_presence_updated(&presence),
                Err(_) => {}
            }
        }
    };
}

#[cfg(test)]
mod test_entry {
    use crate::prelude::*;


    pub struct Test;

    impl Plugin for Test {
        type SaveType = squirrel::Save;

        fn new() -> Self {
            Self {}
        }

        fn initialize(&mut self, _plugin_data: &PluginData) {}

        fn main(&self) {}
    }

    entry!(Test);

    #[test]
    fn test_init() {
        // todo: somehow test all the functions
    }
}
