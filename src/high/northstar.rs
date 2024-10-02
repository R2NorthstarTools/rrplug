//! wrappers for structs that are passed to the plugin

use std::ffi::CStr;

use crate::bindings::plugin_abi::{PluginColor, PluginContext};

/// information about the plugin that the plugins system requests
#[derive(Debug)]
pub struct PluginInfo {
    name: &'static CStr,
    log_name: &'static CStr,
    dependency_name: &'static CStr,
    context: PluginContext,
    color: PluginColor,
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
        // assert!(dependency_name.to_bytes() ); // TODO a check valid constants
        Self {
            name,
            log_name,
            dependency_name,
            context,
            color: PluginColor {
                red: 244,
                green: 106,
                blue: 14,
            },
        }
    }

    /// Creates a new [`PluginInfo`] but with a color for logs.
    ///
    /// # Panics
    /// refer to `new`
    pub const fn new_with_color(
        name: &'static CStr,
        log_name: &'static CStr,
        dependency_name: &'static CStr,
        context: PluginContext,
        color: PluginColor,
    ) -> Self {
        Self {
            color,
            ..Self::new(name, log_name, dependency_name, context)
        }
    }

    /// Returns a reference to the name of the plugin.
    pub const fn get_name(&self) -> &'static CStr {
        self.name
    }
    /// Returns a reference to the log name of the plugin.
    pub const fn get_log_name(&self) -> &'static CStr {
        self.log_name
    }
    /// Returns a reference to the dependency name of the plugin.
    pub const fn get_dependency_name(&self) -> &'static CStr {
        self.dependency_name
    }
    /// Returns the context of the plugin.
    pub const fn get_context(&self) -> PluginContext {
        self.context
    }
    /// Returns the color of the plugin.
    pub const fn get_color(&self) -> PluginColor {
        self.color
    }
}
