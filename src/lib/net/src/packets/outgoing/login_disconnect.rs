use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "login_disconnect", state = "login")]
pub struct LoginDisconnectPacket<'a> {
    pub reason: &'a str,
}

impl<'a> LoginDisconnectPacket<'a> {
    pub fn new(reason: &'a str) -> Self {
        Self { reason }
    }
}
