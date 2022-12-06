use crate::bindings::squirreldatatypes::HSquirrelVM;
use crate::bindings::plugin_abi::SquirrelFunctions;

pub enum ScriptVm {
    Server(HSquirrelVM),
    Client(HSquirrelVM),
    Ui(HSquirrelVM),
}

/// Client [`SquirrelFunctions`] can be used for Ui too
pub enum SqFunctions {
    Server(SquirrelFunctions),
    Client(SquirrelFunctions),
}

pub struct SquirrelVMCallbacks {
    callback_functions_server: Vec<Box<dyn Fn(HSquirrelVM)>>,
    callback_functions_client: Vec<Box<dyn Fn(HSquirrelVM)>>,
    callback_functions_ui: Vec<Box<dyn Fn(HSquirrelVM)>>,
    callback_functions_server_init: Vec<Box<dyn Fn(SquirrelFunctions)>>,
    callback_functions_client_init: Vec<Box<dyn Fn(SquirrelFunctions)>>,
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
    pub(crate) fn add_callback_server(&mut self, callback: Box<dyn Fn(HSquirrelVM)>) {
        self.callback_functions_server.push(callback);
    }
    
    /// from https://github.com/R2Northstar/NorthstarDiscordRPC/blob/0364829e70ac64189452bcfe6d7cd6804c665276/DiscordRPCV2/lib/loader.cpp#L34
    pub(crate) fn add_callback_client(&mut self, callback: Box<dyn Fn(HSquirrelVM)>) {
        self.callback_functions_client.push(callback);
    }
    
    /// from https://github.com/R2Northstar/NorthstarDiscordRPC/blob/0364829e70ac64189452bcfe6d7cd6804c665276/DiscordRPCV2/lib/loader.cpp#L34
    pub(crate) fn add_callback_ui(&mut self, callback: Box<dyn Fn(HSquirrelVM)>) {
        self.callback_functions_ui.push(callback);
    }
    
    // from https://github.com/R2Northstar/NorthstarDiscordRPC/blob/0364829e70ac64189452bcfe6d7cd6804c665276/DiscordRPCV2/lib/loader.cpp#L30
    pub(crate) fn add_callback_server_init(&mut self, callback: Box<dyn Fn(SquirrelFunctions)>) {
        self.callback_functions_server_init.push(callback);
    }
    
    /// from https://github.com/R2Northstar/NorthstarDiscordRPC/blob/0364829e70ac64189452bcfe6d7cd6804c665276/DiscordRPCV2/lib/loader.cpp#L26
    pub(crate) fn add_callback_client_init(&mut self, callback: Box<dyn Fn(SquirrelFunctions)>) {
        self.callback_functions_client_init.push(callback);
    }

    pub fn call_callbacks_created(&self, sqvm: ScriptVm) {
        let (specific_callbacks,sqvm) = match sqvm {
            ScriptVm::Server(sqvm) => (&self.callback_functions_server,sqvm),
            ScriptVm::Client(sqvm) => (&self.callback_functions_client,sqvm),
            ScriptVm::Ui(sqvm) => (&self.callback_functions_ui,sqvm),
        };

        for callback in specific_callbacks {
            callback(sqvm);
        }
    }

    pub fn call_callbacks_init(&self, functions: SqFunctions) {
        let (specific_callbacks,functions) = match functions {
            SqFunctions::Server(functions) => (&self.callback_functions_server_init,functions),
            SqFunctions::Client(functions) => (&self.callback_functions_client_init,functions),
        };

        for callback in specific_callbacks {
            callback(functions);
        }
    }
}

impl Default for SquirrelVMCallbacks {
    fn default() -> Self {
        Self::new()
    }
}