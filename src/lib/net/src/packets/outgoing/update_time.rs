use ferrumc_macros::{packet, NetEncode};

#[derive(NetEncode)]
#[packet(packet_id = "set_time", state = "play")]
pub struct UpdateTimePacket {
    pub world_age: u64,
    pub time_of_day: u64,
    pub time_of_day_increasing: bool,
}