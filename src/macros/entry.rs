
/// # entry
/// marco used to generate the entry for your plugin
///
/// ### example
/// ```
///struct HelloWorld {
///    gamestate: Option<GameState>,
///}
///
///impl Plugin for HelloWorld {
///    fn new() -> Self {
///        Self {
///            gamestate: None,
///        }
///    }
///
///    fn initialize(&mut self, external_plugin_data: ExternalPluginData) {
///        println!("rust plugin initialized");
///
///        self.gamestate = external_plugin_data.get_game_state_struct();
///    }
///
///    fn main(&self) {
///        println!("hello world from rust");
///
///        let gamestate = self.gamestate.as_ref().unwrap();
///
///        loop {
///            println!( "our score: {}", gamestate.our_score() );
///
///            wait(3000)
///        }
///    }
///}
/// ```
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
