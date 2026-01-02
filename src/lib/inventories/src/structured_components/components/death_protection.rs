use ferrumc_macros::{NetDecode, NetEncode};
use crate::structured_components::data::ConsumeEffect;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct DeathProtection {
    pub effects: LengthPrefixedVec<ConsumeEffect>,
}
