use std::io::Write;
use ferrumc_macros::{packet, NetEncode};

#[derive(NetEncode)]
#[packet(packet_id = "start_configuration", state = "play")]
pub struct StartConfigurationPacket {}