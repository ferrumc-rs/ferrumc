use ferrumc_macros::{NetEncode, packet};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "finish_configuration", state = "configuration")]
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
