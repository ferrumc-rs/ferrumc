use ferrumc_macros::{Encode};

#[derive(Encode)]
pub struct LoginSuccess {
    pub packet_id: VarInt,
    pub uuid: u128,
    pub username: String,
    // Just set this to 0
    pub property_count: VarInt,
    // TODO: Figure out how what in the everloving fuck this is
    // pub properties: Vec<Property>,
    // For client gets an out of bounds read error when this is defined. I'd love to fix it but
    // it's probably dependant on the properties field tho
    // pub strict_error: bool,
}