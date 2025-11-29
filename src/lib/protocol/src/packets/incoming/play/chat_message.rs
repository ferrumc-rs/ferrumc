use crate::codec::net_types::{prefixed_optional::PrefixedOptional, var_int::VarInt};
use crate::ids;
use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(id = ids::PLAY_SERVERBOUND_CHAT, state = "play")]
pub struct ChatMessagePacket {
    pub message: String,
    pub timestamp: u64,
    pub salt: u64,
    pub has_signature: bool,
    pub signature: PrefixedOptional<Vec<u64>>,
    pub message_count: VarInt,
    pub acknowledged: Vec<u8>,
}
