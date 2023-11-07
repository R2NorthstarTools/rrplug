{% if example %}use rrplug::bindings::cvar::convar::FCVAR_GAMEDLL;
use rrplug::prelude::*;

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn new(plugin_data: &PluginData) -> Self {
        log::info!("yay logging :D");

        plugin_data.register_sq_functions(example);

        Self {}
    }

    fn on_dll_load(&self, engine: Option<&EngineData>, _dll_ptr: &DLLPointer) {
        let engine = match engine {
            Some(engine) => engine,
            None => return,
        };

        let mut convar = ConVarStruct::try_new().unwrap();
        let register_info = ConVarRegister {
            callback: Some(basic_convar_changed_callback),
            ..ConVarRegister::mandatory(
                "basic_convar",
                "48",
                FCVAR_GAMEDLL.try_into().unwrap(),
                "basic_convar",
            )
        };

        convar.register(register_info).unwrap();

        _ = engine.register_concommand(
            "basic_command",
            basic_command_callback,
            "basic_command",
            FCVAR_GAMEDLL.try_into().unwrap(),
        );
    }
}

#[rrplug::concommand]
fn basic_command_callback(command: CCommandResult) {
    log::info!("running basic_command");
    log::info!("args: {:?}", command.get_args())
}

#[rrplug::convar]
fn basic_convar_changed_callback(_: String, float_old_value: f32) {
    log::info!("old value: {}", float_old_value)
}

#[rrplug::sqfunction(VM = "Client", ExportName = "BasicExample")]
fn example(name: String) {
    log::info!("exmaple {name}");
}

entry!(ExamplePlugin);

{% else %}use rrplug::prelude::*;

pub struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn new(_plugin_data: &PluginData) -> Self {
        Self {}
    }
}

entry!(BasicPlugin);
{% endif %}