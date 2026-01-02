use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct MaxDamage {
    pub max_damage: VarInt,
}
