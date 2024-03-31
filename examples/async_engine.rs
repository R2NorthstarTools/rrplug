use rrplug::{bindings::cvar::convar::FCVAR_GAMEDLL, prelude::*};

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
                "set_max_score",
                set_max_score,
                "this will set the game's max score",
                FCVAR_GAMEDLL as i32,
                engine_token,
            )
            .expect("could not create set_max_score concommand");
    }
}

entry!(ExamplePlugin);

#[rrplug::concommand]
fn set_max_score(command: CCommandResult) -> Option<()> {
    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
        "SetScoreLimit",
        ScriptContext::SERVER,
        command.get_arg(0)?.parse::<i32>().ok()?,
    ));

    None
}
