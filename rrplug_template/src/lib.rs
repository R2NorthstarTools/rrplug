{% if example %}use rrplug::prelude::*;
use rrplug::{
    bindings::convar::FCVAR_GAMEDLL,
    concommand, convar, sq_return_null, sqfunction,
    wrappers::convars::{ConvarRegister, ConvarStruct},
    wrappers::northstar::{EngineLoadType, PluginData},
};

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn new() -> Self {
        Self {}
    }

    fn initialize(&mut self, plugin_data: &PluginData) {
        log::info!("yay logging :D");

        _ = plugin_data.register_sq_functions(info_example);
    }

    fn main(&self) {}

    fn on_engine_load(&self, engine: EngineLoadType) {
        let engine = match engine {
            EngineLoadType::Engine(engine) => engine,
            EngineLoadType::EngineFailed => return,
            EngineLoadType::Server => return,
            EngineLoadType::Client => return,
        };

        let convar = ConvarStruct::try_new().unwrap();
        let register_info = ConvarRegister {
            callback: Some(basic_convar_changed_callback),
            ..ConvarRegister::mandatory(
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

#[concommand]
fn basic_command_callback(command: CCommandResult) {
    log::info!("running basic_command");
    log::info!("args: {}", command.args)
}

#[convar]
fn basic_convar_changed_callback(convar: ConvarStruct, old_value: String, float_old_value: f32) {
    log::info!("convar name: {}", convar.get_name());
    log::info!("new value: {}", convar.get_value().value_float);
    log::info!("old value: {}", float_old_value)
}

#[sqfunction(VM=Client,ExportName=BasicExample)]
fn example(name: String) {
    log::info!("exmaple {name}");

    sq_return_null!()
}

entry!(ExamplePlugin);
{% else %}use rrplug::prelude::*;

pub struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn new() -> Self {
        Self {}
    }

    fn initialize(&mut self, plugin_data: &PluginData) {
        log::info!("yay logging :D");
    }

    fn main(&self) {}
}

entry!(BasicPlugin);
{% endif %}