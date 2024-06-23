extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Decode)]
pub fn decode_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used to store all field decoding statements
    let mut field_statements = Vec::new();

    // Check if our struct has named fields
    if let syn::Data::Struct(syn::DataStruct {
                                 fields: syn::Fields::Named(fields),
                                 ..
                             }) = input.data
    {
        for field in fields.named {
            // Get the identifier of the field
            let ident = field.ident.unwrap();
            // Generate a statement to decode this field from the bytes
            let type_name = field.ty;
            let statement = quote! {
                #ident: match <#type_name as Decode>::decode(bytes).await {
                    Ok(value) => Box::into_inner(value),
                    Err(e) => return Err(Error::Generic(format!("Failed to decode field {}: {}", stringify!(#ident), e)))
                },
            };
            field_statements.push(statement);
        }
    }

    // Get the identifier of our struct
    let name = input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl #name {
            pub async fn decode<T>(bytes: &mut T) -> core::result::Result<Self, Error>
            where
                T: AsyncRead + AsyncSeek + Unpin,
            {
                Ok(Self {
                    #(#field_statements)*
                })

            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[proc_macro_derive(Encode)]
pub fn encode_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used to store all field encoding statements
    let mut field_statements = Vec::new();

    // Check if our struct has named fields
    if let syn::Data::Struct(syn::DataStruct {
                                 fields: syn::Fields::Named(fields),
                                 ..
                             }) = input.data
    {
        for field in fields.named {
            // Get the identifier of the field
            let ident = field.ident.unwrap();
            // Generate a statement to encode this field to the bytes
            let type_name = field.ty;
            let statement = quote! {
                <#type_name as Encode>::encode(&self.#ident, bytes).await?;
            };
            field_statements.push(statement);
        }
    }

    // Get the identifier of our struct
    let name = input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl #name {
            pub async fn encode<T>(&self, bytes: &mut T) -> core::result::Result<(), Error>
            where
                T: AsyncWrite + AsyncSeek + Unpin,
            {
                #(#field_statements)*
                Ok(())
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}