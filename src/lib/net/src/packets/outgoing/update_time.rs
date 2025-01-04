use ferrumc_macros::Event;
use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode)]
#[packet(packet_id = "set_time", state = "play")]
pub struct UpdateTimePacket {
    pub world_age: i64,
    pub time_of_day: i64,
}

impl UpdateTimePacket {
    pub fn new(world_age: i64, time_of_day: i64) -> UpdateTimePacket {
        Self {
            world_age,
            time_of_day,
        }
    }
}

#[derive(Event, Clone, Copy)]
pub struct TickEvent {
    pub tick: i64,
}

impl TickEvent {
    pub fn new(tick: i64) -> Self {
        Self { tick }
    }
}
