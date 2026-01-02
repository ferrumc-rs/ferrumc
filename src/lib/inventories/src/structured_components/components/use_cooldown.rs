use ferrumc_macros::{NetDecode, NetEncode};
use crate::structured_components::data::{HashableF32, Identifier};
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct UseCooldown {
    pub seconds: HashableF32,
    pub cooldown_group: PrefixedOptional<Identifier>,
}
