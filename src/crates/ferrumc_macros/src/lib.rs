extern crate proc_macro;

use proc_macro::TokenStream;
use std::env;
use std::ops::Add;
use std::path::Path;

use quote::quote;
use syn::{DeriveInput, LitInt, LitStr, parse_macro_input};

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
        use tokio::io::{AsyncRead, AsyncSeek};
        use ferrumc_utils::error::Error;
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
        use ferrumc_utils::error::Error;
        use ferrumc_utils::encoding::varint::VarInt;
        use ferrumc_utils::type_impls::Encode;
        use tokio::io::AsyncWriteExt;
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

/// Just exists, so we can use it in the packet attribute
#[proc_macro_attribute]
pub fn packet(args: TokenStream, input: TokenStream) -> TokenStream {

    // check if the packet attribute has the packet_id and state fields
    // if not, compile_error

    if args.is_empty() {
        return TokenStream::from(quote! {
            compile_error!("packet attribute must have the packet_id and state fields");
        });
    }

    if !vec!["packet_id", "state"].iter().all(|x| args.to_string().contains(x)) {
        return TokenStream::from(quote! {
            compile_error!("packet attribute must have the packet_id and state fields");
        });
    }

    TokenStream::from(input)
}

/// This macro generates a packet registry for the server.
#[proc_macro]
pub fn bake_packet_registry(input: TokenStream) -> TokenStream {
    // read all the files in /src/packets/incoming
    // for each file, read the packet_id attribute

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let module_path = parse_macro_input!(input as syn::LitStr).value();

    let mut path = manifest_dir.add(module_path.as_str());
    path = path.replace("\\", "/");

    println!("[FERRUMC_MACROS] Parsing packets in: {}", path);

    let dir_path = Path::new(&path);

    if !std::fs::metadata(&dir_path).unwrap().is_dir() {
        return TokenStream::from(quote! {
            compile_error!("Provided path is not a directory");
        });
    }

    let mut match_arms = Vec::new();

    let start = std::time::Instant::now();

    for entry in std::fs::read_dir(dir_path).expect("read_dir call failed") {
        let entry = entry.expect("entry failed");
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let content = std::fs::read_to_string(path).expect("read_to_string call failed");
        let syntax = syn::parse_file(&content).expect("parse_file call failed");

        for item in syntax.items {
            let syn::Item::Struct(item_struct) = item else {
                continue;
            };

            // format: #[packet(packet_id = 0x00, state = "handshake")]

            let mut packet_id = None;
            let mut state = None;

            for attr in item_struct.attrs {
                if !(attr.path().is_ident("packet")) {
                    continue;
                }

                attr.parse_nested_meta(|meta| {
                    let Some(ident) = meta.path.get_ident() else {
                        return Ok(());
                    };

                    match ident.to_string().as_str() {
                        "packet_id" => {
                            let value = meta.value().expect("value failed");
                            let value = value.parse::<LitInt>().expect("parse failed");
                            let n: usize = value.base10_parse().expect("base10_parse failed");
                            packet_id = Some(n);

                        },
                        "state" => {
                            let value = meta.value().expect("value failed");
                            let value = value.parse::<LitStr>().expect("parse failed");
                            let n = value.value();
                            state = Some(n);

                        },
                        &_ => {
                            return Ok(());
                        }
                    }

                    Ok(())
                }).unwrap();
            }


            let packet_id = match packet_id {
                Some(id) => id,
                None => continue,
            };
            let packet_id = packet_id as u8;

            let state = match state {
                Some(state) => state,
                None => continue,
            };

            let struct_name = &item_struct.ident;

            println!("[FERRUMC_MACROS] Found Packet (ID: 0x{:02X}, State: {}, Struct Name: {})", packet_id, state, struct_name);

            let struct_name_lowercase = struct_name.clone().to_string().to_lowercase();
            let struct_name_lowercase = syn::Ident::new(&struct_name_lowercase, struct_name.span());

            match_arms.push(quote! {
                (#packet_id, #state) => {
                    let packet = crate::packets::incoming::#struct_name_lowercase::#struct_name::decode(cursor).await?;
                    packet.handle(conn_owned).await?;
                },
            });
        }
    }

    let elapsed = start.elapsed();
    println!("[FERRUMC_MACROS] Found {} packets", match_arms.len());
    println!("[FERRUMC_MACROS] It took: {:?} to parse all the files and generate the packet registry", elapsed);

    let match_arms = match_arms.into_iter();

    let output = quote! {
        pub async fn handle_packet(packet_id: u8, conn_owned: &mut crate::Connection, cursor: &mut std::io::Cursor<Vec<u8>>) -> ferrumc_utils::prelude::Result<()> {
            match (packet_id, conn_owned.state.as_str()) {
                #(#match_arms)*
                _ => println!("No packet found for ID: 0x{:02X} in state: {}", packet_id, conn_owned.state.as_str()),
            }

            Ok(())
        }
    };

    TokenStream::from(output)
}