use crate::codec::net_types::{
    length_prefixed_vec::LengthPrefixedVec, prefixed_optional::PrefixedOptional, var_int::VarInt,
};
use crate::ids;
use ferrumc_macros::{NetEncode, packet};
use ferrumc_text::TextComponent;

#[derive(NetEncode)]
#[packet(id = ids::PLAY_CLIENTBOUND_COMMAND_SUGGESTIONS, state = "play")]
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
