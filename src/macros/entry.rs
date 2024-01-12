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
///     fn new() -> Self {
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
            use super::$plugin;

            use high::{engine::EngineData, northstar::ScriptVmType};
            use mid::squirrel::SQFUNCTIONS;
            use $crate::bindings::{plugin_abi, squirrelclasstypes, squirreldatatypes};
            use $crate::exports::log;
            use $crate::interfaces::external::SourceInterface;
            use $crate::plugin::Plugin;
            use $crate::rrplug;
            use $crate::{high, mid};

            use std::ffi::CString;

            pub static PLUGIN: $crate::exports::OnceCell<$plugin> =
                $crate::exports::OnceCell::new();

            struct PluginId;
            struct PluginCallbacks;

            #[$crate::as_interface]
            impl PluginId {
                fn new() {
                    Self
                }

                fn GetString(&self, prop: plugin_abi::PluginString) -> *const std::ffi::c_char {
                    match prop {
                        plugin_abi::PluginString::Name => todo!(),
                        plugin_abi::PluginString::LogName => todo!(),
                        plugin_abi::PluginString::DependencyName => todo!(),
                    }
                }

                fn GetField(&self, prop: plugin_abi::PluginField) -> i64 {
                    match prop {
                        plugin_abi::PluginField::Context => todo!(),
                    }
                }
            }

            #[$crate::as_interface]
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

                    $crate::nslog::try_init(plugin_data.handle);

                    let plugin: $plugin = $crate::plugin::Plugin::new();

                    if PLUGIN.set(plugin).is_err() {
                        panic!("PLUGIN failed initialization")
                    }
                }
                fn Finalize(&self) {}
                fn Unload(&self) -> bool {
                    false // TODO: add this to Plugin
                }
                fn OnSqvmCreated(&self, sqvm: *mut squirreldatatypes::CSquirrelVM) {
                    let context = std::convert::Into::<ScriptVmType>::into(unsafe {
                        (*sqvm).vmContext // rewrite later
                    });
                    log::info!("PLUGIN_INFORM_SQVM_CREATED called {}", context);

                    let locked_register_functions = high::squirrel::FUNCTION_SQ_REGISTER.lock();

                    let sq_functions = match context {
                        ScriptVmType::Server => SQFUNCTIONS.server.get(),
                        ScriptVmType::Client => SQFUNCTIONS.client.get(),
                        ScriptVmType::Ui => SQFUNCTIONS.client.get(),
                        _ => {
                            log::error!("invalid ScriptContext");
                            return;
                        }
                    }
                    .expect("SQFUNCTIONS should be initialized at this point");

                    let sq_register_func = sq_functions.register_squirrel_func;

                    for func_info in locked_register_functions
                        .iter()
                        .filter(|info| info.vm.is_right_vm(&context))
                    {
                        log::info!(
                            "Registering {context} function {} with types: {}",
                            func_info.sq_func_name,
                            func_info.types
                        );

                        let esq_returntype =
                            match func_info.return_type.split('<').collect::<Vec<&str>>()[0] {
                                "bool" => squirrelclasstypes::eSQReturnType::Boolean,
                                "float" => squirrelclasstypes::eSQReturnType::Float,
                                "vector" => squirrelclasstypes::eSQReturnType::Vector,
                                "int" => squirrelclasstypes::eSQReturnType::Integer,
                                "entity" => squirrelclasstypes::eSQReturnType::Entity,
                                "string" => squirrelclasstypes::eSQReturnType::String,
                                "array" => squirrelclasstypes::eSQReturnType::Arrays,
                                "asset" => squirrelclasstypes::eSQReturnType::Asset,
                                "table" => squirrelclasstypes::eSQReturnType::Table,
                                "void" => squirrelclasstypes::eSQReturnType::Default,
                                "var" => squirrelclasstypes::eSQReturnType::Default,
                                _ => squirrelclasstypes::eSQReturnType::Default,
                            };

                        // shouldn't be unwraping here but I will say : why did you name your function like this?
                        let sq_func_name = CString::new(func_info.sq_func_name).unwrap();
                        let cpp_func_name = CString::new(func_info.cpp_func_name).unwrap();
                        let help_text = CString::new("what help").unwrap();
                        let return_type: &str = &func_info.return_type;
                        let returntype = CString::new(return_type).unwrap();
                        let types: &str = &func_info.types;
                        let types = CString::new(types).unwrap();

                        let mut reg =
                            std::mem::MaybeUninit::<squirrelclasstypes::SQFuncRegistration>::zeroed(
                            );
                        let struct_ptr = reg.as_mut_ptr();

                        unsafe {
                            std::ptr::addr_of_mut!((*struct_ptr).squirrelFuncName)
                                .write(sq_func_name.as_ptr());
                            std::ptr::addr_of_mut!((*struct_ptr).cppFuncName)
                                .write(cpp_func_name.as_ptr());
                            std::ptr::addr_of_mut!((*struct_ptr).helpText)
                                .write(help_text.as_ptr());
                            std::ptr::addr_of_mut!((*struct_ptr).returnTypeString)
                                .write(returntype.as_ptr());
                            std::ptr::addr_of_mut!((*struct_ptr).returnType).write(esq_returntype);
                            std::ptr::addr_of_mut!((*struct_ptr).argTypes).write(types.as_ptr());
                            std::ptr::addr_of_mut!((*struct_ptr).funcPtr).write(func_info.function);
                        };

                        debug_assert!(!struct_ptr.is_null());
                        debug_assert!(!sqvm.is_null());

                        unsafe {
                            sq_register_func(sqvm, struct_ptr, 1);
                        }
                    }

                    let handle = high::squirrel::CSquirrelVMHandle::new(sqvm, context);

                    PLUGIN.wait().on_sqvm_created(&handle);
                }
                fn OnSqvmDestroying(&self, sqvm: *mut squirreldatatypes::CSquirrelVM) {
                    let context = std::convert::Into::<ScriptVmType>::into(unsafe {
                        (*sqvm).vmContext // rewrite later
                    });
                    let handle = high::squirrel::CSquirrelVMHandle::new(sqvm, context);
                    PLUGIN.wait().on_sqvm_destroyed(&handle);
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

                    mid::convars::CvarGlobals::try_init(&dll_ptr, &mid::convars::CVAR_GLOBALS);
                    mid::concommands::RegisterConCommands::try_init(
                        &dll_ptr,
                        &mid::concommands::REGISTER_CONCOMNMADS,
                    );

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

                    PLUGIN.wait().on_dll_load(engine_data, &dll_ptr);

                    called_dlls.push(dll_string);
                }
                fn RunFrame(&self) {
                    PLUGIN.wait().runframe();
                }
            }

            #[no_mangle]
            #[export_name = "DllMain"]
            extern "stdcall" fn dll_main(
                hmodule: $crate::exports::windows::Win32::Foundation::HINSTANCE,
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
        fn new() -> Self {
            Self {}
        }
    }

    entry!(Test);

    #[test]
    fn test_init() {
        // todo: somehow test all the functions
    }
}
