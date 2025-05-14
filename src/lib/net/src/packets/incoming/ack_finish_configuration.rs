use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode)]
#[packet(packet_id = "finish_configuration", state = "configuration")]
pub struct AckFinishConfigurationPacket;
