use crate::structured_components::data::{HashableF32, IdOr, IdSet, Identifier, SoundEvent};
use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct BlocksAttacks {
    pub block_delay_seconds: HashableF32,
    pub disable_cooldown_scale: HashableF32,
    pub damage_reductions: LengthPrefixedVec<DamageReduction>,
    pub item_damage_threshold: HashableF32,
    pub item_damage_base: HashableF32,
    pub item_damage_factor: HashableF32,
    pub bypassed_by: PrefixedOptional<Identifier>,
    pub block_sound: PrefixedOptional<IdOr<SoundEvent>>,
    pub disable_sound: PrefixedOptional<IdOr<SoundEvent>>,
}

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct DamageReduction {
    pub horizontal_blocking_angle: HashableF32,
    pub types: PrefixedOptional<IdSet>,
    pub base: HashableF32,
    pub factor: HashableF32,
}
