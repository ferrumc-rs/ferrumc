use ferrumc_macros::{NetEncode, packet};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "game_event", state = "play")]
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
