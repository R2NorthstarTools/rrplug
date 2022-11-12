
/// marco used to generate the entry for your plugin
/// 
/// takes in a struct that implements the [`crate::plugin::Plugin`] trait.
#[macro_export]
macro_rules! entry {
    ( $func:ty ) => {
        #[no_mangle]
        // could use #[export_name = "initializePlugin"] here :)
        extern "C" fn initializePlugin(get_plugin_data_external: *const std::ffi::c_void) {
            let mut plugin: $func = $crate::plugin::Plugin::new();
            
            plugin.initialize( $crate::ffi::ExternalPluginData::new(get_plugin_data_external) );

            std::thread::spawn(move || plugin.main());
        }
    };
}
