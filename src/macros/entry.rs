/// marco used to generate the entry for your plugin
///
/// takes in a struct that implements the [`crate::plugin::Plugin`] trait.
#[macro_export]
macro_rules! entry {
    ( $func:ty ) => {
        use $crate::bindings::plugin_abi::{PluginInitFuncs, PluginNorthstarData};

        #[no_mangle]
        #[export_name = "PLUGIN_INIT"]
        extern "C" fn plugin_init(
            plugin_init_funcs: *const PluginInitFuncs,
            plugin_northstar_data: *const PluginNorthstarData,
        ) {
            let mut plugin: $func = $crate::plugin::Plugin::new();

            unsafe { $crate::wrappers::loader::ENGINE_CALLBACKS = Some(std::sync::Mutex::new($crate::wrappers::engine::EngineCallbacks::default())) }

            let plugin_data = unsafe {
                $crate::wrappers::northstar::PluginData::new(
                    plugin_init_funcs,
                    plugin_northstar_data,
                    &mut $crate::wrappers::loader::ENGINE_CALLBACKS,
                )
            };

            plugin_data.init_logger();
            log::info!("plugin logging initialized");

            plugin.initialize(&plugin_data);

            std::thread::spawn(move || plugin.main());
        }
    };
}
