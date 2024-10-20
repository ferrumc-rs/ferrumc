use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;
use tokio::io::AsyncWriteExt;

#[derive(NetEncode)]
#[packet(packet_id = 0x02)]
pub struct LoginSuccessPacket {
    pub uuid: u128,
    pub username: String,
    pub number_of_properties: VarInt,
    pub strict_error_handling: bool,
}

impl LoginSuccessPacket {
    pub fn new(uuid: u128, username: String) -> Self {
        Self {
            uuid,
            username,
            number_of_properties: VarInt::from(0),
            strict_error_handling: false,
        }
    }
}
