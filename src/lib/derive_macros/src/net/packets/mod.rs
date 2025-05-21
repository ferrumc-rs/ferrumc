use crate::static_loading::packets::{get_packet_id, PacketBoundiness};
use colored::Colorize;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use regex::Regex;
use std::env;
use std::ops::Add;
use syn::{parse_macro_input, Attribute};

/// Returns: (state, packet_id)
fn parse_packet_attribute(attr: &Attribute) -> Option<(String, String)> {
    let attr_str = attr.to_token_stream().to_string();

    // This regex matches both formats:
    // #[packet(packet_id = "something", state = "play")]
    let re = Regex::new(r#"packet_id\s*=\s*"([^"]+)"(?:\s*,\s*)?state\s*=\s*"([^"]+)""#).unwrap();

    if let Some(caps) = re.captures(&attr_str) {
        let packet_id = caps.get(1).map(|m| m.as_str().to_string())?;
        let state = caps.get(2).map(|m| m.as_str().to_string())?;
        Some((state, packet_id))
    } else {
        None
    }
}

/// Returns: (state, packet_id)
pub(crate) fn get_packet_details_from_attributes(
    attrs: &[Attribute],
    bound_to: PacketBoundiness,
) -> Option<(String, u8)> {
    let mut val = Option::<(String, String)>::None;

    for attr in attrs {
        if !attr.path().is_ident("packet") {
            continue;
        }

        val = parse_packet_attribute(attr);
    }

    let (state, packet_id) = val?;

    let packet_id =
        parse_packet_id(state.as_str(), packet_id, bound_to).expect("parse_packet_id failed");

    Some((state, packet_id))
}

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

/// Essentially, this just reads all the files in the directory and generates a match arm for each packet.
/// (packet_id, state) => { ... }
pub fn setup_packet_handling(input: TokenStream) -> TokenStream {
    #[cfg(feature = "colors")]
    colored::control::set_override(true);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let module_path = parse_macro_input!(input as syn::LitStr).value();

    let mut path = manifest_dir.add(module_path.as_str());
    path = path.replace("\\", "/");

    let dir_path = std::path::Path::new(&path);
    // get the module path like crate::xxx:xxx from module_path
    let base_path = module_path.split("\\").collect::<Vec<&str>>()[2..].join("::");
    let base_path = format!("crate::{}", base_path);

    println!(
        "   {} {}",
        "[FERRUMC_MACROS]".blue().bold(),
        format!("Parsing packets in {}", dir_path.display())
            .white()
            .bold()
    );

    if !std::fs::metadata(dir_path).unwrap().is_dir() {
        return TokenStream::from(quote! {
            compile_error!("The path provided is not a directory");
        });
    }

    let mut match_arms = Vec::new();

    let start = std::time::Instant::now();

    let entries = std::fs::read_dir(dir_path).expect("read_dir call failed");

    let mut packet_channel_structs = vec![];

    for entry in entries {
        let entry = entry.expect("entry failed");
        let path = entry.path();
        let file_name = path.file_name().expect("file_name failed").to_os_string();

        if !path.is_file() {
            continue;
        }

        let content = std::fs::read_to_string(&path).expect("read_to_string failed");
        let syntax = syn::parse_file(&content).expect("parse_file failed");

        for item in syntax.items {
            let syn::Item::Struct(item_struct) = item else {
                continue;
            };

            // If the struct does not have the #[packet(...)] attribute, then skip it.
            if !item_struct
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("packet"))
            {
                continue;
            }

            // format: #[packet(packet_id = 0x00, state = "handshake")]
            let (state, packet_id) = get_packet_details_from_attributes(
                &item_struct.attrs,
                PacketBoundiness::Serverbound,
            )
                .expect(
                    "parse_packet_attribute failed\
                \nPlease provide the packet_id and state fields in the #[packet(...)] attribute.\
                \nExample: #[packet(packet_id = \"example_packet\", state = \"handshake\")]",
                );

            if state == "play" {
                let struct_name = item_struct.ident;

                println!(
                    "   {} {} (ID: {}, State: {}, Struct Name: {})",
                    "[FERRUMC_MACROS]".bold().blue(),
                    "Found Packet".white().bold(),
                    format!("0x{:02X}", packet_id).cyan(),
                    state.green(),
                    struct_name.to_string().yellow()
                );

                let path = format!(
                    "{}::{}",
                    base_path,
                    file_name.to_string_lossy().replace(".rs", "")
                );
                let struct_path = format!("{}::{}", path, struct_name);

                packet_channel_structs.push((struct_name.clone(), struct_path.clone()));

                let struct_path =
                    syn::parse_str::<syn::Path>(&struct_path).expect("parse_str failed");

                let field_name = syn::parse_str::<syn::Ident>(&to_snake_case(&struct_name.to_string()))
                    .expect("to_snake_case failed");

                match_arms.push(quote! {
                        (#packet_id) => {
                            // let packet= #struct_path::net_decode(cursor)?;
                            let packet = <#struct_path as ferrumc_net_codec::decode::NetDecode>::decode(cursor, &ferrumc_net_codec::decode::NetDecodeOpts::None)?;
                            packet_sender.#field_name.send((packet, entity)).expect("Failed to send packet");
                            Ok(())
                        },
                    });
            }
        }
    }

    let elapsed = start.elapsed();
    println!(
        "   {} {}",
        "[FERRUMC_MACROS]".bold().blue(),
        format!("Found {} packets", match_arms.len())
            .purple()
            .bold()
    );
    println!(
        "   {} {}",
        "[FERRUMC_MACROS]".bold().blue(),
        format!(
            "It took: {:?} to parse all the files and generate the packet registry",
            elapsed
        )
            .red()
            .bold()
    );

    let mut sender_mega_struct_fields = vec![];
    let mut send_recv_pairs = vec![];
    let mut build_mega_struct = vec![];
    let mut register_structs = vec![];
    let mut receiver_structs = vec![];

    packet_channel_structs
        .iter()
        .for_each(|(struct_name, path)| {
            let appended_name = format_ident!("{}Receiver", struct_name);
            let snake_case_name =
                syn::parse_str::<syn::Ident>(&to_snake_case(&struct_name.to_string()))
                    .expect("to_snake_case failed");
            let struct_path = syn::parse_str::<syn::Path>(path).expect("parse_str failed");
            sender_mega_struct_fields.push(quote! {
                pub #snake_case_name: Sender<(#struct_path, bevy_ecs::entity::Entity)>,
            });
            let sender_name = format_ident!("{}_sender", snake_case_name);
            let receiver_name = format_ident!("{}_receiver", snake_case_name);
            send_recv_pairs.push(quote! {
                let (#sender_name, #receiver_name) = crossbeam_channel::unbounded();
            });
            build_mega_struct.push(quote! {
                #snake_case_name: #sender_name,
            });
            register_structs.push(quote! {
                world.insert_resource(#appended_name(#receiver_name));
            });
            receiver_structs.push(quote! {
                #[derive(Resource)]
                pub struct #appended_name(pub Receiver<(#struct_path, bevy_ecs::entity::Entity)>);
            });
        });

    let match_arms = match_arms.into_iter();

    let output = quote! {
        pub fn handle_packet<R: std::io::Read>(packet_id: u8, entity: bevy_ecs::entity::Entity, cursor: &mut R, packet_sender: Arc<PacketSender>) -> Result<(), crate::errors::NetError> {
            match (packet_id) {
                #(#match_arms)*
                _ => {tracing::debug!("No packet found for ID: 0x{:02X} (from {})", packet_id, entity); Err(crate::errors::PacketError::InvalidPacket(packet_id).into())},
            }
        }

        #(#receiver_structs)*

        pub struct PacketSender {
            #(#sender_mega_struct_fields)*
        }

        pub fn create_packet_senders(world: &mut World) -> PacketSender {
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

fn parse_packet_id(state: &str, value: String, bound_to: PacketBoundiness) -> syn::Result<u8> {
    //! Sorry to anyone reading this code. The get_packet_id method PANICS if there is any type of error.
    //! these macros are treated like trash gah damn. they need better care 😔

    // If the user provided a direct integer (like 0x01, or any number) value.
    if value.starts_with("0x") {
        let value = value.strip_prefix("0x").expect("strip_prefix failed");
        let n = u8::from_str_radix(value, 16).expect("from_str_radix failed");
        return Ok(n);
    }

    // If the user provided referencing packet id, then just get that.
    let n = get_packet_id(state, bound_to, value.as_str());

    Ok(n)
}

/// `#[packet]` attribute is used to declare an incoming/outgoing packet.
///
/// <b>packet_id</b> => The packet id of the packet. In hexadecimal.
/// <b>state</b> => The state of the packet. Can be: "handshake", "status", "login", "play".
///
/// e.g.
/// ```ignore
/// use ferrumc_macros::NetDecode;
///
/// #[derive(NetDecode)]
/// #[packet(packet_id = 0x05, state = "play")]
/// pub struct PacketChatMessage {
///     pub message: String,
///     pub timestamp: i64,
/// }
/// ```
///
/// ```ignore
/// use ferrumc_macros::{packet, NetEncode};
///
/// #[derive(NetEncode)]
/// #[packet(packet_id = 0x05)]
/// pub struct PacketChatMessage {
///    pub message: String,
///    pub timestamp: i64,
/// }
pub fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    // These are just some checks to make sure the packet attribute is used correctly.
    // This is not actual functionality.
    // The actual functionality is in the `bake_registry` function.

    const E: &str = "packet attribute must have the packet_id and/or state fields. In case of incoming: both. In case of outgoing: only packet_id.";
    if args.is_empty() {
        return TokenStream::from(quote! {
            compile_error!(#E);
        });
    }

    if !&["packet_id", "state"]
        .iter()
        .all(|x| args.to_string().contains(x))
    {
        return TokenStream::from(quote! {
            compile_error!(#E);
        });
    }

    input
}
