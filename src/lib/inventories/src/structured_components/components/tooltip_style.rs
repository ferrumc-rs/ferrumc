use ferrumc_macros::{NetDecode, NetEncode};
use crate::structured_components::data::Identifier;

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct TooltipStyle {
    pub style : Identifier
}