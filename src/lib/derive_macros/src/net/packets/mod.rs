use crate::static_loading::packets::{get_packet_id, PacketBoundiness};
use colored::Colorize;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
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

    for entry in entries {
        let entry = entry.expect("entry failed");
        let path = entry.path();
        let file_name = path.file_name().expect("file_name failed").to_os_string();

        println!(
            "   {} {}",
            "[FERRUMC_MACROS]".bold().blue(),
            format!("Parsing file: {}", file_name.to_string_lossy())
                .white()
                .bold()
        );

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
                \nExample: #[packet(packet_id = 0x00, state = \"handshake\")]",
            );

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

    let match_arms = match_arms.into_iter();

    let output = quote! {
        pub async fn handle_packet<R: std::io::Read>(packet_id: u8, conn_id: usize, conn_state: &crate::connection::ConnectionState, cursor: &mut R, state: std::sync::Arc<ferrumc_state::ServerState>) -> crate::NetResult<()> {
            match (packet_id, conn_state.as_str()) {
                #(#match_arms)*
                _ => tracing::debug!("No packet found for ID: 0x{:02X} in state: {}", packet_id, conn_state.as_str()),
            }

            Ok(())
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
