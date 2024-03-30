//! rrplug's prototype of a plugin reloading system

/// the reponse to a unload event
///
/// # Difficulties
/// the main issue arises from dangling pointers since the moment a plugin is reloaded callbacks, sqfunctions, etc will start calling into uninit memory.
/// which may be hard to handle at times.
///
/// the other issue is reinstating the plugin. rrplug will get back up by getting handles to engine, client and server in order
/// but squirrel cannot be fully restorted so it's best to reset all vms.
pub struct ReloadResponse {
    should_reload: bool,
}

impl ReloadResponse {
    #[doc(hidden)]
    pub const fn should_reload(self) -> bool {
        self.should_reload
    }

    /// simply denies plugin sys from reloading the plugin
    pub const fn deny_reload() -> Self {
        Self {
            should_reload: false,
        }
    }

    /// allows Plugin Sys to reload the plugin
    ///
    /// # Safety
    ///
    /// **this will create ub!**
    ///
    /// **unless** before calling this everything will be cleaned up!
    ///
    /// ex: convars, concommands, sqfunctions (sqvm reload), hooks, etc
    pub const unsafe fn allow_reload() -> Self {
        Self {
            should_reload: true,
        }
    }
}
