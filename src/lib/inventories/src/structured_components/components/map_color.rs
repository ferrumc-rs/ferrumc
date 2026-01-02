use ferrumc_macros::{NetDecode, NetEncode};

#[derive(Debug, NetDecode, NetEncode, Clone, Hash, Default, PartialEq)]
pub struct MapColor {
    pub color: i32,
}
