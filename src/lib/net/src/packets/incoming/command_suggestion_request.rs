use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetDecode, Debug, Clone)]
#[packet(packet_id = "command_suggestion", state = "play")]
pub struct CommandSuggestionRequest {
    pub transaction_id: VarInt,
    pub input: String,
}