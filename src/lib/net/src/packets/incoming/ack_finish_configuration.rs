use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(packet_id = "finish_configuration", state = "configuration")]
pub struct AckFinishConfigurationPacket;
