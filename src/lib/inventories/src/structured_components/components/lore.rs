use crate::structured_components::data::StructuredTextComponent;
use ferrumc_macros::{NetDecode, NetEncode};
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]

pub struct Lore {
    pub contents: LengthPrefixedVec<StructuredTextComponent>,
}
