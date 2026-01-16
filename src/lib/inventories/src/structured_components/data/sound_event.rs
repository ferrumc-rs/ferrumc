use crate::structured_components::data::{HashableF32, Identifier};
use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::prefixed_optional::PrefixedOptional;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct SoundEvent {
    sound_name: Identifier,
    has_fixed_range: bool,
    fixed_range: PrefixedOptional<HashableF32>,
}
