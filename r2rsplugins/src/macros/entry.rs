#[macro_export]
macro_rules! entry {
    ( $func:ty ) => {
        static PluginData: $crate::Once<ExternalPluginData> = $crate::Once::new();

        use std;
        use std::ffi::c_void;
        use r2rsplugins::bindings::*;
        use r2rsplugins::prelude::*;

        #[no_mangle]
        extern "C" fn initializePlugin(
            get_plugin_data_external: Option<unsafe extern "C" fn(*const PluginObject) -> Return>,
        ) {
            let mut plugin: $func = $crate::plugin::Plugin::new();

            // PluginData.call_once(|| $crate::ffi::ExternalPluginData::new(get_plugin_data_external));

            // plugin.initialize( $crate::ffi::ExternalPluginData::new(get_plugin_data_external) );

            // let test = $crate::ffi::ExternalPluginData::new(& get_plugin_data_external);

            // let get_plugin_object:  = std::mem::transmute(get_plugin_data_external);

            test( get_plugin_data_external );

            // println!("got {:?}", get_plugin_data_external);

            // wait(3000);
            
            // unsafe {
            //     let game_state: GameState = get_plugin_data_external.unwrap()($crate::bindings::PluginObject_GAMESTATE as *const i32);
                
            //     println!( "gamestate struct: {:?}", game_state );

            //     wait(3000);

            //     let thing = $crate::ffi::utils::use_get_int_value_func( game_state.getGameStateInt.unwrap(), GameStateInfoType_ourScore );

            //     println!( "{:?}", thing );
            // }

            // match get_plugin_object {
            //     None => return None,
            //     Some(func) => {
            //         let game_state: &'static UnsafeGameState =
            //             func(PluginObject_GAMESTATE as *const i32);
            //         return Some(GameState::new(game_state));
            //     }
            // }

            std::thread::spawn(move || plugin.main());
        }
    };
}
