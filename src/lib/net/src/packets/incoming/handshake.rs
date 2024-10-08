use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_types::var_int::VarInt;

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x00, state = "handshake")]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}