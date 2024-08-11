use proc_macro::TokenStream;
use quote::{format_ident, quote};

use syn::DeriveInput;

pub fn derive_name(input: TokenStream) -> TokenStream {
    // auto generate a #name() method for the struct
    let input = syn::parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let name_str = name.to_string();

    let expanded = quote! {
        impl #name {
            pub fn type_name() -> &'static str {
                #name_str
            }
        }
    };

    TokenStream::from(expanded)
}

pub(crate) fn derive_getter(input: TokenStream) -> TokenStream {
    // auto generate a get_#field() method for the struct

    let input = syn::parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let fields = match input.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => fields.named.iter().map(|f| (f.ident.clone().unwrap(), f.ty.clone())),
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let expanded = fields.map(|field| {
        let (field_name, field_type) = field;

        let getter_name = format_ident!("get_{}", field_name);

        quote! {
            impl #name {
                pub fn #getter_name(&self) -> &#field_type {
                    &self.#field_name
                }
            }
        }

    });

    TokenStream::from(quote! {
        #(#expanded)*
    })
}