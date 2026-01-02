use crate::structured_components::data::potion_effect::PotionEffect;
use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]

pub struct PotionContents {
    pub potion_id: PrefixedOptional<VarInt>,
    pub custom_color: PrefixedOptional<i32>,
    pub custom_effects: LengthPrefixedVec<PotionEffect>,
    pub custom_name: String,
}
