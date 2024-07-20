use proc_macro::TokenStream;
use quote::quote;

use syn::{DeriveInput, Fields, parse_macro_input};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("New can only be derived for structs"),
    };

    let (field_names, field_types): (Vec<_>, Vec<_>) = match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| (&f.ident, &f.ty))
            .unzip(),
        _ => panic!("New can only be derived for structs with named fields"),
    };

    let expanded = quote! {
        impl #name {
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(#field_names),*
                }
            }
        }
    };

    expanded.into()
}