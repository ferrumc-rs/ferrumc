use ferrumc_macros::{NetDecode, NetEncode};
use crate::structured_components::data::IdSet;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct Repairable {
    pub items: IdSet,
}
