use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use crate::structured_components::data::HashableF32;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct Weapon {
    pub damage_per_attack : VarInt,
    pub disable_blocking_for_seconds : HashableF32
}