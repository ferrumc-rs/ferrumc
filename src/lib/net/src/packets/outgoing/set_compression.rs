use std::io::Write;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetEncode)]
#[packet(packet_id = "login_compression", state = "login")]
pub struct SetCompressionPacket {
    threshold: VarInt
}


impl SetCompressionPacket {
    pub fn new(threshold: i32) -> Self {
        Self {
            threshold: VarInt::new(threshold)
        }
    }
}