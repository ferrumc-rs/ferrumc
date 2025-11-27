use ferrumc_macros::{packet, NetDecode};
use ferrumc_protocol::ids;

#[derive(NetDecode)]
#[packet(id = ids::CONFIGURATION_CLIENTBOUND_FINISH_CONFIGURATION, state = "configuration")]
pub struct AckFinishConfigurationPacket;
