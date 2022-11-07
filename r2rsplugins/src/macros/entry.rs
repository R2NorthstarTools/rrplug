#[macro_export]
macro_rules! entry {
    ( $func:ty ) => {
        use $crate::bindings::*;

        #[no_mangle]
        extern "C" fn initializePlugin(get_plugin_data_external: &std::ffi::c_void) {
            let mut plugin: $func = $crate::plugin::Plugin::new();

            plugin.initialize(get_plugin_data_external);

            std::thread::spawn(move || plugin.main());
        }
    };
}
