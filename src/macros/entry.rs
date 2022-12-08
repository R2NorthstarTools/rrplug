/// marco used to generate the entry for your plugin
///
/// takes in a struct that implements the [`crate::plugin::Plugin`] trait.
#[macro_export]
macro_rules! entry {
    ( $func:ty ) => {
        use $crate::bindings::plugin_abi::{PluginInitFuncs, PluginNorthstarData};

        static mut ENGINE_CALLBACKS: Option<std::sync::Mutex<$crate::wrappers::engine::EngineCallbacks>> = None;
        static mut SQVM_CALLBACKS: Option<std::sync::Mutex<$crate::wrappers::squrrielvm::SquirrelVMCallbacks>> = None;

        #[no_mangle]
        #[export_name = "PLUGIN_INIT"]
        extern "C" fn plugin_init(
            plugin_init_funcs: *const PluginInitFuncs,
            plugin_northstar_data: *const PluginNorthstarData,
        ) {
            let mut plugin: $func = $crate::plugin::Plugin::new();

            unsafe { ENGINE_CALLBACKS = Some(std::sync::Mutex::new($crate::wrappers::engine::EngineCallbacks::default())) }
            unsafe { SQVM_CALLBACKS = Some(std::sync::Mutex::new($crate::wrappers::squrrielvm::SquirrelVMCallbacks::default())) }

            let plugin_data = unsafe {
                $crate::wrappers::northstar::PluginData::new(
                    plugin_init_funcs,
                    plugin_northstar_data,
                    &mut ENGINE_CALLBACKS,
                    &mut SQVM_CALLBACKS
                )
            };



            plugin_data.init_logger();
            log::info!("plugin logging initialized");

            plugin.initialize(&plugin_data);

            std::thread::spawn(move || plugin.main());
        }

        #[no_mangle]
        #[export_name = "PLUGIN_INIT_SQVM_CLIENT"]
        fn plugin_init_sqvm_client( funcs: *const $crate::bindings::plugin_abi::SquirrelFunctions ) {
            let funcs = $crate::wrappers::squrrielvm::SqFunctions::Client(unsafe {*funcs});

            match unsafe {SQVM_CALLBACKS.as_ref().unwrap().try_lock()} {
                Ok(sqvm_callbacks) => sqvm_callbacks.call_callbacks_init( funcs ),
                Err(err) => log::error!("calling sqvm client init callbacks failed: {err:?}")
            }
        }

        #[no_mangle]
        #[export_name = "PLUGIN_INIT_SQVM_SERVER"]
        fn plugin_init_sqvm_server(funcs: *const $crate::bindings::plugin_abi::SquirrelFunctions) {
            let funcs = $crate::wrappers::squrrielvm::SqFunctions::Server(unsafe {*funcs});

            match unsafe {SQVM_CALLBACKS.as_ref().unwrap().try_lock()} {
                Ok(sqvm_callbacks) => sqvm_callbacks.call_callbacks_init( funcs ),
                Err(err) => log::error!("calling sqvm server init callbacks failed: {err:?}")
            }
        }

        #[no_mangle]
        #[export_name = "PLUGIN_INFORM_SQVM_CREATED"]
        extern "C" fn plugin_inform_sqvm_created(
            context: $crate::bindings::squirrelclasstypes::ScriptContext,
            sqvm: *const $crate::bindings::squirreldatatypes::CSquirrelVM,
        ) {
            log::warn!("PLUGIN_INFORM_SQVM_CREATED called");
            let sqvm = match context {
                $crate::bindings::squirrelclasstypes::ScriptContext_SERVER => {
                    let sqvm = unsafe {*sqvm};
                    log::warn!(
                        "PLUGIN_INFORM_SQVM_CREATED got SERVER ScriptContext {:?}",
                        sqvm
                    );

                    $crate::wrappers::squrrielvm::ScriptVm::Server(sqvm)
                },
                $crate::bindings::squirrelclasstypes::ScriptContext_CLIENT => {
                    let sqvm = unsafe {*sqvm};
                    log::warn!(
                        "PLUGIN_INFORM_SQVM_CREATED got CLIENT ScriptContext {:?}",
                        sqvm
                    );

                    $crate::wrappers::squrrielvm::ScriptVm::Client(sqvm)
                },
                $crate::bindings::squirrelclasstypes::ScriptContext_UI => {
                    let sqvm = unsafe {*sqvm};
                    log::warn!(
                        "PLUGIN_INFORM_SQVM_CREATED got UI ScriptContext {:?}",
                        sqvm
                    );

                    $crate::wrappers::squrrielvm::ScriptVm::Ui(sqvm)
                },
                _ => {log::warn!(
                    "PLUGIN_INFORM_SQVM_CREATED called with unknown ScriptContext {context}"
                ); return;},
            };

            match unsafe {SQVM_CALLBACKS.as_ref().unwrap().try_lock()} {
                Ok(sqvm_callbacks) => sqvm_callbacks.call_callbacks_created( sqvm ),
                Err(err) => log::error!("calling sqvm created callbacks failed: {err:?}")
            }
        }

        #[no_mangle]
        #[export_name = "PLUGIN_INFORM_DLL_LOAD"]
        extern "C" fn plugin_inform_dll_load(
            dll: $crate::bindings::plugin_abi::PluginLoadDLL,
            data: *const ::std::os::raw::c_void,
        ) {
            match dll {
                $crate::bindings::plugin_abi::PluginLoadDLL_ENGINE => unsafe {
                    let engine_dll: *const $crate::bindings::plugin_abi::PluginEngineData =
                        std::mem::transmute(data);
                    let engine_dll = *engine_dll;
                    log::debug!("PLUGIN_INFORM_DLL_LOAD got a engine dll with data: {:?}", engine_dll);

                    match ENGINE_CALLBACKS.as_ref().unwrap().try_lock() {
                        Ok(engine_callbacks) => engine_callbacks.call_callbacks( engine_dll ),
                        Err(err) => log::error!("calling dll load callbacks failed: {err:?}")
                    }
                },
                _ => log::warn!(
                    "PLUGIN_INFORM_DLL_LOAD called with unknown PluginLoadDLL type {dll}"
                ),
            }
        }
    };
}
