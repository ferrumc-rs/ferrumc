use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct TooltipDisplay {
    pub hide_tooltip: bool,
    pub hidden_components: LengthPrefixedVec<VarInt>,
}
