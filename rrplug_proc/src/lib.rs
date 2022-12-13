// REMINDER: THIS CRATE MIGHT NEEDS TO BE A SEPARETE CRATE ON CRATES.io

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn;
use syn::{parse_macro_input, FnArg, ItemFn, Stmt, Type};

#[proc_macro_attribute]
pub fn sqfunction(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());

    let input = parse_macro_input!(item as ItemFn);

    let ItemFn {
        attrs,
        vis: _,
        sig,
        block,
    } = input;
    let mut stmts = block.stmts;
    let ident = &sig.ident;
    let input = &sig.inputs;

    for i in input.iter() {
        match i.to_owned() {
            FnArg::Receiver(_) => {
                panic!("wtf are you doing stop this now we don't support methods")
            }
            FnArg::Typed(t) => match &*t.ty {
                Type::Path(type_path)
                    if type_path.clone().into_token_stream().to_string() == "bool" =>
                {
                    let new_stmt: TokenStream = quote! {
                        let test = 0;
                    }
                    .into();
                    let new_stmt = parse_macro_input!(new_stmt as Stmt);
                    stmts.insert(0, new_stmt);
                    // Ident::new(&format!("let test = 0;"), Span::call_site())
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

    let out: TokenStream = quote! {
        #(#attrs)* unsafe extern "C" fn #ident (sqvm: *mut rrplug::bindings::squirreldatatypes::HSquirrelVM) -> rrplug::bindings::squirrelclasstypes::SQRESULT {
            #(#stmts)*
        }
    }.into();

    println!("out: \"{}\"", out.to_string());
    out
}
