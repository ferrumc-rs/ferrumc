use crate::codec::net_types::var_int::VarInt;
use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode, Debug)]
#[packet(id = ids::HANDSHAKE_SERVERBOUND_INTENTION, state = "handshake")]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}
