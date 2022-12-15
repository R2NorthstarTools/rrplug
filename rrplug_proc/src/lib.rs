// REMINDER: THIS CRATE MIGHT NEEDS TO BE A SEPARETE CRATE ON CRATES.io

extern crate proc_macro;

use proc_macro::{TokenStream, Span};
use quote::{quote, ToTokens};
use syn;
use syn::{parse_macro_input, FnArg, ItemFn, Stmt, Type, Ident};

#[proc_macro_attribute]
pub fn sqfunction(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());

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
    let cpp_func_name = format!( "sq_{}", ident.to_string() );
    let sq_func_name = ident.to_string();
    let sq_ident = Ident::new(&cpp_func_name.clone()[..], Span::call_site().into());

    println!( "last stmt: {}", stmts.last().to_token_stream().to_string() );

    for i in input.iter() {
        match i.to_owned() {
            FnArg::Receiver(_) => {
                panic!("wtf are you doing stop this now we don't support methods")
            }
            FnArg::Typed(t) => match &*t.ty {
                Type::Path(type_path)
                    if type_path.to_token_stream().to_string() == "bool" =>
                {
                    let name = t.clone().pat.to_token_stream();
                    if !sqtypes.is_empty() {
                        sqtypes.push(',');
                    }
                    sqtypes.push_str("bool");
                    sqtypes.push(' ');
                    sqtypes.push_str(&name.to_string()[..]);
                    let tk = quote! {let #name = 0;}.into();
                    let new_stmt = parse_macro_input!( tk as Stmt);
                    stmts.insert(0, new_stmt);
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
                _ => panic!("type isn't supported"),
            },
        }
    }

    let func_info = quote!(
        const fn #ident () -> (&'static str, &'static str, &'static str, rrplug::bindings::squirrelclasstypes::SQFunction) {
            (#cpp_func_name, #sq_func_name, #sqtypes, #sq_ident ) // todo add name for sq since sq_ is confusing
        }
    );

    let out: TokenStream = quote! {
        #func_info
        
        unsafe extern "C" fn #sq_ident (sqvm: *mut rrplug::bindings::squirreldatatypes::HSquirrelVM) -> rrplug::bindings::squirrelclasstypes::SQRESULT {
            #(#stmts)*
        }
    }.into();

    println!("out: \"{}\"", out.to_string());
    out
}