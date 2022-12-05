/// marco used to generate the entry for your plugin
///
/// takes in a struct that implements the [`crate::plugin::Plugin`] trait.
#[macro_export]
macro_rules! entry {
    ( $func:ty ) => {
        use $crate::bindings::plugin_abi::{PluginInitFuncs, PluginNorthstarData};

        static mut ENGINE_CALLBACKS: Option<std::sync::Mutex<$crate::wrappers::engine::EngineCallbacks>> = None;

        #[no_mangle]
        #[export_name = "PLUGIN_INIT"]
        extern "C" fn plugin_init(
            plugin_init_funcs: *const PluginInitFuncs,
            plugin_northstar_data: *const PluginNorthstarData,
        ) {
            let mut plugin: $func = $crate::plugin::Plugin::new();

            unsafe { ENGINE_CALLBACKS = Some(std::sync::Mutex::new($crate::wrappers::engine::EngineCallbacks::default())) }

            let plugin_data = unsafe {
                $crate::wrappers::northstar::PluginData::new(
                    plugin_init_funcs,
                    plugin_northstar_data,
                    &mut ENGINE_CALLBACKS
                )
            };



            plugin_data.init_logger();
            log::info!("plugin logging initialized");

            plugin.initialize(&plugin_data);

            std::thread::spawn(move || plugin.main());
        }

        #[no_mangle]
        #[export_name = "PLUGIN_INFORM_SQVM_CREATED"]
        extern "C" fn plugin_inform_sqvm_created(
            context: $crate::bindings::squirrelclasstypes::ScriptContext,
            sqvm: *const $crate::bindings::squirrelclasstypes::CSquirrelVM,
        ) {
            match context {
                $crate::bindings::squirrelclasstypes::ScriptContext_SERVER => unsafe {
                    log::warn!(
                        "PLUGIN_INFORM_SQVM_CREATED got SERVER ScriptContext {:?}",
                        *sqvm
                    )
                },
                $crate::bindings::squirrelclasstypes::ScriptContext_CLIENT => unsafe {
                    log::warn!(
                        "PLUGIN_INFORM_SQVM_CREATED got CLIENT ScriptContext {:?}",
                        *sqvm
                    )
                },
                $crate::bindings::squirrelclasstypes::ScriptContext_UI => unsafe {
                    log::warn!("PLUGIN_INFORM_SQVM_CREATED got UI ScriptContext {:?}", *sqvm)
                },
                _ => log::warn!(
                    "PLUGIN_INFORM_SQVM_CREATED called with unknown ScriptContext {context}"
                ),
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

                    if let Ok(engine_callbacks) = ENGINE_CALLBACKS.as_ref().unwrap().try_lock() {
                        engine_callbacks.call_callbacks( engine_dll );
                    }
                },
                _ => log::warn!(
                    "PLUGIN_INFORM_DLL_LOAD called with unknown PluginLoadDLL type {dll}"
                ),
            }
        }
    };
}
