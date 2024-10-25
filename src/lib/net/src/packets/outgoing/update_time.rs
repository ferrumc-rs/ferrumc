use ferrumc_macros::{packet, NetEncode};
use std::io::Write;
use tokio::io::AsyncWriteExt;

#[derive(NetEncode)]
#[packet(packet_id = 0x64)]
pub struct UpdateTimePacket {
    pub world_age: i64,
    pub time_of_day: i64,
}

impl UpdateTimePacket {
    pub fn new(world_age: i64, time_of_day: i64) -> UpdateTimePacket {
        Self {
            world_age: world_age,
            time_of_day: time_of_day,
        }
    }
}
