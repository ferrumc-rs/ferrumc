use proc_macro2::TokenStream;
use quote::quote;
use std::fmt::Display;
use std::sync::LazyLock;
use syn::parse::Parse;
use syn::{parse_macro_input, LitStr, Token};

pub(crate) static PACKETS_JSON: LazyLock<serde_json::Value> = LazyLock::new(|| {
    let json_str = include_str!("../../../../../assets/data/packets.json");
    serde_json::from_str(json_str).unwrap()
});

pub(crate) fn get_packet_id(
    state: impl Into<PacketState>,
    bound: PacketBoundiness,
    packet_name: &str,
) -> u8 {
    let mut current_value = &*PACKETS_JSON;

    // remove `"` from start and end of the packet_name:
    let packet_name = packet_name.trim_matches('"');

    let state = state.into();
    current_value = current_value
        .get(state.to_string())
        .unwrap_or_else(|| panic!("Could not find key: {}.", state));

    current_value = current_value
        .get(bound.to_string())
        .unwrap_or_else(|| panic!("Could not find key: {}", bound));

    current_value = current_value.get(format!("minecraft:{}", packet_name))
        .unwrap_or_else(|| panic!("Could not find key: `minecraft:{}` in the packet registry. Example: `add_entity`, would be 0x01 in the 1.21.1 protocol", packet_name));

    let protocol_id = current_value
        .get("protocol_id")
        .and_then(|v| v.as_u64())
        .unwrap_or_else(|| panic!("Could not find key: {}", "protocol_id"));

    protocol_id as u8
}

struct PacketTypeInput {
    state: PacketState,
    bound: PacketBoundiness,
    packet_name: String,
}

pub(crate) enum PacketState {
    Configuration,
    Handshake,
    Login,
    Play,
    Status,
}

impl From<&str> for PacketState {
    fn from(value: &str) -> Self {
        match value {
            "configuration" => PacketState::Configuration,
            "handshake" => PacketState::Handshake,
            "login" => PacketState::Login,
            "play" => PacketState::Play,
            "status" => PacketState::Status,
            wrong => panic!("Invalid state: {}. Must be: `configuration`, `handshake`, `login`, `play`, or `status`", wrong),
        }
    }
}

impl Display for PacketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketState::Configuration => write!(f, "configuration"),
            PacketState::Handshake => write!(f, "handshake"),
            PacketState::Login => write!(f, "login"),
            PacketState::Play => write!(f, "play"),
            PacketState::Status => write!(f, "status"),
        }
    }
}

pub(crate) enum PacketBoundiness {
    Clientbound,
    Serverbound,
}

impl Display for PacketBoundiness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketBoundiness::Clientbound => write!(f, "clientbound"),
            PacketBoundiness::Serverbound => write!(f, "serverbound"),
        }
    }
}

impl Parse for PacketTypeInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // LitStr, Token![,], LitStr, Token![,], LitStr

        let state = match input.parse::<LitStr>()?.value().as_str() {
            "configuration" => PacketState::Configuration,
            "handshake" => PacketState::Handshake,
            "login" => PacketState::Login,
            "play" => PacketState::Play,
            "status" => PacketState::Status,
            wrong => panic!("Invalid state: {}. Must be: `configuration`, `handshake`, `login`, `play`, or `status`", wrong),
        };

        input.parse::<Token![,]>()?;

        let bound = match input.parse::<LitStr>()?.value().as_str() {
            "clientbound" => PacketBoundiness::Clientbound,
            "serverbound" => PacketBoundiness::Serverbound,
            wrong => panic!(
                "Invalid bound: {}. Must be: `clientbound` or `serverbound` ",
                wrong
            ),
        };

        input.parse::<Token![,]>()?;

        let packet_name = input.parse::<LitStr>()?.value();
        Ok(Self {
            state,
            bound,
            packet_name,
        })
    }
}

pub(crate) fn get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as PacketTypeInput);

    let protocol_id = get_packet_id(input.state, input.bound, &input.packet_name);

    let hex_id = {
        let str_value = format!("0x{:02X}", protocol_id);
        str_value.parse::<TokenStream>().unwrap()
    };

    let expanded = quote! {
        #hex_id
    };

    proc_macro::TokenStream::from(expanded)
}
