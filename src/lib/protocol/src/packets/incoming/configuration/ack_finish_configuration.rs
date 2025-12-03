use ferrumc_macros::{NetDecode, packet};

#[derive(NetDecode)]
#[packet(id = ids::CONFIGURATION_CLIENTBOUND_FINISH_CONFIGURATION, state = "configuration")]
pub struct AckFinishConfigurationPacket;
