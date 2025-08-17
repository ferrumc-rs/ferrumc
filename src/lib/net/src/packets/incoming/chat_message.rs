use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetDecode, Debug, Clone)]
#[packet(packet_id = "chat", state = "play")]
pub struct ChatMessagePacket {
    pub message: String,
    pub timestamp: u64,
    pub salt: u64,
    pub has_signature: bool,
    pub signature: Option<Vec<u64>>,
    pub message_count: VarInt,
    pub acknowledged: Vec<u8>,
}
