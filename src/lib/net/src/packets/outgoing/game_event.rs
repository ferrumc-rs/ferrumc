use ferrumc_macros::{packet, NetEncode};
use std::io::Write;
use tokio::io::AsyncWriteExt;

#[derive(NetEncode)]
#[packet(packet_id = 0x22)]
pub struct GameEventPacket {
    pub event_id: u8,
    pub value: f32,
}

impl GameEventPacket {
    pub fn new(event_id: u8, value: f32) -> Self {
        Self { event_id, value }
    }

    pub fn start_waiting_for_level_chunks() -> Self {
        Self::new(13, 0f32)
    }
}
