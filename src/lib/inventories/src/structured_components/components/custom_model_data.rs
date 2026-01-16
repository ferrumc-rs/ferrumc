use ferrumc_macros::{NetDecode, NetEncode};
use crate::structured_components::data::HashableF32;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct CustomModelData {
    pub floats: LengthPrefixedVec<HashableF32>,
    pub flags: LengthPrefixedVec<bool>,
    pub strings: LengthPrefixedVec<String>,
    pub colors: LengthPrefixedVec<i32>,
}
