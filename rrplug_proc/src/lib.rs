extern crate proc_macro;

use proc_macro::{TokenStream};
use syn::__private::TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    self, parse_macro_input, FnArg, Ident, ItemFn, Result as SynResult, ReturnType, Stmt, Token,
    Type, parse_quote,
};

struct Arg {
    ident: Ident,
    arg: Ident,
}

// TODO: rewrite this with string literals since this garbage
impl Parse for Arg {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let ident = input.parse()?;
        _ = input.parse::<Token![=]>()?;
        let arg = input.parse()?;
        Ok(Self { ident, arg })
    }
}

struct Args {
    args: Vec<Arg>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut args = Args { args: Vec::new() };

        loop {
            args.args.push(input.parse::<Arg>()?);
            if input.peek(Token![,]) {
                _ = input.parse::<Token![,]>();
            } else {
                break;
            }
        }
        Ok(args)
    }
}

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

    // otherwise the proc marco is blind :skull:
    macro_rules! push_type {
        ($var:ident, $sqtype:expr, $name:expr) => {
            if !$var.is_empty() {
                $var.push(',');
                $var.push(' ');
            }
            $var.push_str($sqtype);
            $var.push(' ');
            $var.push_str($name);
        };
    }

    macro_rules! push_stmts {
        ($stmts:ident, $tk:ident) => {
            let new_stmt = parse_macro_input!($tk as Stmt);
            $stmts.insert(0, new_stmt);
        };
    }

    fn recursive_type_match(t: Type) -> Result<String, String> {
        match t {
            Type::Path(type_path) if type_path.to_token_stream().to_string() == "bool" => {
                Ok("bool".into())
            }
            Type::Path(type_path) if type_path.to_token_stream().to_string() == "i32" => {
                Ok("int".into())
            }
            Type::Path(type_path) if type_path.to_token_stream().to_string() == "f32" => {
                Ok("float".into())
            }
            Type::Path(type_path) if type_path.to_token_stream().to_string() == "String" => {
                Ok("string".into())
            }
            Type::Path(type_path)
                if type_path.to_token_stream().to_string().ends_with("Vector3") =>
            {
                Ok("vector".into())
            }

            Type::BareFn(fun) => {
                let head_types = format!("{} functionref( ", get_sqoutput(&fun.output));
                let mut func_args = String::new();

                for arg in fun.inputs {
                    push_type!(func_args, &recursive_type_match(arg.ty)?[..], "");
                }

                Ok(format!("{head_types}{func_args})"))
            }

            _ => Err(format!(
                "{} type isn't supported",
                t.into_token_stream().to_string()
            )),
        }
    }

    /// type, name, token stream
    fn match_input(
        arg: &FnArg,
        sq_stack_pos: i32,
    ) -> Result<(String, String, TokenStream), String> {
        match arg.to_owned() {
            FnArg::Receiver(_) => {
                Err("wtf are you doing? stop this now! rrplug doesn't support methods".into())
            }
            FnArg::Typed(t) => match &*t.ty {
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "bool" => {
                    let name = t.clone().pat.to_token_stream();
                    let tk = quote! {let #name: bool = unsafe { (sq_functions.sq_getbool)(sqvm, #sq_stack_pos) } != 0;}.into();
                    Ok(("bool".into(), name.to_string(), tk))
                }
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "i32" => {
                    let name = t.clone().pat.to_token_stream();
                    let tk = quote! {let #name = unsafe { (sq_functions.sq_getinteger)(sqvm, #sq_stack_pos) } as i32;}.into();
                    Ok(("int".into(), name.to_string(), tk))
                }
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "f32" => {
                    let name = t.clone().pat.to_token_stream();
                    let tk = quote! {let #name = unsafe { (sq_functions.sq_getfloat)(sqvm, #sq_stack_pos) } as f32;}.into();
                    Ok(("float".into(), name.to_string(), tk))
                }
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "String" => {
                    let name = t.clone().pat.to_token_stream();
                    let tk = quote! {
                        let #name = unsafe {
                            let _sq_str = (sq_functions.sq_getstring)(sqvm, #sq_stack_pos);
                            let _c_str = std::ffi::CStr::from_ptr(_sq_str);
                            String::from_utf8_lossy(_c_str.to_bytes()).to_string()
                        };
                    }
                    .into();
                    Ok(("string".into(), name.to_string(), tk))
                }
                Type::Path(type_path)
                    if type_path.to_token_stream().to_string().ends_with("Vector3") =>
                {
                    let name = t.clone().pat.to_token_stream();
                    let tk = quote! {
                        let #name = unsafe {
                            rrplug::high::vector::Vector3::from( (sq_functions.sq_getvector)(sqvm, #sq_stack_pos) )
                        };
                    }
                    .into();
                    Ok(("vector".into(), name.to_string(), tk))
                }

                Type::BareFn(_) => {
                    let name = t.clone().pat.to_token_stream();
                    let sqty = recursive_type_match(*t.ty)?;
                    let tk = quote! {
                        let mut #name = unsafe {
                            let mut obj = Box::new(std::mem::MaybeUninit::<SQObject>::zeroed()); // TODO: import SQObject maybe? 
                            (sq_functions.sq_getobject)(sqvm, #sq_stack_pos, obj.as_mut_ptr());
                            obj
                        };
                    }
                    .into();

                    Ok((sqty, name.to_string(), tk))
                }

                _ => Err(format!(
                    "{} type isn't supported",
                    t.ty.into_token_stream().to_string()
                )),
            },
        }
    }

    fn input_mapping(
        args: &Punctuated<FnArg, Comma>,
        sqtypes: &mut String,
        sq_stack_pos: &mut i32,
    ) -> Result<Vec<TokenStream>, String> {
        let mut token_streams = Vec::new();

        for arg in args.iter() {
            let out = match_input(arg, *sq_stack_pos);

            let out = out?;

            push_type!(sqtypes, &out.0[..], &out.1[..]);
            token_streams.push(out.2);

            *sq_stack_pos += 1;
        }

        Ok(token_streams)
    }

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
    let input_vec = input
        .iter().map(|arg| 
            if let FnArg::Typed(t) = arg.clone() {
                 if let Type::BareFn(_) = &*t.ty {
                    let name = t.clone().pat.to_token_stream();
                    parse_quote!(mut #name: Box<std::mem::MaybeUninit<SQObject>> )
                 }
                 else {
                    arg.to_owned()
                }

        } else {
            arg.to_owned()
        } );
    let input_var_names: Vec<TokenStream2> = input.iter().cloned()
        .filter_map(|input| if let FnArg::Typed(t) = input { Some(t) } else { None } )
        .map::<TokenStream2,_>(|t| t.pat.into_token_stream().into() )
        .collect();
    let output = &sig.output;
    let (ouput_type,ouput_parsing) = match output.clone() {
        ReturnType::Default => (parse_quote!(()),quote!(rrplug::bindings::squirrelclasstypes::SQRESULT::SQRESULT_NULL)),
        ReturnType::Type(_, t) => 
        (t,quote!({
            use rrplug::high::squirrel_traits::PushToSquirrelVm;
            output.push_to_sqvm(sqvm, sq_functions);
            rrplug::bindings::squirrelclasstypes::SQRESULT::SQRESULT_NOTNULL
        })),
    };
    let func_name = ident.to_string();
    let sq_functions_func: Ident = format_ident!("sq_func_{}", func_name.clone());
    let mut export_name = ident.to_string();

    let mut sq_stack_pos = 1;
    let mut sq_gets_stmts = Vec::new();

    fn get_sqoutput(output: &ReturnType) -> &str {
        match output {
            syn::ReturnType::Default => "void",
            syn::ReturnType::Type(_, ty) => match &**ty {
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "bool" => {
                    "bool"
                }
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "i32" => "int",
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "f32" => {
                    "float"
                }
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "String" => {
                    "string"
                }
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "Vector3" => {
                    "vector"
                }
                Type::Path(type_path)
                    if type_path.to_token_stream().to_string().replace(' ', "")
                        == "Vec<String>" =>
                {
                    "array<string>"
                }
                Type::Path(type_path)
                    if type_path.to_token_stream().to_string().replace(' ', "")
                        == "Vec<Vector3>" =>
                {
                    "array<vector>"
                }
                Type::Path(type_path)
                    if type_path.to_token_stream().to_string().replace(' ', "") == "Vec<bool>" =>
                {
                    "array<bool>"
                }
                Type::Path(type_path)
                    if type_path.to_token_stream().to_string().replace(' ', "") == "Vec<i32>" =>
                {
                    "array<int>"
                }
                Type::Path(type_path)
                    if type_path.to_token_stream().to_string().replace(' ', "") == "Vec<f32>" =>
                {
                    "array<float>"
                }
                _ => "var",
            },
        }
    }

    let mut out = get_sqoutput(output);

    match input_mapping(input, &mut sqtypes, &mut sq_stack_pos) {
        Ok(tks) => {
            for tk in tks {
                push_stmts!(sq_gets_stmts, tk);
            }
        }
        Err(err) => {
            return quote! {
                compile_error!(#err);
            }
            .into()
        }
    }
    sq_gets_stmts.reverse();
    for s in sq_gets_stmts {
        sub_stms.insert(0, s);
    }

    let mut script_vm_func = "client";
    let mut script_vm = "Client";

    for arg in args {
        let input = arg.arg.to_string();
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
            "ExportName" => export_name = input,
            "ReturnOverwrite" => out = out,
            _ => {
                let fmt = format!("wrong arg {} or arg {}", input, arg.ident.to_string());
                return quote! {
                    compile_error!(#fmt);
                }
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
        #vis extern "C" fn #sq_functions_func (sqvm: *mut rrplug::bindings::squirreldatatypes::HSquirrelVM) -> rrplug::bindings::squirrelclasstypes::SQRESULT {
            
            #(#sub_stms)*
            
            #[allow(clippy::boxed_local)]
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
    
    // TODO: allow the users to manipulate the input of the inner function

    quote! {
        #vis unsafe extern "C" fn #ident (ccommand: *const rrplug::bindings::command::CCommand) {
            fn inner_function ( #input ) #output {
                #(#stmts)*
            }

            let ccommand = rrplug::high::concommands::CCommandResult::new(ccommand);

            _ = inner_function(ccommand);
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

    quote! {
        #vis unsafe extern "C" fn #ident (
            convar: *mut rrplug::bindings::convar::ConVar,
            old_value: *const ::std::os::raw::c_char,
            float_old_value: f32
        ) {
            fn inner_function ( #input ) #output {
                #(#stmts)*
            }

            let old_value = std::ffi::CStr::from_ptr(old_value).to_string_lossy().to_string();

            _ = inner_function(old_value,float_old_value);
        }
    }
    .into()
}
