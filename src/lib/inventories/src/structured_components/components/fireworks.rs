use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct Fireworks {
    pub flight_duration: VarInt,
    pub explosions: LengthPrefixedVec<FireworkExplosion>,
}

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct FireworkExplosion {
    pub shape: VarInt,
    pub colors: LengthPrefixedVec<i32>,
    pub fade_colors: LengthPrefixedVec<i32>,
    pub has_trail: bool,
    pub has_twinkle: bool,
}
