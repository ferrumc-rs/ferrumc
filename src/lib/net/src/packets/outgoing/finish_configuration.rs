use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x03, state = "configuration")]
pub struct FinishConfigurationPacket;

impl Default for FinishConfigurationPacket {
    fn default() -> Self {
        Self::new()
    }
}

impl FinishConfigurationPacket {
    pub fn new() -> Self {
        Self {}
    }
}
