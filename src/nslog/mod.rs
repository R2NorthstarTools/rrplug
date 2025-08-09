use windows::Win32::Foundation::HMODULE;

#[cfg(feature = "logging-log")]
mod log;

#[cfg(feature = "logging-tracing")]
mod tracing;

#[cfg(all(feature = "logging-tracing", feature = "logging-log"))]
compile_error!("tracing and log are mutually exclusive and cannot be enabled together");

pub fn try_init(plugin_handle: HMODULE) -> Result<(), String> {
    #[cfg(feature = "logging-log")]
    log::try_init(plugin_handle).map_err(|err| format!("{err:?}"))
    #[cfg(feature = "logging-tracing")]
    tracing::try_init(plugin_handle).map_err(|err| format!("{err:?}"))
}
