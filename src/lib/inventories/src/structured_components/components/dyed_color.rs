use ferrumc_macros::{NetDecode, NetEncode};

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct DyedColor {
    pub color: i32,
}
