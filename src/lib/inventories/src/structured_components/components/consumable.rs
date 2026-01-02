use crate::structured_components::data::{ConsumeEffect, HashableF32, IdOr, SoundEvent};
use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct Consumable {
    pub consume_seconds: HashableF32,
    pub animation: VarInt,
    pub sound: IdOr<SoundEvent>,
    pub has_consume_particles: bool,
    pub effects: LengthPrefixedVec<ConsumeEffect>,
}
