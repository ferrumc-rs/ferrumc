use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn decode(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used to store all field decoding statements
    let mut statements = Vec::new();

    let is_nbtcompound = input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("nbtcompound"));

    // Check if our struct has named fields
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(fields),
        ..
    }) = input.data
    {
        for field in fields.named {
            // Get the identifier of the field
            let ident = field.clone().ident.unwrap();
            // Generate a statement to decode this field from the bytes
            let type_name = field.clone().ty;
            let is_field_nbtcompound = field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("nbtcompound"));
            if is_field_nbtcompound {
                statements.push(quote! {
                #ident: #type_name::decode_from_compound(&nbt.compound(stringify!(#ident)).unwrap())?,
                });
            } else if is_nbtcompound {
                statements.push(
                    quote! {
                    #ident: <#type_name as crate::utils::nbt_impls::NBTDecodable>::decode_from_compound(nbt, stringify!(#ident))?,
                    },
                );
            } else {
                statements.push(
                quote! {
                    #ident: <#type_name as crate::utils::nbt_impls::NBTDecodable>::decode_from_base(&nbt, stringify!(#ident))?,
                    },
            );
            };
        }
    }

    // Get the identifier of our struct
    let name = input.ident;

    // Generate the implementation
    let expanded = if is_nbtcompound {
        quote! {
            impl #name {

                pub fn decode_from_compound(nbt: &simdnbt::borrow::NbtCompound) -> core::result::Result<Self, crate::utils::error::Error>
                {
                    Ok(Self {
                        #(#statements)*
                    })
                }
            }
        }
    } else {
        quote! {
        impl #name {
            pub fn decode(bytes: Vec<u8>) -> core::result::Result<Self, crate::utils::error::Error>
            {
                let nbt = simdnbt::borrow::read(&mut std::io::Cursor::new(bytes.as_slice())).unwrap().unwrap();
                Ok(Self {
                    #(#statements)*
                })

            }
        } }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
