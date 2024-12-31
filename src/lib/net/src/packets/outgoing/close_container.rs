use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = 0x12)]
pub struct CloseContainerPacket {
    pub window_id: u8,
}

impl CloseContainerPacket {
    pub fn new(window_id: u8) -> Self {
        Self { window_id }
    }
}