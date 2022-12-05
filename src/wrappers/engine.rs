use crate::bindings::plugin_abi::PluginEngineData;

pub struct EngineCallbacks {
    callback_functions: Vec<Box<dyn Fn(PluginEngineData)>>,
}

impl EngineCallbacks {
    pub fn new() -> Self {
        Self {
            callback_functions: Vec::new(),
        }
    }

    pub(crate) fn add_callback(&mut self, callback: Box<dyn Fn(PluginEngineData)>) {
        self.callback_functions.push(callback);
    }

    pub fn call_callbacks(&self, data: PluginEngineData) {
        for callback in &self.callback_functions {
            callback(data);
        }
    }
}

impl Default for EngineCallbacks {
    fn default() -> Self {
        Self::new()
    }
}
