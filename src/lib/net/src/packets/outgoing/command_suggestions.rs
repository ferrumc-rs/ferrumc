use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::{
    length_prefixed_vec::LengthPrefixedVec, prefixed_optional::PrefixedOptional, var_int::VarInt,
};
use ferrumc_text::TextComponent;
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "command_suggestions", state = "play")]
pub struct CommandSuggestionsPacket {
    pub transaction_id: VarInt,
    pub start: VarInt,
    pub length: VarInt,
    pub matches: LengthPrefixedVec<Match>,
}

#[derive(NetEncode)]
pub struct Match {
    pub content: String,
    pub tooltip: PrefixedOptional<TextComponent>,
}
