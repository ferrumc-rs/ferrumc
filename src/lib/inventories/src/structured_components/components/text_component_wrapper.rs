use crate::structured_components::data::StructuredTextComponent;
use ferrumc_macros::{NetDecode, NetEncode};

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct TextComponentWrapper {
    pub name: StructuredTextComponent,
}
