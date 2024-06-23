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
        use ferrumc_utils::type_impls::Decode;
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
    let mut has_packet_id = false;

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

            if ident=="packet_id" {
                has_packet_id = true;
            }

            let type_name = field.ty;
            let statement = quote! {
                <#type_name as Encode>::encode(&self.#ident, &mut bytes).await?;
            };
            field_statements.push(statement);
        }
    }

    if !has_packet_id {
        return TokenStream::from(quote! {
            compile_error!("Struct must have a packet_id field");
        });
    }

    // Get the identifier of our struct
    let name = input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl #name {
            pub async fn encode(&self) -> core::result::Result<Vec<u8>, Error>
            {
                let mut bytes = std::io::Cursor::new(Vec::new());

                #(#field_statements)*

                let __packet_data = bytes.into_inner();
                let __length = __packet_data.len() as i32;
                let __length: ferrumc_utils::encoding::varint::VarInt = ferrumc_utils::encoding::varint::VarInt::new(__length);

                let mut __cursor = std::io::Cursor::new(Vec::new());
                __length.encode(&mut __cursor).await?;

                __cursor.write_all(&__packet_data).await?;

                Ok(__cursor.into_inner())
            }
        }
    };


    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}