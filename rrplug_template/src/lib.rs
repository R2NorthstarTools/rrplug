{% if example %}use rrplug::prelude::*;
use rrplug::{
    bindings::convar::FCVAR_GAMEDLL, sq_return_null,
    wrappers::convars::{ConVarRegister, ConVarStruct},
    wrappers::northstar::{EngineLoadType, PluginData},
};

#[derive(Debug)]
pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    type SaveType = squirrel::Save;

    fn new() -> Self {
        Self {}
    }

    fn initialize(&mut self, plugin_data: &PluginData) {
        log::info!("yay logging :D");

        plugin_data.register_sq_functions(info_example);
    }

    fn on_engine_load(&self, engine: &EngineLoadType) {
        let engine = match engine {
            EngineLoadType::Engine(engine) => engine,
            EngineLoadType::EngineFailed => return,
            EngineLoadType::Server => return,
            EngineLoadType::Client => return,
        };

        let convar = ConVarStruct::try_new().unwrap();
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
    log::info!("args: {:?}", command.args)
}

#[rrplug::convar]
fn basic_convar_changed_callback(convar: Option<ConVarStruct>, old_value: String, float_old_value: f32) {
    log::info!("old value: {}", float_old_value)
}

#[rrplug::sqfunction(VM=Client,ExportName=BasicExample)]
fn example(name: String) {
    log::info!("exmaple {name}");

    sq_return_null!()
}

entry!(ExamplePlugin);
{% else %}use rrplug::prelude::*;

#[derive(Debug)]
pub struct BasicPlugin;

impl Plugin for BasicPlugin {
    type SaveType = squirrel::Save;

    fn new() -> Self {
        Self {}
    }

    fn initialize(&mut self, plugin_data: &PluginData) {
        log::info!("yay logging :D");
    }
}

entry!(BasicPlugin);
{% endif %}