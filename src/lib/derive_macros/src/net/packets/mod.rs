use proc_macro::TokenStream;
use quote::quote;
use std::env;
use std::ops::Add;
use syn::{parse_macro_input, LitInt, LitStr};
use colored::Colorize;

/// Essentially, this just reads all the files in the directory and generates a match arm for each packet.
/// (packet_id, state) => { ... }
pub fn bake_registry(input: TokenStream) -> TokenStream {
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
        format!("Parsing packets in {}", dir_path.display()).white().bold()
    );

    if !std::fs::metadata(dir_path).unwrap().is_dir() {
        return TokenStream::from(quote! {
            compile_error!("The path provided is not a directory");
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

        let content = std::fs::read_to_string(&path).expect("read_to_string failed");
        let syntax = syn::parse_file(&content).expect("parse_file failed");

        for item in syntax.items {
            let syn::Item::Struct(item_struct) = item else {
                continue;
            };

            // format: #[packet(packet_id = 0x00, state = "handshake")]

            let mut packet_id: Option<u8> = None;
            let mut state: Option<String> = None;

            for attr in item_struct.attrs {
                // #[packet(...)] part.
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
                            let n: u8 = value.base10_parse().expect("base10_parse failed");
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
                }).unwrap();

                let packet_id = packet_id.expect("packet_id not found");

                let state = state.clone().expect("state not found");
                let struct_name = &item_struct.ident;

                println!(
                    "   {} {} (ID: {}, State: {}, Struct Name: {})",
                    "[FERRUMC_MACROS]".bold().blue(),
                    "Found Packet".white().bold(),
                    format!("0x{:02X}", packet_id).cyan(),
                    state.green(),
                    struct_name.to_string().yellow()
                );

                let path = format!(
                    // "crate::net::packets::incoming::{}",
                    "{}::{}",
                    base_path,
                    file_name.to_string_lossy().replace(".rs", "")
                );
                let struct_path = format!("{}::{}", path, struct_name);

                let struct_path = syn::parse_str::<syn::Path>(&struct_path).expect("parse_str failed");

                match_arms.push(quote! {
                    (#packet_id, #state) => {
                        // let packet= #struct_path::net_decode(cursor).await?;
                        let packet = <#struct_path as ferrumc_net_codec::decode::NetDecode>::decode(cursor, &ferrumc_net_codec::decode::NetDecodeOpts::None)?;
                        // packet.handle(conn_id, state).await?;
                        <#struct_path as crate::packets::IncomingPacket>::handle(packet, conn_id, state).await?;
                        // tracing::debug!("Received packet: {:?}", packet);
                    },
                });
            }
        }
    }

    let elapsed = start.elapsed();
    println!(
        "   {} {}",
        "[FERRUMC_MACROS]".bold().blue(),
        format!("Found {} packets", match_arms.len()).purple().bold()
    );
    println!(
        "   {} {}",
        "[FERRUMC_MACROS]".bold().blue(),
        format!("It took: {:?} to parse all the files and generate the packet registry", elapsed).red().bold()
    );

    let match_arms = match_arms.into_iter();
    
    let output = quote! {
        pub async fn handle_packet<R: std::io::Read>(packet_id: u8, conn_id: usize, conn_state: &crate::connection::ConnectionState, cursor: &mut R, state: std::sync::Arc<crate::ServerState>) -> crate::NetResult<()> {
            match (packet_id, conn_state.as_str()) {
                #(#match_arms)*
                _ => tracing::debug!("No packet found for ID: 0x{:02X} in state: {}", packet_id, conn_state.as_str()),
            }
            
            Ok(())
        }
    };
    
    TokenStream::from(output)
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
        .any(|x| args.to_string().contains(x))
    {
        return TokenStream::from(quote! {
            compile_error!(#E);
        });
    }

    input
}