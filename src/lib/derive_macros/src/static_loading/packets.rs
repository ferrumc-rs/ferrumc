use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::LazyLock;
use syn::parse::Parse;
use syn::{parse_macro_input, LitStr, Token};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PacketId {
    protocol_id: u8,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PacketDirection {
    #[serde(default)]
    pub clientbound: HashMap<String, PacketId>,
    #[serde(default)]
    pub serverbound: HashMap<String, PacketId>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Packets {
    pub configuration: PacketDirection,
    pub handshake: PacketDirection,
    pub login: PacketDirection,
    pub play: PacketDirection,
    pub status: PacketDirection,
}

pub(crate) static PACKETS_JSON: LazyLock<Packets> = LazyLock::new(|| {
    let json_str = include_str!("../../../../../assets/data/packets.json");
    serde_json::from_str(json_str).unwrap()
});

pub(crate) fn get_packet_id(
    state: impl Into<PacketState>,
    bound: PacketBoundiness,
    packet_name: &str,
) -> u8 {
    let packets = &*PACKETS_JSON;

    // remove `"` from start and end of the packet_name:
    let packet_name = packet_name.trim_matches('"');

    let state = state.into();
    let packets = match state {
        PacketState::Play => &packets.play,
        PacketState::Login => &packets.login,
        PacketState::Status => &packets.status,
        PacketState::Handshake => &packets.handshake,
        PacketState::Configuration => &packets.configuration,
    };
    let packets = match bound {
        PacketBoundiness::Clientbound => &packets.clientbound,
        PacketBoundiness::Serverbound => &packets.serverbound,
    };
    let id = packets.get(&format!("minecraft:{packet_name}"))
        .unwrap_or_else(|| panic!("Could not find key: `minecraft:{packet_name}` in the packet registry. Example: `add_entity`, would be 0x01 in the 1.21.1 protocol"));

    id.protocol_id
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
            wrong => panic!("Invalid state: {wrong}. Must be: `configuration`, `handshake`, `login`, `play`, or `status`"),
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
            wrong => panic!("Invalid state: {wrong}. Must be: `configuration`, `handshake`, `login`, `play`, or `status`"),
        };

        input.parse::<Token![,]>()?;

        let bound = match input.parse::<LitStr>()?.value().as_str() {
            "clientbound" => PacketBoundiness::Clientbound,
            "serverbound" => PacketBoundiness::Serverbound,
            wrong => panic!("Invalid bound: {wrong}. Must be: `clientbound` or `serverbound` "),
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

pub fn lookup_packet(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as PacketTypeInput);

    let protocol_id = get_packet_id(input.state, input.bound, &input.packet_name);

    let hex_id = {
        let str_value = format!("0x{protocol_id:02X}");
        str_value.parse::<TokenStream>().unwrap()
    };

    let expanded = quote! {
        #hex_id
    };

    proc_macro::TokenStream::from(expanded)
}
