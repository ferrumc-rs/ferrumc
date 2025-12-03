use crate::codec::net_types::var_int::VarInt;
use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode, Debug, Clone)]
#[packet(id = ids::PLAY_SERVERBOUND_COMMAND_SUGGESTION, state = "play")]
pub struct CommandSuggestionRequest {
    pub transaction_id: VarInt,
    pub input: String,
}
