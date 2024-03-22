//! wrappers for structs that are passed to the plugin

use std::ffi::CStr;

use crate::bindings::plugin_abi::PluginContext;

/// information about the plugin that the plugins system requests
#[derive(Debug)]
pub struct PluginInfo {
    name: &'static CStr,
    log_name: &'static CStr,
    dependency_name: &'static CStr,
    context: PluginContext,
}

impl PluginInfo {
    /// Creates a new [`PluginInfo`].
    ///
    /// # Panics
    ///
    /// Panics if the following conditions are not met
    /// - the strings have to be null terminated
    /// - strings cannot be empty
    /// - log name has to be 9 chars in lenght (not counting the null terminator)
    pub const fn new(
        name: &'static CStr,
        log_name: &'static CStr,
        dependency_name: &'static CStr,
        context: PluginContext,
    ) -> Self {
        assert!(name.to_bytes().len() > 1, "consider actually having a name");
        assert!(
            log_name.to_bytes().len() > 1,
            "consider actually having a log_name"
        );
        assert!(
            dependency_name.to_bytes().len() > 1,
            "consider actually having a dependency_name"
        );
        assert!(log_name.to_bytes().len() == 9, "log name is used for logging and ideally should be 9 chars long and all upercase to look like every other log str");
        Self {
            name,
            log_name,
            dependency_name,
            context,
        }
    }

    /// Returns a reference to the get name of the plugin.
    pub const fn get_name(&self) -> &'static CStr {
        self.name
    }
    /// Returns a reference to the get log name of the plugin.
    pub const fn get_log_name(&self) -> &'static CStr {
        self.log_name
    }
    /// Returns a reference to the get dependency name of the plugin.
    pub const fn get_dependency_name(&self) -> &'static CStr {
        self.dependency_name
    }
    /// Returns the get context of the plugin.
    pub const fn get_context(&self) -> PluginContext {
        self.context
    }
}
