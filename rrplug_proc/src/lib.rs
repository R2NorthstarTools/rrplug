extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    self, parse_macro_input, parse_quote, DeriveInput, FnArg, Ident, ItemFn, ReturnType, Stmt, Error as SynError
};

#[macro_use]
pub(crate) mod parsing;
pub(crate) mod impl_traits;

// todo: redo docs for proc macros

use impl_traits::{ impl_struct_or_enum, push_to_sqvm_impl_struct, push_to_sqvm_impl_enum, get_from_sqvm_impl_enum, get_from_sqvm_impl_struct, get_from_sqobject_impl_enum};
use parsing::{filter_args, get_sqoutput, input_mapping, Args};
/// proc marco for generating compatible functions with the sqvm
///
///  ### abstractions
/// the marco catpures any arguments types and return types and tranlates them into sqfunction deffintion
/// also adds code to transform the sqtypes to rust types at runtime
///
/// all the information that is relevent for the sqfunction registration is collected into the functin with the same and a `info_` prefix
///
/// returns are managed by other marcos
#[proc_macro_attribute]
pub fn sqfunction(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as Args).args;

    let input = parse_macro_input!(item as ItemFn);

    let ItemFn {
        attrs: _,
        vis,
        sig,
        block,
    } = input;

    let stmts = block.stmts;
    let mut sub_stms = Vec::new();
    let mut sqtypes = String::new();
    let ident = &sig.ident;
    let input = &sig.inputs;
    let input_vec = input.iter().filter_map(|arg| filter_args(arg));
    let input_var_names: Vec<Ident> = input
        .iter()
        .cloned()
        .filter_map(|_input| {
            if let FnArg::Typed(t) = _input {
                Some(t)
            } else {
                None
            }
        })
        .map(|t| {
            t.pat
                .into_token_stream()
                .to_string()
                .replace("mut", "")
                .replace(" ", "")
        }) // not the best solution
        .map(|ident| format_ident!("{ident}"))
        .collect();
    let output = &sig.output;
    let (ouput_type, ouput_parsing) = match output.clone() {
        ReturnType::Default => (
            parse_quote!(()),
            quote!(rrplug::bindings::squirrelclasstypes::SQRESULT::SQRESULT_NULL),
        ),
        ReturnType::Type(_, t) => (
            t,
            quote!({
                use rrplug::high::squirrel_traits::PushToSquirrelVm;
                output.push_to_sqvm(sqvm, sq_functions);
                rrplug::bindings::squirrelclasstypes::SQRESULT::SQRESULT_NOTNULL
            }),
        ),
    };
    let func_name = ident.to_string();
    let sq_functions_func: Ident = format_ident!("sq_func_{}", func_name.clone());
    let mut export_name = ident.to_string();

    let mut sq_stack_pos = 1;
    let mut sq_gets_stmts = Vec::new();

    let mut out = get_sqoutput(output);

    match input_mapping(input, &mut sqtypes, &mut sq_stack_pos) {
        Ok(tks) => {
            for tk in tks {
                push_stmts!(sq_gets_stmts, tk);
            }
        }
        Err(err) => {
            return err.to_compile_error().into();
        }
    }
    sq_gets_stmts.reverse();
    for s in sq_gets_stmts {
        sub_stms.insert(0, s);
    }

    let mut script_vm_func = "client";
    let mut script_vm = "Client";

    for arg in args {
        let input = arg.arg.to_token_stream().to_string().replace('"', "");
        match &arg.ident.to_string()[..] {
            "VM" if input.to_uppercase().ends_with("UI") => {
                script_vm = "Ui";
                script_vm_func = "client";
            }
            "VM" if input.to_uppercase().ends_with("SERVER") => {
                script_vm = "Server";
                script_vm_func = "server";
            }
            "VM" if input.to_uppercase().ends_with("UICLIENT") => {
                script_vm = "UiClient";
                script_vm_func = "client";
            }
            "VM" if input.to_uppercase().ends_with("CLIENT") => {
                script_vm = "Client";
                script_vm_func = "client";
            }
            "VM" => {
                let fmt = format!("invalid VM {}", input);
                return quote! { compile_error!(#fmt) }.into();
            }
            "ExportName" => export_name = input,
            "ReturnOverwrite" => out = out,
            _ => {
                return SynError::new(
                    arg.ident.span(),
                    format!("wrong arg \"{}\" or arg {}", input, arg.ident.to_string())
                ).to_compile_error()
                .into();
            }
        }
    }
    let script_vm_type = format_ident!("{script_vm}");
    let script_vm_func = format_ident!("{script_vm_func}");

    let tk = quote! {let sq_functions = SQFUNCTIONS.#script_vm_func.wait();}.into();
    push_stmts!(sub_stms, tk);

    let out: TokenStream = quote! {
        #[doc(hidden)]
        #[doc = "generated ffi function for #func_name"]
        #vis extern "C" fn #sq_functions_func (sqvm: *mut rrplug::bindings::squirreldatatypes::HSquirrelVM) -> rrplug::bindings::squirrelclasstypes::SQRESULT {
            use rrplug::high::squirrel_traits::GetFromSquirrelVm;

            #(#sub_stms)*
            
            fn inner_function( sqvm: *mut rrplug::bindings::squirreldatatypes::HSquirrelVM, sq_functions: &SquirrelFunctionsUnwraped #(, #input_vec)* ) -> Result<#ouput_type,String> {
                #(#stmts)*
            }

            match inner_function( sqvm, sq_functions #(, #input_var_names)* ) {
                Ok(output) => {
                    #ouput_parsing
                },
                Err(err) => {
                    let err = rrplug::to_sq_string!(err);
                    unsafe { (sq_functions.sq_raiseerror)(sqvm, err.as_ptr()) };
                    rrplug::bindings::squirrelclasstypes::SQRESULT::SQRESULT_ERROR 
                }
            }
        }

        #vis const fn #ident () -> rrplug::high::northstar::SQFuncInfo {
            rrplug::high::northstar::SQFuncInfo{ 
                cpp_func_name: #func_name, 
                sq_func_name: #export_name, 
                types: #sqtypes, 
                return_type: #out, 
                vm: ScriptVmType::#script_vm_type, 
                function: Some( #sq_functions_func ),
            }
        }
    }.into();

    out
}

// TODO: Rewrite concommand and convar to use the user's varible names and maybe types

// NOTES FOR cat_or_not
// so the convar callback is weird
// it returns a weird convar instead of the real one
// we can travel up the convar list to get ours but we can't get the values only the name
// since we know the name of the convar we can get it from g_pCVar but thats not exposed to plugins
// I think v3 plugins should have this :)
// once it does the convar proc marco should be updated to support it
// also if we are talking v3 plugins wishlist uwu
// 1. g_pCVar provided to plugins
// 2. client.dll, engine.dll and server.dll modules provided to plugins
// 3. runframe ran on plugins ;)
// ^ this would allow safe editing of convars and conconmands and also call any sqvm function "safely" :D

/// proc marco for generating compatible concommand callbacks
///
/// any arguments and return deffition are discarded
///
/// in favor of `convar` with `CCommandResult` which is created at compile time from the actuall passed arguments
#[proc_macro_attribute]
pub fn concommand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let ItemFn {
        attrs: _,
        vis,
        sig,
        block,
    } = input;

    let stmts = block.stmts;
    let ident = &sig.ident;
    let input = &sig.inputs;
    let output = &sig.output;

    let inner_call = if input.iter().count() != 0 {
        quote! {
            let ccommand = rrplug::high::concommands::CCommandResult::new(ccommand);

            _ = inner_function(ccommand);
        }
    } else {
        quote! {
            _ = ccommand; // so it doesn't complain about unused varibles

            _ = inner_function();
        }
    };

    // TODO: allow the users to manipulate the input of the inner function

    quote! {
        #vis unsafe extern "C" fn #ident (ccommand: *const rrplug::bindings::command::CCommand) {
            fn inner_function ( #input ) #output {
                #(#stmts)*
            }

            #inner_call
        }
    }
    .into()
}

/// proc marco for generating compatible concommand callbacks
///
/// any arguments and return deffition are discarded
///
/// are the actual agurments: convar: `Option<ConVarStruct>`, old_value: `String`, float_old_value: `f32`
///
/// the convar that is passed to this callback is always garbage data so you will have to bring the convar from a `static`
#[proc_macro_attribute]
pub fn convar(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let ItemFn {
        attrs: _,
        vis,
        sig,
        block,
    } = input;

    let stmts = block.stmts;
    let ident = &sig.ident;
    let input = &sig.inputs;
    let output = &sig.output;

    let inner_call = if input.iter().count() != 0 {
        quote! {
            let old_value = std::ffi::CStr::from_ptr(old_value).to_string_lossy().to_string();

            _ = inner_function(old_value,float_old_value);
        }
    } else {
        quote! {
            _ = (old_value,float_old_value); // so it doesn't complain about unused varibles

            _ = inner_function();
        }
    };

    quote! {
        #vis unsafe extern "C" fn #ident (
            convar: *mut rrplug::bindings::convar::ConVar,
            old_value: *const ::std::os::raw::c_char,
            float_old_value: f32
        ) {
            fn inner_function ( #input ) #output {
                #(#stmts)*
            }

            #inner_call
        }
    }
    .into()
}

#[proc_macro_derive(GetFromSquirrelVm)]
pub fn get_from_sqvm_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    impl_struct_or_enum(input, get_from_sqvm_impl_struct, get_from_sqvm_impl_enum)
}

#[proc_macro_derive(PushToSquirrelVm)]
pub fn push_to_sqvm_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    impl_struct_or_enum(input, push_to_sqvm_impl_struct, push_to_sqvm_impl_enum)
}

#[proc_macro_derive(GetFromSQObject)]
pub fn get_from_sqobject_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    get_from_sqobject_impl_enum(input)
}
