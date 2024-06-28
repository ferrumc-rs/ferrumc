use ferrumc_macros::{Encode};
use ferrumc_utils::encoding::varint::VarInt;

#[derive(Encode)]
pub struct LoginSuccess {
    pub packet_id: VarInt,
    pub uuid: Vec<u8>,
    pub username: String,
    // Just set this to 0
    pub property_count: VarInt,
    // TODO: Figure out how what in the everloving fuck this is
    pub properties: Vec<Property>,
    // For client gets an out of bounds read error when this is defined. I'd love to fix it but
    // it's probably dependant on the properties field tho
    pub strict_error: bool,
}

#[derive(Encode)]
pub struct Property {
    name: String,
    value: String,
    is_signed: bool,
    // Only if is_signed is true
    signature: String
}