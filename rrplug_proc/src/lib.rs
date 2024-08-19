extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    parse, parse_macro_input, parse_str, punctuated::Punctuated, spanned::Spanned,
    AngleBracketedGenericArguments, DeriveInput, Error as SynError, FnArg, Ident, ImplItem,
    ImplItemFn, ItemFn, ItemImpl, ReturnType, Stmt, Token, Type, TypePath,
};

#[macro_use]
pub(crate) mod parsing;
pub(crate) mod impl_traits;

use impl_traits::{
    get_from_sqobject_impl_enum, get_from_sqobject_impl_struct, get_from_sqvm_impl_enum,
    get_from_sqvm_impl_struct, impl_struct_or_enum, push_to_sqvm_impl_enum,
    push_to_sqvm_impl_struct, sqvm_name_impl,
};
use parsing::{filter_args, get_arg_ident, input_mapping, Args};

// TODO: add multiple vm targets to sqfunction

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
/// ## Traits
/// this macro heavily relies on traits from rrplug and only exists to generate a parsing code.
///
/// refer to the traits for more info
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
    let ident = &sig.ident;
    let input = &sig.inputs;
    let input_vec: Vec<FnArg> = input.iter().filter_map(|arg| filter_args(arg)).collect();
    let input_type_names = input_vec
        .iter()
        .filter_map(|_input| {
            if let FnArg::Typed(t) = _input {
                Some(t)
            } else {
                None
            }
        })
        .map(|arg| arg.ty.as_ref());
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
    let func_name = ident.to_string();
    let sq_functions_func: Ident = format_ident!("sq_func_{}", func_name.clone());
    let mut export_name = ident.to_string();

    let mut sq_stack_pos = 1;
    let mut sq_gets_stmts = Vec::new();

    let mut out: Box<Type> = match output {
        syn::ReturnType::Default => {
            match parse_str::<Type>("()").map_err(|err| err.to_compile_error().into()) {
                Ok(v) => Box::new(v),
                Err(err) => return err,
            }
        }
        syn::ReturnType::Type(_, ty) => ty.clone(),
    };

    match input_mapping(input, &mut sq_stack_pos) {
        Ok(tks) => {
            for tk in tks {
                sq_gets_stmts.push(parse_macro_input!(tk as Stmt));
            }
        }
        Err(err) => {
            return err.to_compile_error().into();
        }
    }

    let mut script_vm: Punctuated<TypePath, Token![|]> = Punctuated::new();

    if !args.iter().any(|arg| arg.ident.to_string() == "VM") {
        return SynError::new(
            Span::mixed_site(),
            "consider specifying a VM parameter in macro's attributes. ex : VM = \"UI | CLIENT\"",
        )
        .to_compile_error()
        .into();
    }

    for arg in args {
        let input = arg.arg.to_token_stream().to_string().replace('"', "");
        match &arg.ident.to_string()[..] {
            "VM" => {
                if !input.is_empty() {
                    match input
                        .split('|')
                        .map(|vm| "SQFunctionContext::".to_string() + vm.to_uppercase().as_str())
                        .map(|vm| parse_str::<TypePath>(&vm))
                        .collect::<Result<Vec<TypePath>, SynError>>()
                    {
                        Ok(exprs) => exprs.into_iter().for_each(|expr| script_vm.push(expr)),
                        Err(err) => return err.to_compile_error().into(),
                    }
                }
            }
            "ExportName" => export_name = input,
            "ReturnOverwrite" => {
                out = match parse_str::<Type>(&input).map_err(|err| err.to_compile_error().into()) {
                    Ok(v) => Box::new(v),
                    Err(err) => return err,
                }
            }
            _ => {
                return SynError::new(
                    arg.ident.span(),
                    format!("wrong arg \"{}\" or arg {}", input, arg.ident.to_string()),
                )
                .to_compile_error()
                .into();
            }
        }
    }

    let out: TokenStream = quote! {
        #[doc(hidden)]
        #[doc = "generated ffi function for #func_name"]
        #vis extern "C" fn #sq_functions_func (sqvm: *mut rrplug::bindings::squirreldatatypes::HSquirrelVM) -> rrplug::bindings::squirrelclasstypes::SQRESULT {
            let sqvm = std::ptr::NonNull::new(sqvm).expect("sqvm has to be non null");
            use rrplug::high::squirrel_traits::{GetFromSquirrelVm,ReturnToVm};
            let sq_functions = SQFUNCTIONS.from_sqvm(sqvm);

            #[allow(unused)]
            let mut current_stack_pos = 1;

            #(#sq_gets_stmts)*

            fn inner_function( sqvm: std::ptr::NonNull<rrplug::bindings::squirreldatatypes::HSquirrelVM>, sq_functions: &'static SquirrelFunctions #(, #input_vec)* ) #output {
                let engine_token = unsafe { rrplug::high::engine::EngineToken::new_unchecked() };
                #(#stmts)*
            }

            inner_function( sqvm, sq_functions #(, #input_var_names)* ).return_to_vm(sqvm, sq_functions)
        }

        #(#attrs)*
        #vis fn #ident () -> rrplug::mid::squirrel::SQFuncInfo {
            use rrplug::high::squirrel_traits::SQVMName;
            use rrplug::mid::squirrel::SQFunctionContext;

            let mut types = String::new();
            #(
                if ( !types.is_empty() ) {
                    types.push(',');
                    types.push(' ');
                }
                types.push_str(&<#input_type_names as SQVMName>::get_sqvm_name());
                types.push(' ');
                types.push_str(stringify!(#input_var_names));
            )*

            rrplug::mid::squirrel::SQFuncInfo {
                cpp_func_name: #func_name,
                sq_func_name: #export_name,
                types: types,
                return_type: <#out as SQVMName>::get_sqvm_name(),
                vm: #script_vm,
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
            let ccommand = rrplug::high::engine::concommands::CCommandResult::new(ccommand);

            _ = inner_function(ccommand);
        }
    } else {
        quote! {
            _ = ccommand; // so it doesn't complain about unused varibles

            _ = inner_function();
        }
    };

    quote! {
        #(#attrs)*
        #vis unsafe extern "C" fn #ident (ccommand: *const rrplug::bindings::cvar::command::CCommand) {
            fn inner_function ( #input ) #output {
                let engine_token = unsafe { rrplug::high::engine::EngineToken::new_unchecked() };
                #(#stmts)*
            }

            #inner_call
        }
    }
    .into()
}

/// macro that transforms a function into a that can be used for completion by the engine while also adding a few abstraction layers
///
/// uses rrplug's `CommandCompletion` and `CurrentCommand` to abstract unsafer interactions with the completion buffer
///
/// the target function has to simply accept `CommandCompletion` and `CurrentCommand` as agurements for the macro to know where to pass them
///
/// refer to rrplug's `CommandCompletion` and `register_concommand_with_completion` for more info and examples
#[proc_macro_attribute]
pub fn completion(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let stmts = block.stmts;
    let ident = &sig.ident;
    let mut inputs = sig.inputs.iter();
    let output = &sig.output;

    if sig.inputs.len() > 2 {
        return quote_spanned! {
            sig.inputs.span() => {
                compile_error!("can only have two args");
            }
        }
        .into();
    }

    if inputs
        .next()
        .map(|arg| {
            arg.to_token_stream()
                .to_string()
                .ends_with("CurrentCommand")
                .then_some(())
        })
        .is_none()
    {
        return quote_spanned! {
            sig.inputs.span() => {
                compile_error!("the first arg must have type a of CurrentCommand");
            }
        }
        .into();
    }

    if inputs
        .next()
        .map(|arg| {
            arg.to_token_stream()
                .to_string()
                .ends_with("CommandCompletion")
                .then_some(())
        })
        .is_none()
    {
        return quote_spanned! {
            sig.inputs.span() => {
                compile_error!("the second arg must have a type of CommandCompletion");
            }
        }
        .into();
    }

    let ident1 = get_arg_ident(&sig.inputs[0]).unwrap();
    let ident2 = get_arg_ident(&sig.inputs[1]).unwrap();

    quote! {
        #(#attrs)*
        #vis unsafe extern "C" fn #ident (
            partial: *const std::ffi::c_char,
            commands: *mut [std::ffi::c_char;rrplug::bindings::cvar::convar::COMMAND_COMPLETION_ITEM_LENGTH as usize],
        ) -> i32 {
            let current = rrplug::high::engine::concommands::CurrentCommand::new(partial).unwrap();
            let mut suggestions = rrplug::high::engine::concommands::CommandCompletion::from(commands);


            fn inner_function ( #ident1 : rrplug::high::engine::concommands::CurrentCommand, #ident2: &mut rrplug::high::engine::concommands::CommandCompletion ) #output {
                let engine_token = unsafe { rrplug::high::engine::EngineToken::new_unchecked() };
                #(#stmts)*
            }

            _ = inner_function(current, &mut suggestions);

            suggestions.commands_used()
        }
    }
    .into()
}

/// proc marco for generating compatible convar callbacks
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
                let engine_token = unsafe { rrplug::high::engine::EngineToken::new_unchecked() };
                #(#stmts)*
            }

            #inner_call
        }
    }
    .into()
}

/// creates a valid source interface layout from the provided impl block
///
/// implements `AsInterface` for the sturct and transforms the functions from the impl block into extern "C" to put them into a vtable
///
/// the vtable won't contain the constructor :p
///
/// more info at `register_interface`
#[proc_macro_attribute]
pub fn as_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let ItemImpl {
        attrs,
        defaultness: _,
        unsafety: _,
        impl_token,
        generics,
        trait_,
        self_ty,
        brace_token: _,
        items,
    } = input;

    let self_ty_ident: Type = match parse_str(
        self_ty
            .to_token_stream()
            .to_string()
            .split('<')
            .nth(0)
            .unwrap(),
    ) {
        Ok(ty) => ty,
        Err(err) => return err.to_compile_error().into(),
    };
    let generics_bracked: Option<AngleBracketedGenericArguments> = self_ty
        .to_token_stream()
        .to_string()
        .split_once('<')
        .map(|(_, generics)| format!("<{}>", generics))
        .map(|generic_str| parse_str(&generic_str).ok())
        .flatten();

    if let Some(trait_) = trait_ {
        let error_site = trait_.1.span();
        return quote_spanned! {error_site => compile_error!("interfaces must not be a implementation for a trait");}.into();
    }

    let mut funcs = match items
        .into_iter()
        .map(|item| match (item.span(), item) {
            (_, ImplItem::Fn(func)) => Ok(func),
            (span, _) => Err(span),
        })
        .collect::<Result<Vec<ImplItemFn>, Span>>()
    {
        Ok(funcs) => funcs,
        Err(error_site) => {
            return quote_spanned! {error_site => compile_error!("can only have functions in interfaces");}.into()
        }
    };

    let mut new_func = match funcs
        .iter()
        .position(|func| func.sig.ident.to_string() == "new")
    {
        Some(index) => funcs.remove(index),
        None => {
            return quote! {compile_error!("a interface must have a new function to initialize it");}
                .into()
        }
    };

    // the most sane solution me thinks
    new_func = {
        let ImplItemFn {
            attrs,
            vis,
            defaultness,

            mut sig,
            block,
        } = new_func;
        let stmts = block.stmts;
        sig.output = parse_str("-> rrplug::interfaces::interface::Interface<Self>").unwrap();

        let tk = quote! {
            #(#attrs)*
            #vis #defaultness #sig {
                use rrplug::interfaces::interface::AsInterface;
                Self::to_interface({
                    #(#stmts)*
                })
            }
        };
        match parse(tk.into()) {
            Ok(s) => s,
            Err(err) => return err.to_compile_error().into(),
        }
    };

    if funcs.is_empty() {
        return quote! {compile_error!("perhaps you should add a function to your interface");}
            .into();
    }

    let function_idents = funcs
        .iter()
        .map(|func| &func.sig.ident)
        .collect::<Vec<&Ident>>();

    if let Some(error_site) = funcs
        .iter()
        .find(|func| {
            func.sig
                .inputs
                .first()
                .map(|selfarg| match selfarg {
                    FnArg::Receiver(recveiver)
                        if recveiver.reference.is_some() && recveiver.mutability.is_none() =>
                    {
                        Some(())
                    }
                    _ => None,
                })
                .flatten()
                .is_none()
        })
        .map(|func| func.sig.inputs.span())
    {
        return quote_spanned! {error_site => compile_error!("functions must have &self");}.into();
    }

    let extern_inputs = funcs
        .iter()
        .map(|func| {
            func.sig
                .inputs
                .iter()
                .skip(1)
                .collect::<Punctuated<&FnArg, Token![,]>>()
        })
        .collect::<Vec<Punctuated<&FnArg, Token![,]>>>();
    let extern_inputs_idents = extern_inputs
        .iter()
        .map(|inputs| {
            inputs.iter().filter_map(|input| get_arg_ident(input)).fold(
                Punctuated::new(),
                |mut acc, ident| {
                    acc.push(ident);
                    acc
                },
            )
        })
        .collect::<Vec<Punctuated<Ident, Token![,]>>>();
    let extern_outputs = funcs
        .iter()
        .map(|func| &func.sig.output)
        .collect::<Vec<&ReturnType>>();

    quote! {
        #(#attrs)*
        #impl_token #generics #self_ty {
            #(#funcs)*
            #new_func
        }

        #(#attrs)*
        impl #generics rrplug::interfaces::interface::AsInterface for #self_ty {
            fn to_interface(self) -> rrplug::interfaces::interface::Interface<Self> {
                    // make these extern c wrapped and offset the self since it will have the vtable
                // TODO: make generic functions work (big headache)
                #(
                    #[allow(unsafe_op_in_unsafe_fn)]
                    unsafe extern "C" fn #function_idents #generics(self_: *const std::ffi::c_void, #extern_inputs) #extern_outputs {
                        // transmute because I want to be lazy here
                        #self_ty_ident::#generics_bracked #function_idents(self_.cast::<usize>().add(1).cast::<#self_ty_ident>().as_ref().expect("how!"), #extern_inputs_idents )
                    }
                )*

                const VTABLE: &[*const std::ffi::c_void] = &[#(#function_idents #generics_bracked as *const std::ffi::c_void,)*];

                rrplug::interfaces::interface::Interface::new(
                    unsafe { std::ptr::NonNull::new_unchecked(VTABLE.as_ptr().cast_mut()) },
                    self
                )
            }
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

/// macro to auto generate a `GetFromSQObject` implementation for enums and structs behaves mostly like `GetFromSquirrelVm`
///
/// since squirrel's enums are integers the enum must be a unit-only enum
///
/// maybe also use `#[repr(i32)]` idk
#[proc_macro_derive(GetFromSQObject)]
pub fn get_from_sqobject_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    impl_struct_or_enum(
        input,
        get_from_sqobject_impl_struct,
        get_from_sqobject_impl_enum,
    )
}

/// macro to auto generate a `SQVMName` implementation
///
/// the implementation will just be the name of the struct/enum so if the squirrel name is diffrent use a util macro in `rrplug::macro::utils`
#[proc_macro_derive(SQVMName)]
pub fn sqvm_name_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    sqvm_name_impl(input)
}
