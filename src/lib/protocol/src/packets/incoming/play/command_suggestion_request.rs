use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::codec::net_types::var_int::VarInt;
use ferrumc_protocol::ids;

#[derive(NetDecode, Debug, Clone)]
#[packet(id = ids::PLAY_SERVERBOUND_COMMAND_SUGGESTION, state = "play")]
pub struct CommandSuggestionRequest {
    pub transaction_id: VarInt,
    pub input: String,
}
