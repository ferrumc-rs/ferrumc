use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]

pub struct SuspiciousStewEffects {
    pub effects: LengthPrefixedVec<StewPotionEffect>,
}

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]

pub struct StewPotionEffect {
    pub effect_id: VarInt,
    pub duration: VarInt,
}
