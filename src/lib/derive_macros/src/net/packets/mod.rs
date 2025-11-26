use colored::Colorize;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::env;
use std::ops::Add;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Expr, LitStr, Token,
};

// --- 1. Custom Parser for #[packet(...)] ---

struct PacketArgs {
    id: Expr,      // Accepts constants like `ids::KEEP_ALIVE`
    state: String, // Accepts "play", "login"
}

impl Parse for PacketArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut id = None;
        let mut state = None;

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            if key == "id" {
                id = Some(input.parse()?);
            } else if key == "state" {
                let s: LitStr = input.parse()?;
                state = Some(s.value());
            } else if key == "packet_id" {
                // Gracefully handle old syntax if you want, or error
                // For now, let's just eat the literal string and panic/error
                let _s: LitStr = input.parse()?;
                return Err(input.error("The 'packet_id' string argument is deprecated. Please use 'id = CONSTANT' (e.g. ids::PLAY_...)"));
            } else {
                return Err(input.error(format!(
                    "Unknown argument '{}'. Supported: 'id', 'state'",
                    key
                )));
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        let id = id.ok_or_else(|| input.error("Missing 'id' argument"))?;
        let state = state.ok_or_else(|| input.error("Missing 'state' argument"))?;

        Ok(PacketArgs { id, state })
    }
}

// --- 2. Attribute Macro: #[packet] ---

pub fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    // We parse the arguments to validate them, but we don't strictly need to use them
    // here unless we are implementing a trait (like `impl Packet for Struct`).
    // For now, we just pass the struct through unchanged, as the real work is done
    // by the router generator scanning the files.

    let _ = parse_macro_input!(args as PacketArgs);
    let item = parse_macro_input!(input as DeriveInput);

    // You could add `impl Packet` here if you wanted to attach metadata to the struct itself.
    // For now, we just return the struct.
    TokenStream::from(quote! { #item })
}

// --- 3. Router Generator: setup_packet_handling ---

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i != 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

pub fn setup_packet_handling(input: TokenStream) -> TokenStream {
    #[cfg(feature = "colors")]
    colored::control::set_override(true);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let module_path = parse_macro_input!(input as syn::LitStr).value();

    let mut path = manifest_dir.add(module_path.as_str());
    path = path.replace("\\", "/");

    let dir_path = std::path::Path::new(&path);
    // Calculate crate import path: "src/bin/src/..." -> "crate::..."
    // This logic assumes a specific folder structure.
    let base_path = module_path.split("\\").collect::<Vec<&str>>()[2..].join("::");
    let base_path = format!("crate::{base_path}");

    println!(
        "   {} {}",
        "[FERRUMC_MACROS]".bold().blue(),
        format!("Parsing packets in {}", dir_path.display())
            .white()
            .bold()
    );

    if !dir_path.is_dir() {
        return TokenStream::from(quote! {
            compile_error!("The path provided is not a directory");
        });
    }

    let mut match_arms = Vec::new();
    let start = std::time::Instant::now();

    // Lists for generating the mega-struct
    let mut sender_mega_struct_fields = vec![];
    let mut send_recv_pairs = vec![];
    let mut build_mega_struct = vec![];
    let mut register_structs = vec![];
    let mut receiver_structs = vec![];

    let entries = std::fs::read_dir(dir_path).expect("read_dir call failed");

    for entry in entries {
        let entry = entry.expect("entry failed");
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = path.file_name().unwrap().to_string_lossy();
        if !file_name.ends_with(".rs") {
            continue;
        }

        let content = std::fs::read_to_string(&path).expect("read_to_string failed");
        let syntax = syn::parse_file(&content).expect("parse_file failed");

        for item in syntax.items {
            if let syn::Item::Struct(item_struct) = item {
                // Check for #[packet]
                let mut packet_args = None;

                for attr in &item_struct.attrs {
                    if attr.path().is_ident("packet") {
                        // Use our new Syn parser!
                        if let Ok(args) = attr.parse_args_with(PacketArgs::parse) {
                            packet_args = Some(args);
                        }
                    }
                }

                if let Some(args) = packet_args {
                    // We only generate routers for incoming (Serverbound) "Play" packets usually.
                    // The old code filtered by `state == "play"`.
                    if args.state == "play" {
                        let struct_name = item_struct.ident;
                        let packet_id_expr = args.id; // This is the CONSTANT expression

                        // Debug output
                        println!(
                            "   {} {} (State: {}, Struct: {})",
                            "[FERRUMC_MACROS]".bold().blue(),
                            "Found Packet".white().bold(),
                            args.state.green(),
                            struct_name.to_string().yellow()
                        );

                        // Construct paths
                        let mod_name = file_name.replace(".rs", "");
                        let struct_path_str =
                            format!("{}::{}::{}", base_path, mod_name, struct_name);
                        let struct_path: syn::Path = syn::parse_str(&struct_path_str).unwrap();

                        let snake_name = to_snake_case(&struct_name.to_string());
                        let field_name = format_ident!("{}", snake_name);
                        let receiver_type = format_ident!("{}Receiver", struct_name);
                        let sender_name = format_ident!("{}_sender", snake_name);
                        let receiver_name = format_ident!("{}_receiver", snake_name);

                        // 1. Generate Match Arm
                        // Note: We match on `id` (expression), not a literal string.
                        match_arms.push(quote! {
                            (#packet_id_expr) => {
                                // Decode
                                let packet = <#struct_path as ferrumc_net_codec::decode::NetDecode>::decode(cursor, &ferrumc_net_codec::decode::NetDecodeOpts::None)?;
                                // Send
                                packet_sender.#field_name.send((packet, entity)).expect("Failed to send packet");
                                Ok(())
                            },
                        });

                        // 2. Struct Field
                        sender_mega_struct_fields.push(quote! {
                            pub #field_name: crossbeam_channel::Sender<(#struct_path, bevy_ecs::entity::Entity)>,
                        });

                        // 3. Channel Setup
                        send_recv_pairs.push(quote! {
                            let (#sender_name, #receiver_name) = crossbeam_channel::unbounded();
                        });

                        // 4. Struct Build
                        build_mega_struct.push(quote! {
                            #field_name: #sender_name,
                        });

                        // 5. Resource Registration
                        register_structs.push(quote! {
                            world.insert_resource(#receiver_type(#receiver_name));
                        });

                        // 6. Resource Definition
                        receiver_structs.push(quote! {
                            #[derive(bevy_ecs::prelude::Resource)]
                            pub struct #receiver_type(pub crossbeam_channel::Receiver<(#struct_path, bevy_ecs::entity::Entity)>);
                        });
                    }
                }
            }
        }
    }

    let elapsed = start.elapsed();
    println!(
        "   {} Parsed {} packets in {:?}",
        "[FERRUMC_MACROS]".bold().blue(),
        match_arms.len(),
        elapsed
    );

    let output = quote! {
        pub fn handle_packet<R: std::io::Read>(
            packet_id: u8,
            entity: bevy_ecs::entity::Entity,
            cursor: &mut R,
            packet_sender: std::sync::Arc<PacketSender>
        ) -> Result<(), crate::errors::NetError> {
            match packet_id {
                #(#match_arms)*
                _ => {
                    tracing::debug!("No packet found for ID: 0x{:02X}", packet_id);
                    Err(crate::errors::PacketError::InvalidPacket(packet_id).into())
                },
            }
        }

        #(#receiver_structs)*

        pub struct PacketSender {
            #(#sender_mega_struct_fields)*
        }

        pub fn create_packet_senders(world: &mut bevy_ecs::world::World) -> PacketSender {
            #(#send_recv_pairs)*
            let mut packet_senders = PacketSender {
                #(#build_mega_struct)*
            };
            #(#register_structs)*
            packet_senders
        }
    };

    TokenStream::from(output)
}
