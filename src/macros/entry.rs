//! macro used to create export functions for the plugin

/// marco used to generate the entry for your plugin
///
/// takes in a struct that implements the [`crate::plugin::Plugin`] trait.
///
/// ### The most basic use
///
/// ```
/// use rrplug::prelude::*;
///
/// pub struct BasicPlugin;
///
/// impl Plugin for BasicPlugin {
/// const PLUGIN_INFO: PluginInfo =
///         PluginInfo::new(c"test", c"Testttttt", c"test", PluginContext::all());
///
///     fn new(reloaded: bool) -> Self {
///         Self {}
///     }
/// }
///
/// entry!(BasicPlugin);
///
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! entry {
    ( $plugin:ident ) => {
        #[allow(unused_imports)]
        #[doc(hidden)]
        pub(crate) use exports::PLUGIN;
        #[doc(hidden)]
        pub(crate) mod exports {
            #![allow(non_snake_case)]
            use super::$plugin;

            use high::engine::EngineData;
            use mid::squirrel::SQFUNCTIONS;
            use std::ffi::{CStr, CString};
            use $crate::bindings::{plugin_abi, squirrelclasstypes, squirreldatatypes};
            use $crate::exports::log;
            use $crate::exports::windows::{
                core::PCSTR, Win32::System::LibraryLoader::GetModuleHandleA,
            };
            use $crate::interfaces::external::SourceInterface;
            use $crate::plugin::Plugin;
            use $crate::rrplug;
            use $crate::{high, mid};

            pub static PLUGIN: $crate::exports::OnceCell<$plugin> =
                $crate::exports::OnceCell::new();

            struct PluginId;
            struct PluginCallbacks;

            #[allow(non_snake_case)]
            #[$crate::as_interface]
            impl PluginId {
                fn new() {
                    Self
                }

                fn GetString(&self, prop: plugin_abi::PluginString) -> *const std::ffi::c_char {
                    match prop {
                        plugin_abi::PluginString::Name => {
                            $plugin::PLUGIN_INFO.get_name().as_ptr() as *const i8
                        }
                        plugin_abi::PluginString::LogName => {
                            $plugin::PLUGIN_INFO.get_log_name().as_ptr() as *const i8
                        }
                        plugin_abi::PluginString::DependencyName => {
                            $plugin::PLUGIN_INFO.get_dependency_name().as_ptr() as *const i8
                        }
                        #[allow(unreachable_patterns)]
                        // for some reason this warning appears even tho the pattern is non exhaustive
                        _ => {
                            log::warn!("invalid plugin string requested!");
                            c"err".as_ptr()
                        }
                    }
                }

                fn GetField(&self, prop: plugin_abi::PluginField) -> i64 {
                    match prop {
                        plugin_abi::PluginField::Context => {
                            $plugin::PLUGIN_INFO.get_context().bits() as i64
                        }
                        plugin_abi::PluginField::Color => {
                            let mut packed = 0;
                            let color = $plugin::PLUGIN_INFO.get_color();

                            packed += color.red as i64;
                            packed += (color.green as i64) << 8;
                            packed += (color.blue as i64) << 16;

                            packed
                        }
                        #[allow(unreachable_patterns)]
                        _ => {
                            log::warn!("invalid plugin field requested!");
                            0
                        }
                    }
                }
            }

            #[$crate::as_interface]
            #[allow(non_snake_case)]
            impl PluginCallbacks {
                fn new() {
                    Self
                }

                fn Init(
                    &self,
                    ns_module: $crate::exports::windows::Win32::Foundation::HMODULE,
                    init_data: *const plugin_abi::PluginNorthstarData,
                    reloaded: bool,
                ) {
                    let plugin_data =
                        unsafe { init_data.as_ref().expect("plugin data should be valid") };

                    unsafe { mid::northstar::init_northstar_interfaces(ns_module, plugin_data) };

                    $crate::nslog::try_init(plugin_data.handle)
                        .expect("ns log didn't init correctly");

                    let plugin: $plugin = $crate::plugin::Plugin::new(reloaded);

                    if PLUGIN.set(plugin).is_err() {
                        panic!("PLUGIN failed initialization")
                    }

                    if reloaded {
                        const ENGINE: &CStr = c"engine.dll";
                        const SERVER: &CStr = c"server.dll";
                        const CLIENT: &CStr = c"client.dll";
                        unsafe {
                            _ = self.OnLibraryLoaded(
                                GetModuleHandleA(PCSTR(ENGINE.as_ptr().cast()))
                                    .expect("engine.dll should exists if called for reload"),
                                ENGINE.as_ptr(),
                            );
                            if let Ok(handle) = GetModuleHandleA(PCSTR(CLIENT.as_ptr().cast())) {
                                self.OnLibraryLoaded(handle, CLIENT.as_ptr());
                            } // client gets loaded before server
                            self.OnLibraryLoaded(
                                GetModuleHandleA(PCSTR(SERVER.as_ptr().cast()))
                                    .expect("server.dll should exists if called for reload"),
                                SERVER.as_ptr(),
                            );
                        }
                    }
                }
                fn Finalize(&self) {
                    PLUGIN
                        .wait()
                        .plugins_loaded(unsafe { high::engine::EngineToken::new_unchecked() })
                }
                fn Unload(&self) -> bool {
                    PLUGIN.wait().on_reload_request().should_reload()
                }
                fn OnSqvmCreated(&self, sqvm: *mut squirreldatatypes::CSquirrelVM) {
                    _ = mid::squirrel::SQFUNCTIONS.try_init();

                    let context: squirrelclasstypes::ScriptContext = unsafe { (*sqvm).vmContext }
                        .try_into()
                        .expect("sqvm was not valid :((((");

                    for func_info in high::squirrel::FUNCTION_SQ_REGISTER
                        .lock()
                        .iter()
                        .filter(|info| info.vm.contains_context(context))
                    {
                        unsafe {
                            if let Err(err) =
                                mid::squirrel::manually_register_sq_functions(&mut *sqvm, func_info)
                            {
                                log::error!("{err}");
                            }
                        }
                    }

                    let token = unsafe { high::engine::EngineToken::new_unchecked() };
                    let sqvm =
                        std::ptr::NonNull::new(sqvm).expect("sqvm should not be null on sqvm init");
                    let handle =
                        high::squirrel::CSquirrelVMHandle::new(sqvm, context, false, token);

                    PLUGIN.wait().on_sqvm_created(&handle, token);
                }
                fn OnSqvmDestroying(&self, sqvm: *mut squirreldatatypes::CSquirrelVM) {
                    let context: squirrelclasstypes::ScriptContext = unsafe { (*sqvm).vmContext }
                        .try_into()
                        .expect("sqvm was not valid :((((");
                    let token = unsafe { high::engine::EngineToken::new_unchecked() };
                    let sqvm = std::ptr::NonNull::new(sqvm)
                        .expect("sqvm should not be null on sqvm destroy");
                    let handle = high::squirrel::CSquirrelVMHandle::new(sqvm, context, true, token);
                    PLUGIN.wait().on_sqvm_destroyed(&handle, token);
                    unsafe { $crate::high::engine_sync::run_async_routine() }; // run all possible stuff to not leak into next sqvm
                }
                fn OnLibraryLoaded(
                    &self,
                    module: $crate::exports::windows::Win32::Foundation::HMODULE,
                    library_name: *const std::ffi::c_char,
                ) {
                    let dll_string = unsafe { std::ffi::CStr::from_ptr(library_name) }
                        .to_string_lossy()
                        .to_string();

                    let mut called_dlls = high::engine::CALLED_DLLS.lock();
                    if called_dlls.contains(&dll_string) {
                        return;
                    }

                    let dll_ptr = $crate::mid::engine::DLLPointer::new(
                        dll_string.as_str(),
                        module.0 as *const std::ffi::c_void,
                    );

                    unsafe {
                        mid::engine::convars::CvarGlobals::try_init(
                            &dll_ptr,
                            &mid::engine::convars::CVAR_GLOBALS,
                        );
                        mid::engine::concommands::RegisterConCommands::try_init(
                            &dll_ptr,
                            &mid::engine::concommands::REGISTER_CONCOMNMADS,
                        );
                        mid::server::EntityClassVtable::try_init(
                            &dll_ptr,
                            &mid::server::ENTITY_CLASS_VTABLE,
                        );
                    }
                    mid::squirrel::SQFUNCTIONS.fetch_functions(&dll_ptr);

                    let engine_data = if dll_string == "engine.dll" {
                        unsafe {
                            _ = mid::engine::ENGINE_DATA.set(EngineData::new(
                                $crate::bindings::cvar::RawCVar::from_dll_name(
                                    "vstdlib.dll",
                                    "VEngineCvar007",
                                )
                                .expect("no CVar?????"),
                            ));
                        }
                        Some(mid::engine::ENGINE_DATA.wait())
                    } else {
                        None
                    };

                    PLUGIN.wait().on_dll_load(engine_data, &dll_ptr, unsafe {
                        high::engine::EngineToken::new_unchecked()
                    });

                    called_dlls.push(dll_string);
                }
                fn RunFrame(&self) {
                    unsafe { $crate::high::engine_sync::run_async_routine() };
                    PLUGIN
                        .wait()
                        .runframe(unsafe { high::engine::EngineToken::new_unchecked() });
                }
            }

            #[export_name = "DllMain"]
            extern "C" fn dll_main(
                _hmodule: $crate::exports::windows::Win32::Foundation::HINSTANCE,
                reason: u32,
                _: *mut std::ffi::c_void,
            ) -> bool {
                if reason
                    == $crate::exports::windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH
                {
                    unsafe {
                        $crate::interfaces::manager::register_interface(
                            "PluginId001",
                            PluginId::new(),
                        );
                        $crate::interfaces::manager::register_interface(
                            "PluginCallbacks001",
                            PluginCallbacks::new(),
                        );
                    }
                    $crate::high::engine_sync::init_async_routine();
                    $plugin::on_module_load();
                }
                true
            }
        }
    };
}

#[cfg(test)]
mod test_entry {
    use crate::prelude::*;

    pub struct Test;

    impl Plugin for Test {
        const PLUGIN_INFO: PluginInfo =
            PluginInfo::new(c"test", c"Testttttt", c"test", PluginContext::all());

        fn new(_reloaded: bool) -> Self {
            Self {}
        }
    }

    entry!(Test);

    #[test]
    const fn test_init() {
        // todo: somehow test all the functions
    }
}
