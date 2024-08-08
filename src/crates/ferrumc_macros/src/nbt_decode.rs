use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn decode(input: TokenStream) -> TokenStream {
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
            let statement = if field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("nbtcompound"))
            {
                quote! {
                    #ident: match <#type_name as crate::utils::nbt_impls::NBTDecodable>::decode_from_compound(nbt, stringify!(#ident)) {
                        Ok(value) => value,
                        Err(e) => return Err(Error::Generic(format!("Failed to decode field {}: {}", stringify!(#ident), e)))
                    },
                }
            } else {
                quote! {
                    #ident: match <#type_name as crate::utils::nbt_impls::NBTDecodable>::decode_from_base(nbt, stringify!(#ident)) {
                        Ok(value) => value,
                        Err(e) => return Err(Error::Generic(format!("Failed to decode field {}: {}", stringify!(#ident), e)))
                    },
                }
            };

            field_statements.push(statement);
        }
    }

    // Get the identifier of our struct
    let name = input.ident;

    // Generate the implementation
    let expanded = quote! {
        use crate::utils::type_impls::Decode;
        use tokio::io::{AsyncRead, AsyncSeek};
        use crate::utils::error::Error;
        impl #name {
            pub fn decode(bytes: Vec<u8>) -> core::result::Result<Self, Error>
            {
                    let nbt = simdnbt::borrow::read(&mut std::io::Cursor::new(bytes.as_slice()))
        .unwrap()
        .unwrap();
                Ok(Self {
                    #(#field_statements)*
                })

            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
