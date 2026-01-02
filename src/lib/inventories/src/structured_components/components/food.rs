use crate::structured_components::data::HashableF32;
use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct Food {
    pub nutrition: VarInt,
    pub saturation: HashableF32,
    pub can_always_eat: bool,
}
