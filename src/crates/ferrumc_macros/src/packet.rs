use std::env;
use std::ops::Add;
use std::path::Path;

use quote::quote;
use syn::{parse_macro_input, LitInt, LitStr};

use proc_macro::TokenStream;

pub fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    // check if the packet attribute has the packet_id and state fields
    // if not, compile_error

    if args.is_empty() {
        return TokenStream::from(quote! {
            compile_error!("packet attribute must have the packet_id and state fields");
        });
    }

    if !vec!["packet_id", "state"]
        .iter()
        .all(|x| args.to_string().contains(x))
    {
        return TokenStream::from(quote! {
            compile_error!("packet attribute must have the packet_id and state fields");
        });
    }

    TokenStream::from(input)
}

pub fn bake(input: TokenStream) -> TokenStream {
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
        let file_name = path.file_name().expect("file_name failed").to_os_string();

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
                if !attr.path().is_ident("packet") {
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
                        }
                        "state" => {
                            let value = meta.value().expect("value failed");
                            let value = value.parse::<LitStr>().expect("parse failed");
                            let n = value.value();
                            state = Some(n);
                        }
                        &_ => {
                            return Ok(());
                        }
                    }

                    Ok(())
                })
                    .unwrap();
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

            println!(
                "[FERRUMC_MACROS] Found Packet (ID: 0x{:02X}, State: {}, Struct Name: {})",
                packet_id, state, struct_name
            );

            let path = format!(
                "crate::net::packets::incoming::{}",
                file_name.to_string_lossy().replace(".rs", "")
            );

            let struct_path = format!("{}::{}", path, struct_name);

            let struct_path = syn::parse_str::<syn::Path>(&struct_path).expect("parse_str failed");

            match_arms.push(quote! {
                (#packet_id, #state) => {
                    let packet= #struct_path::net_decode(cursor).await?;
                    packet.handle(conn_id, state).await?;
                },
            });

            /*match_arms.push(quote! {
                (#packet_id, #state) => {
                    let packet= #path::#struct_name::decode(cursor).await?;
                    packet.handle(conn_owned).await?;
                },
            });*/
        }
    }

    let elapsed = start.elapsed();
    println!("[FERRUMC_MACROS] Found {} packets", match_arms.len());
    println!(
        "[FERRUMC_MACROS] It took: {:?} to parse all the files and generate the packet registry",
        elapsed
    );

    let match_arms = match_arms.into_iter();

    let output = quote! {
        pub async fn handle_packet(packet_id: u8, conn_id: usize, conn_state: &crate::net::State, cursor: &mut std::io::Cursor<Vec<u8>>, state: crate::state::GlobalState) -> crate::utils::prelude::Result<()> {
            match (packet_id, conn_state.as_str()) {
                #(#match_arms)*
                _ => tracing::warn!("No packet found for ID: 0x{:02X} in state: {}", packet_id, conn_state.as_str()),
            }

            Ok(())
        }
    };

    TokenStream::from(output)
}
