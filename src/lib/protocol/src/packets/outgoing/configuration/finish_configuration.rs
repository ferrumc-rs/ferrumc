use crate::ids;
use ferrumc_macros::{NetEncode, packet};

#[derive(NetEncode)]
#[packet(id = ids::CONFIGURATION_CLIENTBOUND_FINISH_CONFIGURATION, state = "configuration")]
pub struct FinishConfigurationPacket;

impl Default for FinishConfigurationPacket {
    fn default() -> Self {
        Self::new()
    }
}

impl FinishConfigurationPacket {
    pub fn new() -> Self {
        Self {}
    }
}
