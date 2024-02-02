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
    /// this will create ub!
    ///
    /// **unless** before calling this everything will cleaned up!
    ///
    /// ex: convars, concommands, sqfunctions (sqvm reload), hooks, etc
    pub const unsafe fn allow_reload() -> Self {
        Self {
            should_reload: true,
        }
    }
}
