//! wrappers for structs that are passed to the plugin

use crate::bindings::plugin_abi::PluginContext;

#[derive(Debug)]
pub struct PluginInfo {
    name: &'static str,
    log_name: &'static str,
    dependency_name: &'static str,
    context: PluginContext,
}

impl PluginInfo {
    pub const fn new(
        name: &'static str,
        log_name: &'static str,
        dependency_name: &'static str,
        context: PluginContext,
    ) -> Self {
        assert!(
            name.as_bytes()[name.len().saturating_sub(1)] == 0,
            "name has to end with a null char to be a null terminated string"
        );
        assert!(
            log_name.as_bytes()[log_name.len().saturating_sub(1)] == 0,
            "log_name has to end with a null char to be a null terminated string"
        );
        assert!(
            dependency_name.as_bytes()[dependency_name.len().saturating_sub(1)] == 0,
            "dependency_name has to end with a null char to be a null terminated string"
        );
        assert!(name.len() > 1, "consider actually having a name");
        assert!(log_name.len() > 1, "consider actually having a log_name");
        assert!(
            dependency_name.len() > 1,
            "consider actually having a dependency_name"
        );
        assert!(log_name.len().saturating_sub(1) == 9, "log name is used for logging and ideally should be 9 chars long and all upercase to look like every other log str");
        Self {
            name,
            log_name,
            dependency_name,
            context,
        }
    }

    pub const fn get_name(&self) -> &'static str {
        self.name
    }
    pub const fn get_log_name(&self) -> &'static str {
        self.log_name
    }
    pub const fn get_dependency_name(&self) -> &'static str {
        self.dependency_name
    }
    pub const fn get_context(&self) -> PluginContext {
        self.context
    }
}
