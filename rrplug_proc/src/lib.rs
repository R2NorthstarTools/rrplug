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
            let new_stmt = parse_macro_input!( $tk as Stmt);
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
    let cpp_func_name = format!( "sq_{}", ident.to_string() );
    let sq_func_name = ident.to_string();
    let sq_ident = Ident::new(&cpp_func_name.clone()[..], Span::call_site().into());

    let mut sq_stack_pos = 1;
    let mut sq_gets_stmts = Vec::new();

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
                    push_type!(sqtypes, "bool", &name.to_string()[..]);
                    let tk = quote! {let #name = (sq_functions.sq_getbool)(sqvm, #sq_stack_pos) == 1;}.into();
                    push_stmts!( sq_gets_stmts, tk );

                    sq_stack_pos += 1;
                },
                Type::Path(type_path)
                    if type_path.to_token_stream().to_string() == "i32" =>
                {
                    let name = t.clone().pat.to_token_stream();
                    push_type!(sqtypes, "int", &name.to_string()[..]);
                    let tk = quote! {let #name = (sq_functions.sq_getinteger)(sqvm, #sq_stack_pos) as i32;}.into();
                    push_stmts!( sq_gets_stmts, tk );

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
                _ => panic!("type isn't supported"),
            },
        }
    }
    
    sq_gets_stmts.reverse();
    for s in sq_gets_stmts {
        stmts.insert( 0, s);
    }

    let tk = quote! {let sq_functions = SQFUNCTIONS.client.as_ref().unwrap();}.into();
    push_stmts!( stmts, tk );

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