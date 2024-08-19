use proc_macro::TokenStream;
use quote::{quote, spanned::Spanned, ToTokens};
use syn::{
    parse::Parse, parse::ParseStream, punctuated::Punctuated, token::Comma, FnArg, Ident, LitStr,
    Result as SynResult, Token, Type, __private::TokenStream2, parse_quote, parse_str,
    Error as SynError,
};

pub struct Arg {
    pub ident: Ident,
    pub arg: LitStr,
}

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

pub fn input_mapping(
    args: &Punctuated<FnArg, Comma>,
    sq_stack_pos: &mut i32,
) -> Result<Vec<TokenStream>, SynError> {
    let mut token_streams: Vec<TokenStream> = Vec::new();

    for arg in args.iter() {
        // let (ty,name,tk) = match_input(arg, *sq_stack_pos)?;

        let name = match arg {
            FnArg::Receiver(_) => TokenStream2::new(),
            FnArg::Typed(t) => t.clone().pat.into_token_stream().into(),
        };
        let ty = get_arg_type(arg)?;

        let tk = quote! {
                #[allow(unused_mut)]
                let #name: #ty = GetFromSquirrelVm::get_from_sqvm_internal(sqvm, sq_functions, &mut current_stack_pos);
        }.into();

        token_streams.push(tk);

        *sq_stack_pos += 1;
    }

    Ok(token_streams)
}

pub fn filter_args(input: &FnArg) -> Option<FnArg> {
    let mut input = input.clone();
    match &mut input {
        FnArg::Receiver(_) => None?,
        FnArg::Typed(t) => t.ty = maybe_change(&t.ty),
    };

    Some(input)
}

pub fn get_arg_type(input: &FnArg) -> Result<Box<Type>, SynError> {
    match input {
        FnArg::Receiver(_) => Err(SynError::new(
            input.__span(),
            format!("invalid arg {}", input.to_token_stream().to_string()),
        )),
        FnArg::Typed(t) => Ok(maybe_change(&t.ty)),
    }
}

pub fn get_arg_ident(input: &FnArg) -> Option<Ident> {
    match input {
        FnArg::Receiver(_) => None,
        FnArg::Typed(t) => parse_str(t.pat.to_token_stream().to_string().as_str()).ok(),
    }
}

fn maybe_change(ty: &Type) -> Box<Type> {
    match *ty {
        Type::BareFn(_) => Box::new(parse_quote!(
            rrplug::high::squirrel::SQHandle<rrplug::bindings::squirreldatatypes::SQClosure>
        )),
        _ => Box::new(ty.clone()),
    }
}
