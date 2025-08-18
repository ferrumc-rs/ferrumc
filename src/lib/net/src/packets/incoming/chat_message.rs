use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::{prefixed_optional::PrefixedOptional, var_int::VarInt};

#[derive(NetDecode)]
#[packet(packet_id = "chat", state = "play")]
pub struct ChatMessagePacket {
    pub message: String,
    pub timestamp: u64,
    pub salt: u64,
    pub has_signature: bool,
    pub signature: PrefixedOptional<Vec<u64>>,
    pub message_count: VarInt,
    pub acknowledged: Vec<u8>,
}
