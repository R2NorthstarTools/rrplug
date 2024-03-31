use rrplug::{bindings::cvar::convar::FCVAR_GAMEDLL, prelude::*};
use std::cell::RefCell;

static HELLO_COUNT: EngineGlobal<RefCell<Option<ConVarStruct>>> =
    EngineGlobal::new(RefCell::new(None));

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    const PLUGIN_INFO: PluginInfo =
        PluginInfo::new(c"example", c"EXAMPLLEE", c"EXAMPLE", PluginContext::all());

    fn new(_reloaded: bool) -> Self {
        Self {}
    }

    fn on_dll_load(
        &self,
        engine_data: Option<&EngineData>,
        _dll_ptr: &DLLPointer,
        engine_token: EngineToken,
    ) {
        let Some(engine_data) = engine_data else {
            return;
        };

        engine_data
            .register_concommand(
                "hello_world",
                hello_world,
                "this will set the game's max score",
                FCVAR_GAMEDLL as i32,
                engine_token,
            )
            .expect("could not create set_max_score concommand");

        let convar = ConVarStruct::try_new(
            &ConVarRegister {
                callback: Some(hello_count), // prints the amount of hellos on each update
                ..ConVarRegister::mandatory("hello_count", "0", FCVAR_GAMEDLL as i32, "todo")
            },
            engine_token,
        )
        .expect("could not create hello_count convar");

        _ = HELLO_COUNT.get(engine_token).borrow_mut().replace(convar);
    }
}

entry!(ExamplePlugin);

#[rrplug::concommand]
fn hello_world() {
    log::info!("hello world");

    let mut convar = HELLO_COUNT.get(engine_token).borrow_mut();
    if let Some(convar) = convar.as_mut() {
        convar.set_value_i32(convar.get_value_i32() + 1, engine_token);
    }
}

#[rrplug::convar]
fn hello_count() {
    log::info!(
        "hellos {}",
        HELLO_COUNT
            .get(engine_token)
            .borrow()
            .as_ref()
            .map(|convar| convar.get_value_i32())
            .unwrap_or(0)
    );
}
