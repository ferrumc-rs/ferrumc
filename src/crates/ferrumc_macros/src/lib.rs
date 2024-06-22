extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Decode)]
pub fn decode_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used to store all field decoding statements
    let mut field_statements = Vec::new();

    // Check if our struct has named fields
    if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) = input.data {
        for field in fields.named {
            // Get the identifier of the field
            let ident = field.ident.unwrap();
            // Generate a statement to decode this field from the bytes
            let statement = quote! {
                println!("Decoding field: {}", stringify!(#ident));
            };
            field_statements.push(statement);
        }
    }

    // Get the identifier of our struct
    let name = input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl #name {
            pub fn decode(bytes: &mut impl Into<Vec<u8>>) -> Self {
                #(#field_statements)*
                // Self {
                //     #(#field_statements)*
                // }
                Self::default()
                
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}