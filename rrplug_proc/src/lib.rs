extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    self, parse_macro_input, parse_quote, DeriveInput, FnArg, Ident, ItemFn, ReturnType, Stmt, Error as SynError, Type, parse_str
};

#[macro_use]
pub(crate) mod parsing;
pub(crate) mod impl_traits;

use impl_traits::{sqvm_name_impl, const_sqvm_name_impl, impl_struct_or_enum, push_to_sqvm_impl_struct, push_to_sqvm_impl_enum, get_from_sqvm_impl_enum, get_from_sqvm_impl_struct, get_from_sqobject_impl_enum};
use parsing::{filter_args, input_mapping, Args};

// TODO: trait for tranlating types into sqtypes 

/// proc marco for generating compatible functions with the sqvm. 
///
///  ## abstractions
/// the macro uses arguments types and return types to tranlates them into a sqfunction deffintion
/// `GetFromSquirrelVm` and `PushToSquirrelVm` define logic for how
/// 
/// ## attributes
/// - **VM**
/// 
///     Indicates for which the sqfunction is created
///     .The default is `Client`
/// - **ExportName**
/// 
///     Specifies the name for the function on the sqvm
/// - **ReturnOverwrite**
/// 
///     Overwrites the return type for the sqfunction definition
///     Useful for ensuring type safety for custom structs and other custom types since they default to `var`.
#[proc_macro_attribute]
pub fn sqfunction(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as Args).args;

    let input = parse_macro_input!(item as ItemFn);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let stmts = block.stmts;
    let mut sub_stms = Vec::new();
    let mut sqtypes = String::new();
    let ident = &sig.ident;
    let input = &sig.inputs;
    let input_vec: Vec<FnArg> = input.iter().filter_map(|arg| filter_args(arg)).collect();
    let input_type_names = input_vec.iter().filter_map(|_input| {
        if let FnArg::Typed(t) = _input {
            Some(t)
        } else {
            None
        }
    }).map(|arg| arg.ty.as_ref());
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

    let mut out: Box<Type> = match output {
        syn::ReturnType::Default => match parse_str::<Type>("std::ffi::c_void").map_err(|err| err.to_compile_error().into()) {
            Ok(v) => Box::new(v),
            Err(err) => return err,
        },
        syn::ReturnType::Type(_, ty) => ty.clone(),
    };
    
    // TODO: remove this later too lazy rn
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
                return SynError::new(
                    arg.ident.span(),
                    format!("invalid VM {}", input)
                ).to_compile_error()
                .into();
            }
            "ExportName" => export_name = input,
            "ReturnOverwrite" => out = match parse_str::<Type>(&input).map_err(|err| err.to_compile_error().into()) {
                Ok(v) => Box::new(v),
                Err(err) => return err,
            },
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
                    let err = rrplug::to_c_string!(err);
                    unsafe { (sq_functions.sq_raiseerror)(sqvm, err.as_ptr()) };
                    rrplug::bindings::squirrelclasstypes::SQRESULT::SQRESULT_ERROR 
                }
            }
        }
        
        #(#attrs)*
        #vis fn #ident () -> rrplug::high::northstar::SQFuncInfo {
            use rrplug::high::squirrel_traits::{SQVMName,ConstSQVMName};
            
            let mut types = String::new();
            #(
                if ( !types.is_empty() ) {
                    types.push(',');
                    types.push(' ');
                }
                types.push_str(&<#input_type_names>::get_sqvm_name());
                types.push(' ');
                types.push_str(stringify!(#input_var_names));
            )*

            // this is ok since the string has to be alive for the lifetime of the game
            // this can't be done at comp time since implemented consts can't be used for other implementations of consts :|

            rrplug::high::northstar::SQFuncInfo{ 
                cpp_func_name: #func_name, 
                sq_func_name: #export_name, 
                types: if types.is_empty() {""} else {types.leak()}, 
                return_type: #out::SQ_NAME, 
                vm: ScriptVmType::#script_vm_type, 
                function: Some( #sq_functions_func ),
            }
        }
    }.into();

    out
}

/// proc marco for generating compatible concommand callbacks
/// 
/// this macro wraps your function in another function and tries to provide the argurments your requested
/// 
/// # how to use it
/// the 2 possible function signatures are
/// - `fn(CCommandResult)`
/// - `fn()`
/// 
/// the result is ignored so it can be anything
#[proc_macro_attribute]
pub fn concommand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let ItemFn {
        attrs,
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
        #(#attrs)*
        #vis unsafe extern "C" fn #ident (ccommand: *const rrplug::bindings::cvar::command::CCommand) {
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
/// this macro wraps your function in another function and tries to provide the argurments your requested
/// 
/// # how to use it
/// the 2 possible function signatures are
/// - `fn(String, f32)`
/// - `fn()`
/// 
/// the result is ignored so it can be anything
#[proc_macro_attribute]
pub fn convar(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let ItemFn {
        attrs,
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
        #(#attrs)*
        #vis unsafe extern "C" fn #ident (
            convar: *mut rrplug::bindings::cvar::convar::ConVar,
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

/// implements `GetFromSquirrelVm` for structs or enums
/// 
/// the fields of the struct must implement `GetFromSQObject`
/// 
/// the enum must be unit-only
#[proc_macro_derive(GetFromSquirrelVm)]
pub fn get_from_sqvm_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    impl_struct_or_enum(input, get_from_sqvm_impl_struct, get_from_sqvm_impl_enum)
}

/// implements `PushToSquirrelVm` for structs or enums
/// 
/// the fields of the struct must implement `PushToSquirrelVm`
/// 
/// the enum must be unit-only
#[proc_macro_derive(PushToSquirrelVm)]
pub fn push_to_sqvm_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    impl_struct_or_enum(input, push_to_sqvm_impl_struct, push_to_sqvm_impl_enum)
}

/// macro to auto generate a `GetFromSQObject` implementation for enums
/// 
/// since squirrel's enums are integers the enum must be a unit-only enum
/// 
/// maybe also use `#[repr(i32)]` idk
#[proc_macro_derive(GetFromSQObject)]
pub fn get_from_sqobject_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    get_from_sqobject_impl_enum(input)
}

/// macro to auto generate a `ConstSQVMName` implementation
/// 
/// the implementation will just be the name of the struct/enum so if the squirrel name is diffrent use a util macro in `rrplug::macro::utils` 
#[proc_macro_derive(ConstSQVMName)]
pub fn const_sqvm_name_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    const_sqvm_name_impl(input)
}

/// macro to auto generate a `SQVMName` implementation
/// 
/// the implementation will just be the name of the struct/enum so if the squirrel name is diffrent use a util macro in `rrplug::macro::utils` 
#[proc_macro_derive(SQVMName)]
pub fn sqvm_name_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    sqvm_name_impl(input)
}
