#[macro_export]
macro_rules! entry {
    ( $func:ty ) => {
        static PLUGIN: $crate::once::Once<($func)> = $crate::once::Once::new();

        use $crate::bindings::*;

        #[no_mangle]
        extern "C" fn initializePlugin( getPluginData_external: ::std::os::raw::c_void ) { // std::ffi::c_void


            PLUGIN.call_once(|| $crate::plugin::Plugin::new());
            PLUGIN.wait().initialize( getPluginData_external );

            std::thread::spawn(|| PLUGIN.wait().main());
        }
    };
}
