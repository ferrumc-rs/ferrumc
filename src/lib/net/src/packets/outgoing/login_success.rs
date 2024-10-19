use ferrumc_macros::NetEncode;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
pub struct LoginSuccessPacket {
    pub packet_id: VarInt,
    pub uuid: u128,
    pub username: String,
    pub number_of_properties: VarInt,
    pub strict_error_handling: bool,
}

impl LoginSuccessPacket {
    pub fn new(uuid: u128, username: String) -> Self {
        Self {
            packet_id: VarInt::from(0x02),
            uuid,
            username,
            number_of_properties: VarInt::from(0),
            strict_error_handling: false,
        }
    }
}
