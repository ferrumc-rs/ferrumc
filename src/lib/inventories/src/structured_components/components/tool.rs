use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_net_codec::net_types::var_int::VarInt;
use crate::structured_components::data::{HashableF32, IdSet};

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct Tool {
    pub rules : LengthPrefixedVec<ToolRule>,
    pub default_mining_speed : HashableF32,
    pub damage_per_block : VarInt,
    pub can_destroy_block_in_creative : bool,
}

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct ToolRule {
    pub blocks : IdSet,
    pub speed : PrefixedOptional<HashableF32>,
    pub correct_drop_for_block : PrefixedOptional<bool>,
}