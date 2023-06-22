use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    self, parse::Parse, parse::ParseStream, punctuated::Punctuated, token::Comma, FnArg, Ident,
    LitStr, Result as SynResult, ReturnType, Token, Type, __private::TokenStream2, parse_quote,
};

pub struct Arg {
    pub ident: Ident,
    pub arg: LitStr,
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

pub struct Args {
    pub args: Vec<Arg>,
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

#[allow(unused)] // TODO! Remove later
pub fn recursive_type_match(t: Type) -> Result<String, String> {
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
        Type::Path(type_path) if type_path.to_token_stream().to_string().ends_with("Vector3") => {
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

#[allow(unused)] // TODO! Remove later
/// type, name, token stream
pub fn match_input(
    arg: &FnArg,
    sq_stack_pos: i32,
) -> Result<(String, String, TokenStream), String> {
    match arg.to_owned() {
        FnArg::Receiver(_) => {
            Err("wtf are you doing? stop this now! rrplug doesn't support impl methods".into())
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
                        let mut obj = std::mem::MaybeUninit::<SQObject>::zeroed(); // TODO: import SQObject maybe?
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

pub fn input_mapping(
    args: &Punctuated<FnArg, Comma>,
    sqtypes: &mut String,
    sq_stack_pos: &mut i32,
) -> Result<Vec<TokenStream>, String> {
    let mut token_streams: Vec<TokenStream> = Vec::new();

    for arg in args.iter() {
        // let (ty,name,tk) = match_input(arg, *sq_stack_pos)?;
        let (sqty, name) = get_sqinput(arg);
        let ty = get_arg_type(arg)?;

        let tk = quote! {
                let #name: #ty = GetFromSquirrelVm::get_from_sqvm(sqvm, sq_functions, #sq_stack_pos);
        }.into();

        push_type!(sqtypes, &sqty, &name.to_string());
        token_streams.push(tk);

        *sq_stack_pos += 1;
    }

    Ok(token_streams)
}

pub fn filter_args(input: &FnArg) -> Option<FnArg> {
    let (ty, pt) = match input {
        FnArg::Receiver(_) => None?,
        FnArg::Typed(t) => (maybe_change(&t.ty), t.to_owned()),
    };

    let attrs = &pt.attrs;
    let pat = &pt.pat;

    Some(parse_quote!(#(#attrs)* #pat: #ty ))
}

pub fn get_arg_type(input: &FnArg) -> Result<Box<Type>, String> {
    match input {
        FnArg::Receiver(_) => Err(format!(
            "invalid arg {}",
            input.to_token_stream().to_string()
        )),
        FnArg::Typed(t) => Ok(maybe_change(&t.ty)),
    }
}

fn maybe_change(ty: &Type) -> Box<Type> {
    match *ty {
        Type::BareFn(_) => Box::new(parse_quote!(rrplug::high::squirrel::SQClosureHandle)),
        _ => Box::new(ty.clone()),
    }
}

/// type, name
pub fn get_sqinput(input: &FnArg) -> (String, TokenStream2) {
    match input {
        FnArg::Receiver(_) => ("".to_string(), TokenStream2::new()),
        FnArg::Typed(t) => (get_sqtype(&t.ty), t.clone().pat.into_token_stream().into()),
    }
}

pub fn get_sqoutput(output: &ReturnType) -> String {
    match output {
        syn::ReturnType::Default => "void".to_string(),
        syn::ReturnType::Type(_, ty) => get_sqtype(ty),
    }
}
pub fn get_sqtype(ty: &Box<Type>) -> String {
    match &**ty {
        Type::Path(type_path) if type_path.to_token_stream().to_string() == "bool" => {
            "bool".to_string()
        }
        Type::Path(type_path) if type_path.to_token_stream().to_string() == "i32" => {
            "int".to_string()
        }
        Type::Path(type_path) if type_path.to_token_stream().to_string() == "f32" => {
            "float".to_string()
        }
        Type::Path(type_path) if type_path.to_token_stream().to_string() == "String" => {
            "string".to_string()
        }
        Type::Path(type_path) if type_path.to_token_stream().to_string() == "Vector3" => {
            "vector".to_string()
        }
        Type::Path(type_path)
            if type_path.to_token_stream().to_string().replace(' ', "") == "Vec<String>" =>
        {
            "array<string>".to_string()
        }
        Type::Path(type_path)
            if type_path.to_token_stream().to_string().replace(' ', "") == "Vec<Vector3>" =>
        {
            "array<vector>".to_string()
        }
        Type::Path(type_path)
            if type_path.to_token_stream().to_string().replace(' ', "") == "Vec<bool>" =>
        {
            "array<bool>".to_string()
        }
        Type::Path(type_path)
            if type_path.to_token_stream().to_string().replace(' ', "") == "Vec<i32>" =>
        {
            "array<int>".to_string()
        }
        Type::Path(type_path)
            if type_path.to_token_stream().to_string().replace(' ', "") == "Vec<f32>" =>
        {
            "array<float>".to_string()
        }
        Type::Path(type_path)
            if type_path.to_token_stream().to_string().replace(' ', "") == "CBasePlayer" =>
        {
            "entity".to_string()
        }
        Type::BareFn(fun) => {
            let head_types = format!("{} functionref( ", get_sqoutput(&fun.output));
            let mut func_args = String::new();

            for arg in fun.inputs.iter() {
                push_type!(func_args, &get_sqtype(&Box::new(arg.ty.clone()))[..], "");
            }

            format!("{head_types}{func_args})")
        }
        _ => "var".to_string(),
    }
}
