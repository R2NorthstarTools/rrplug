#[macro_export]
macro_rules! entry {
    ( $func:ty ) => {
        static EPD: $crate::Once<ExternalPluginData> = $crate::Once::new();

        #[no_mangle]
        extern "C" fn initializePlugin(get_plugin_data_external: &'static std::ffi::c_void) {
            let mut plugin: $func = $crate::plugin::Plugin::new();

            EPD.call_once(|| $crate::ffi::ExternalPluginData::new(get_plugin_data_external));

            plugin.initialize();

            std::thread::spawn(move || plugin.main());
        }
    };
}
