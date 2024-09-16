#![allow(dead_code)]
use ferrumc_codec::network_types::varint::VarInt;
use ferrumc_macros::NetEncode;

#[derive(NetEncode)]
pub struct PlayerInfoUpdatePacket {
    actions: u8,
    number_of_players: VarInt,
    players: Vec<PlayerInfo>
}

#[derive(NetEncode)]
struct PlayerInfo {
    uuid: u128,
    actions: Vec<Action>
}

#[derive(NetEncode)]
enum Action {
    AddPlayer(AddPlayer),
    InitializeChat(InitializeChat),
}

#[derive(NetEncode)]
struct AddPlayer {
    name: String,
    number_of_properties: VarInt,
    properties: Vec<Property>,
}

#[derive(NetEncode)]
struct Property {
    name: String,
    value: String,
    is_signed: bool,
    signature: Option<String>,
}

#[derive(NetEncode)]
struct InitializeChat {
    message: String,
}