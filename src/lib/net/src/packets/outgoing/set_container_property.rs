use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "container_set_data", state_id = "play")]
pub struct SetContainerPropertyPacket {
    pub window_id: u8,
    pub property: u16,
    pub value: u16,
}

impl SetContainerPropertyPacket {
    pub fn new(window_id: u8, property: u16, value: u16) -> Self {
        Self {
            window_id,
            property,
            value,
        }
    }
}
