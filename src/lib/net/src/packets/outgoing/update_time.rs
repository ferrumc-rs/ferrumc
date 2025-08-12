use ferrumc_macros::{packet, NetEncode};
use std::io::Write;

#[derive(NetEncode, Copy, Clone)]
#[packet(packet_id = "set_time", state = "play")]
pub struct UpdateTimePacket {
    pub world_age: i64,
    pub time_of_day: i64,
    pub time_of_day_increasing: bool,
}
