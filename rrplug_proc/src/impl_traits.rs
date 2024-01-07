use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, Variant};

pub fn impl_struct_or_enum(
    input: DeriveInput,
    struct_func: fn(DeriveInput) -> TokenStream,
    enum_func: fn(DeriveInput) -> TokenStream,
) -> TokenStream {
    match input.data {
        Data::Struct(_) => struct_func(input),
        Data::Enum(_) => enum_func(input),
        Data::Union(_) => panic!("Unions are not support by this macro"),
    }
}

pub fn get_from_sqvm_impl_struct(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = input;

    let fields = get_struct_fields(data);
    let field_idents: Vec<Ident> = fields.iter().cloned().filter_map(|f| f.ident).collect();
    let field_amount = field_idents.len() as u32;

    quote!(
        impl<#generics> GetFromSquirrelVm for #ident<#generics> {
            #[allow(clippy::not_unsafe_ptr_arg_deref)] // smth should be done about this
            #[inline]
            fn get_from_sqvm(
                sqvm: *mut HSquirrelVM,
                sqfunctions: &SquirrelFunctionsUnwraped,
                stack_pos: i32,
            ) -> Self {
                use rrplug::{high::squirrel_traits::GetFromSQObject,bindings::squirreldatatypes::SQObject};
                let sqstruct = unsafe {
                    let sqvm = sqvm.as_ref().expect("sqvm has to be valid");
                    ((*sqvm._stackOfCurrentFunction.add(stack_pos as usize))
                        ._VAL
                        .asStructInstance)
                        .as_ref()
                        .expect("provided struct was invalid")
                };

                debug_assert_eq!(#field_amount, sqstruct.size, "the size of the struct instance({}) didn't match the size of {}({})", sqstruct.size, stringify!(#ident), #field_amount);

                let data = &sqstruct.data as *const SQObject; // this static array is dynamic in reality
                let mut iter = (0..sqstruct.size)
                    .filter_map(|i| i.try_into().ok())
                    .filter_map(|i| unsafe { data.add(i).as_ref() });

                Self {
                    #(
                        #field_idents: GetFromSQObject::get_from_sqobject(iter.next().expect("ran out of struct instance fields")),
                    )*
                }
            }
        }
    )
    .into()
}

pub fn push_to_sqvm_impl_struct(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = input;

    let fields = get_struct_fields(data);
    let field_idents: Vec<Ident> = fields.iter().cloned().filter_map(|f| f.ident).collect();
    let field_amount = field_idents.len() as i32;
    let field_amount_iter = 0..field_amount;

    quote!(
        impl<#generics> PushToSquirrelVm for #ident<#generics> {
            #[allow(clippy::not_unsafe_ptr_arg_deref)]
            #[inline]
            fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
                unsafe {
                    (sqfunctions.sq_pushnewstructinstance)(sqvm, #field_amount);
                    #(
                        self.#field_idents.push_to_sqvm(sqvm,sqfunctions);
                        (sqfunctions.sq_sealstructslot)(sqvm, #field_amount_iter);
                    )*
                }
            }
        }
    )
    .into()
}

fn get_struct_fields(data: Data) -> Fields {
    match data {
        Data::Struct(data) => data.fields,
        Data::Enum(_) => panic!("Enums are not support by this macro"),
        Data::Union(_) => panic!("Unions are not support by this macro"),
    }
}

pub fn get_from_sqvm_impl_enum(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = input;

    let varients = get_enum_varients(data);
    let varient_first = varients
        .first()
        .expect("this enum is empty, this wouldn't work");
    let varient_last = varients
        .last()
        .expect("this enum is empty, this wouldn't work");

    quote!(
        impl<#generics> GetFromSquirrelVm for #ident<#generics> {
            #[inline]
            #[allow(clippy::not_unsafe_ptr_arg_deref)]
            fn get_from_sqvm(
                sqvm: *mut HSquirrelVM,
                sqfunctions: &SquirrelFunctionsUnwraped,
                stack_pos: i32,
            ) -> Self {
                const _ :#ident<#generics> = unsafe { std::mem::transmute::<i32,#ident<#generics>>(0) };

                let value = unsafe { rrplug::mid::squirrel::get_sq_int(sqvm, sqfunctions, stack_pos) };

                if value >= #ident::#varient_first as i32 && value <= #ident::#varient_last as i32 {
                    unsafe { std::mem::transmute(value) }
                } else {
                    panic!("undefined enum varient; check if the enum defenition matches the squirrel one");
                }
            }
        }
    )
    .into()
}

pub fn push_to_sqvm_impl_enum(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data: _,
    } = input;

    quote!(
        impl<#generics> PushToSquirrelVm for #ident<#generics> {
            #[inline]
            #[allow(clippy::not_unsafe_ptr_arg_deref)]
            fn push_to_sqvm(self, sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped) {
                unsafe { rrplug::mid::squirrel::push_sq_int(sqvm, sqfunctions, self as i32) };
            }
        }
    )
    .into()
}

fn get_enum_varients(data: Data) -> Vec<Variant> {
    match data {
        Data::Struct(_) => panic!("Structs are not support by this macro"),
        Data::Enum(data) => data.variants.into_iter().collect(),
        Data::Union(_) => panic!("Unions are not support by this macro"),
    }
}

pub fn get_from_sqobject_impl_enum(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = input;

    let varients = get_enum_varients(data);
    let varient_first = varients
        .first()
        .expect("this enum is empty, this wouldn't work");
    let varient_last = varients
        .last()
        .expect("this enum is empty, this wouldn't work");

    quote!(
        impl<#generics> GetFromSQObject for #ident<#generics> {
            #[inline]
            #[allow(clippy::not_unsafe_ptr_arg_deref)]
            fn get_from_sqobject(obj: &rrplug::bindings::squirreldatatypes::SQObject) -> Self {
                const _: #ident<#generics> = unsafe { std::mem::transmute::<i32,#ident<#generics>>(0) };

                let value = unsafe { obj._VAL.asInteger };

                if value >= #ident::#varient_first as i32 && value <= #ident::#varient_last as i32 {
                    unsafe { std::mem::transmute(value) }
                } else {
                    panic!("undefined enum varient; check if the enum defenition matches the squirrel one");
                }
            }
        }
    )
    .into()
}

// TODO: refactor this to use what I have in the other implemantion of this
// whar, past self?
pub fn get_from_sqobject_impl_struct(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        // copying this from system clipboard destroyed this, but I won't fix this today
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = input;
    let fields = get_struct_fields(data);
    let field_idents: Vec<Ident> = fields.iter().cloned().filter_map(|f| f.ident).collect();
    let field_amount = field_idents.len() as u32;
    quote!(
         impl<#generics> GetFromSQObject for #ident<#generics> {
             #[allow(clippy::not_unsafe_ptr_arg_deref)] // smth should be done about this
             #[inline]
             fn get_from_sqobject(obj: &rrplug::bindings::squirreldatatypes::SQObject) -> Self {
                 use rrplug::{high::squirrel_traits::getfromsqobject,bindings::squirreldatatypes::sqobject};
                 let sqstruct = unsafe {
                     obj
                         ._VAL
                         .asStructInstance
                         .as_ref()
                         .expect("provided struct was invalid")
                 };
                 debug_assert_eq!(#field_amount, sqstruct.size, "the size of the struct instance({}) didn't match the size of {}({})", sqstruct.size, stringify!(#ident), #field_amount);
                 let data = &sqstruct.data as *const SQObject; // this static array is dynamic in reality
                 let mut iter = (0..sqstruct.size)
                     .filter_map(|i| i.try_into().ok())
                     .filter_map(|i| unsafe { data.add(i).as_ref() });
                 Self {
                      #(
                          #field_idents: GetFromSQObject::get_from_sqobject(iter.next().expect("ran out of struct instance fields")),
                      )*
                     }
                 }
             }
        )
    .into()
}

pub fn sqvm_name_impl(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = input;

    let mut sqname = ident.to_string();
    if let Data::Enum(_) = data {
        sqname = "int".to_string();
    }

    quote!(
        impl<#generics> SQVMName for #ident<#generics> {
            fn get_sqvm_name() -> String {
                #sqname.to_string()
            }
        }
    )
    .into()
}
