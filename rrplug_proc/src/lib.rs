extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    self, parse_macro_input, FnArg, Ident, ItemFn, Result as SynResult, ReturnType, Stmt, Token,
    Type,
};

struct Arg {
    ident: Ident,
    arg: Ident,
}

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

    // type, name, token stream
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
                            rrplug::wrappers::vector::Vector3::from( (sq_functions.sq_getvector)(sqvm, #sq_stack_pos) )
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
                            let mut obj = Box::new(std::mem::MaybeUninit::<SQObject>::zeroed());
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
    let mut stmts = block.stmts;
    let mut sqtypes = String::new();
    let ident = &sig.ident;
    let input = &sig.inputs;
    let output = &sig.output;
    let default_sq_output = "-> rrplug::bindings::squirrelclasstypes::SQRESULT";
    let mut default_sq_output: ReturnType = syn::parse_str(default_sq_output).expect("boom");
    let func_name = ident.to_string();
    let mut export_name = ident.to_string();
    let cpp_ident = format_ident!("info_{}", func_name.clone());

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
        stmts.insert(0, s);
    }

    let mut script_vm_func = "client";
    let mut script_vm = "Client";
    let mut is_message = false;

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
            "SQMessage" => {
                is_message = true;
                default_sq_output = ReturnType::Default;
            } // this has to be finshed some day, rn asycn fn calls with i32 return don't cause problems
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
    let script_vm_func = format_ident!("{script_vm_func}" );

    let tk = quote! {let sq_functions = SQFUNCTIONS.#script_vm_func.wait();}.into();
    push_stmts!(stmts, tk);

    let mut info_func = quote! {
        #vis const fn #cpp_ident () -> rrplug::wrappers::northstar::SQFuncInfo {

            (#func_name, #export_name, #sqtypes, #out, rrplug::wrappers::northstar::ScriptVmType::#script_vm_type, Some(#ident) )
        }
    };

    if is_message {
        info_func = quote!("");
    }

    let out: TokenStream = quote! {
        extern "C" fn #ident (sqvm: *mut rrplug::bindings::squirreldatatypes::HSquirrelVM) #default_sq_output {
            #(#stmts)*
        }

        #info_func
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

    let mut stmts = block.stmts;
    let ident = &sig.ident;

    let tk = quote! {
        let command = rrplug::Lazy::<rrplug::wrappers::concommands::CCommandResult,_>::new( || rrplug::wrappers::concommands::CCommandResult::from(command) );
    }
        .into();
    let new_stmt = parse_macro_input!(tk as Stmt);
    stmts.insert(0, new_stmt);

    quote! {
        #vis extern "C" fn #ident (command: *const rrplug::bindings::command::CCommand) {
            #(#stmts)*
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

    let mut stmts = block.stmts;
    let ident = &sig.ident;

    let tk = quote! {
        let convar: Option<rrplug::wrappers::convars::ConVarStruct> = None;
    }
    .into();
    let new_stmt = parse_macro_input!(tk as Stmt);
    stmts.insert(0, new_stmt);

    let tk = quote! {
        let old_value = unsafe { std::ffi::CStr::from_ptr(old_value).to_string_lossy().to_string() };
    }
    .into();
    let new_stmt = parse_macro_input!(tk as Stmt);
    stmts.insert(0, new_stmt);

    quote! {
        #vis extern "C" fn #ident (
            convar: *mut rrplug::bindings::convar::ConVar,
            old_value: *const ::std::os::raw::c_char,
            float_old_value: f32
        ) {
            #(#stmts)*
        }
    }
    .into()
}
