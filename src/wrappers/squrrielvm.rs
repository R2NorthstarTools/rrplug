use crate::bindings::plugin_abi::SquirrelFunctions;
use crate::bindings::squirreldatatypes::CSquirrelVM;

use super::northstar::ScriptVmType;
use super::squrriel::SquirrelBuilder;

pub enum ScriptVm {
    Server(&'static mut CSquirrelVM),
    Client(&'static mut CSquirrelVM),
    Ui(&'static mut CSquirrelVM),
}

impl From<ScriptVm> for ScriptVmType {
    fn from(val: ScriptVm) -> Self {
        match val {
            ScriptVm::Server(_) => ScriptVmType::Server,
            ScriptVm::Client(_) => ScriptVmType::Client,
            ScriptVm::Ui(_) => ScriptVmType::Ui,
        }
    }
}

/// Client [`SquirrelFunctions`] can be used for Ui too
pub enum SqFunctions {
    Server(&'static SquirrelFunctions),
    Client(&'static SquirrelFunctions),
}

pub struct SquirrelVMCallbacks {
    callback_functions_server: Vec<Box<dyn Fn(SquirrelBuilder)>>,
    callback_functions_client: Vec<Box<dyn Fn(SquirrelBuilder)>>,
    callback_functions_ui: Vec<Box<dyn Fn(SquirrelBuilder)>>,
    callback_functions_server_init: Vec<Box<dyn Fn(SquirrelBuilder)>>,
    callback_functions_client_init: Vec<Box<dyn Fn(SquirrelBuilder)>>,
}

impl SquirrelVMCallbacks {
    pub fn new() -> Self {
        Self {
            callback_functions_server: Vec::new(),
            callback_functions_client: Vec::new(),
            callback_functions_ui: Vec::new(),
            callback_functions_server_init: Vec::new(),
            callback_functions_client_init: Vec::new(),
        }
    }

    /// from https://github.com/R2Northstar/NorthstarDiscordRPC/blob/0364829e70ac64189452bcfe6d7cd6804c665276/DiscordRPCV2/lib/loader.cpp#L34
    pub(crate) fn add_callback_server(&mut self, callback: Box<dyn Fn(SquirrelBuilder)>) {
        self.callback_functions_server.push(callback);
    }

    /// from https://github.com/R2Northstar/NorthstarDiscordRPC/blob/0364829e70ac64189452bcfe6d7cd6804c665276/DiscordRPCV2/lib/loader.cpp#L34
    pub(crate) fn add_callback_client(&mut self, callback: Box<dyn Fn(SquirrelBuilder)>) {
        self.callback_functions_client.push(callback);
    }

    /// from https://github.com/R2Northstar/NorthstarDiscordRPC/blob/0364829e70ac64189452bcfe6d7cd6804c665276/DiscordRPCV2/lib/loader.cpp#L34
    pub(crate) fn add_callback_ui(&mut self, callback: Box<dyn Fn(SquirrelBuilder)>) {

        self.callback_functions_ui.push(callback);
    }

    // from https://github.com/R2Northstar/NorthstarDiscordRPC/blob/0364829e70ac64189452bcfe6d7cd6804c665276/DiscordRPCV2/lib/loader.cpp#L30
    pub(crate) fn add_callback_server_init(&mut self, callback: Box<dyn Fn(SquirrelBuilder)>) {
        self.callback_functions_server_init.push(callback);
    }

    /// from https://github.com/R2Northstar/NorthstarDiscordRPC/blob/0364829e70ac64189452bcfe6d7cd6804c665276/DiscordRPCV2/lib/loader.cpp#L26
    pub(crate) fn add_callback_client_init(&mut self, callback: Box<dyn Fn(SquirrelBuilder)>) {
        self.callback_functions_client_init.push(callback);
    }

    pub fn call_callbacks_created(&self, sqvm_type: ScriptVm) {
        let (specific_callbacks, sqvm, sqvm_type) = match sqvm_type {
            ScriptVm::Server(sqvm) => (&self.callback_functions_server, sqvm, ScriptVmType::UiClient),
            ScriptVm::Client(sqvm) => (&self.callback_functions_client, sqvm, ScriptVmType::Client),
            ScriptVm::Ui(sqvm) => (&self.callback_functions_ui, sqvm, ScriptVmType::UiClient),
        };

        let mut sqbuilder = SquirrelBuilder::new();
        sqbuilder.set_sqtype(sqvm_type).set_sqvm_cs(sqvm);

        for callback in specific_callbacks {
            callback(sqbuilder.clone());
        }
    }

    pub fn call_callbacks_init(&self, functions: SqFunctions) {
        let (specific_callbacks, functions, sqtype) = match functions {
            SqFunctions::Server(functions) => (
                &self.callback_functions_server_init,
                functions,
                ScriptVmType::Server,
            ),
            SqFunctions::Client(functions) => (
                &self.callback_functions_client_init,
                functions,
                ScriptVmType::UiClient,
            ),
        };

        let mut sqbuilder = SquirrelBuilder::new();
        sqbuilder.set_sqtype(sqtype).set_sqvm_sqfunctions(functions);

        for callback in specific_callbacks {
            callback(sqbuilder.clone());
        }
    }
}

impl Default for SquirrelVMCallbacks {
    fn default() -> Self {
        Self::new()
    }
}
