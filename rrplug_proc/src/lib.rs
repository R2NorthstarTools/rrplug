// REMINDER: THIS CRATE MIGHT NEEDS TO BE A SEPARETE CRATE ON CRATES.io

extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{self, parse_macro_input, FnArg, Ident, ItemFn, Result, Stmt, Token, Type};

struct Arg {
    ident: Ident,
    arg: Ident,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
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
    fn parse(input: ParseStream) -> Result<Self> {
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

#[proc_macro_attribute]
pub fn sqfunction(attr: TokenStream, item: TokenStream) -> TokenStream {
    // println!("attr: \"{}\"", attr.to_string());
    // println!("item: \"{}\"", item.to_string());

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

    let input = parse_macro_input!(item as ItemFn);

    let ItemFn {
        attrs: _,
        vis: _,
        sig,
        block,
    } = input;
    let mut stmts = block.stmts;
    let mut sqtypes = String::new();
    let ident = &sig.ident;
    let input = &sig.inputs;
    let output = &sig.output;
    let func_name = ident.to_string();
    let cpp_ident = format_ident!(
        "info_{}", func_name.clone(),
    );

    let mut sq_stack_pos = 1;
    let mut sq_gets_stmts = Vec::new();

    let out = match output {
        syn::ReturnType::Default => "void",
        syn::ReturnType::Type(_, ty) => match &**ty {
            Type::Path(type_path) if type_path.to_token_stream().to_string() == "bool" => "bool",
            Type::Path(type_path) if type_path.to_token_stream().to_string() == "i32" => "int",
            Type::Path(type_path) if type_path.to_token_stream().to_string() == "f32" => "float",
            Type::Path(type_path) if type_path.to_token_stream().to_string() == "String" => {
                "string"
            }
            _ => "var",
        },
    };

    for i in input.iter() {
        match i.to_owned() {
            FnArg::Receiver(_) => {
                let tk = quote! {
                    compile_error!("wtf are you doing? stop this now! we don't support methods");
                }
                .into();
                push_stmts!(sq_gets_stmts, tk);
            }
            FnArg::Typed(t) => match &*t.ty {
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "bool" => {
                    let name = t.clone().pat.to_token_stream();
                    push_type!(sqtypes, "bool", &name.to_string()[..]);
                    let tk = quote! {let #name = unsafe { (sq_functions.sq_getbool)(sqvm, #sq_stack_pos) } == 1;}.into();
                    push_stmts!(sq_gets_stmts, tk);

                    sq_stack_pos += 1;
                }
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "i32" => {
                    let name = t.clone().pat.to_token_stream();
                    push_type!(sqtypes, "int", &name.to_string()[..]);
                    let tk = quote! {let #name = unsafe { (sq_functions.sq_getinteger)(sqvm, #sq_stack_pos) } as i32;}.into();
                    push_stmts!(sq_gets_stmts, tk);

                    sq_stack_pos += 1;
                }
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "f32" => {
                    let name = t.clone().pat.to_token_stream();
                    push_type!(sqtypes, "float", &name.to_string()[..]);
                    let tk = quote! {let #name = unsafe { (sq_functions.sq_getfloat)(sqvm, #sq_stack_pos) } as f32;}.into();
                    push_stmts!(sq_gets_stmts, tk);

                    sq_stack_pos += 1;
                }
                Type::Path(type_path) if type_path.to_token_stream().to_string() == "String" => {
                    let name = t.clone().pat.to_token_stream();
                    push_type!(sqtypes, "string", &name.to_string()[..]);
                    let tk = quote! {
                        let #name = unsafe {
                            let _sq_str = (sq_functions.sq_getstring)(sqvm, #sq_stack_pos);
                            let _c_str = std::ffi::CStr::from_ptr(_sq_str);
                            String::from_utf8_lossy(_c_str.to_bytes()).to_string()
                        };
                    }
                    .into();
                    push_stmts!(sq_gets_stmts, tk);

                    sq_stack_pos += 1;
                }

                // soon
                // Type::BareFn(_) => todo!(),

                // maybe
                // Type::Array(_) => todo!(),
                // Type::Slice(_) => todo!(),
                // Type::Tuple(_) => todo!(),
                // Type::Reference(_) => todo!(),

                // will not support
                // Type::Infer(_) => todo!(),
                // Type::Macro(_) => todo!(),
                // Type::Never(_) => todo!(),
                // Type::Paren(_) => todo!(),
                // Type::Group(_) => todo!(),
                // Type::ImplTrait(_) => todo!(),
                // Type::TraitObject(_) => todo!(),
                // Type::Verbatim(_) => todo!(),
                // Type::Ptr(_) => todo!(),
                _ => {
                    let _ty = format!(
                        "{} type isn't supported",
                        t.ty.into_token_stream().to_string()
                    );
                    let tk = quote! {
                        compile_error!(#_ty);
                    }
                    .into();
                    push_stmts!(sq_gets_stmts, tk);
                }
            },
        }
    }

    sq_gets_stmts.reverse();
    for s in sq_gets_stmts {
        stmts.insert(0, s);
    }

    let mut script_vm_func = "client";
    let mut script_vm = "client";

    for arg in args {
        let input = arg.arg.to_string();
        match &arg.ident.to_string()[..] {
            "VM" if input.to_uppercase().ends_with("UI") => {script_vm = "Ui"; script_vm_func = "client";},
            "VM" if input.to_uppercase().ends_with("SERVER") => {script_vm = "Server"; script_vm_func = "server";},
            "VM" if input.to_uppercase().ends_with("CLIENT") => {script_vm = "Client"; script_vm_func = "client";},
            _ => {
                let fmt = format!("wrong arg {} or arg {}", input, arg.ident.to_string());
                let tk = quote! {
                    compile_error!(#fmt);
                }
                .into();
                push_stmts!(stmts, tk);
            }
        }
    }
    let script_vm_type = format_ident!(
        "{script_vm}" 
    );
    let script_vm_func = format_ident!("{}", script_vm_func);

    let tk = quote! {let sq_functions = unsafe {SQFUNCTIONS.#script_vm_func.as_ref().unwrap()};}.into();
    push_stmts!(stmts, tk);

    let out: TokenStream = quote! {
        extern "C" fn #ident (sqvm: *mut rrplug::bindings::squirreldatatypes::HSquirrelVM) -> ::std::os::raw::c_int {
            #(#stmts)*
        }

        const fn #cpp_ident () -> rrplug::wrappers::northstar::SQFuncInfo {
            
            (#func_name, #func_name, #sqtypes, #out, rrplug::wrappers::northstar::ScriptVmType::#script_vm_type, #ident )
        }
    }.into();

    // println!("out: \"{}\"", out.to_string());
    out
}
