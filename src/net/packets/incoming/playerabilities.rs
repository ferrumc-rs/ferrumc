use tracing::trace;

use ferrumc_macros::{Decode, packet};

use crate::Connection;
use crate::packets::IncomingPacket;

#[derive(Decode)]
#[packet(packet_id = 0x1C, state = "play")]
pub struct PlayerAbilities {
    pub flags: u8,
}

impl IncomingPacket for PlayerAbilities {
    async fn handle(&self, _: &mut Connection) -> ferrumc_utils::prelude::Result<()> {
        trace!("PlayerAbilities packet received");
        trace!("Flags: {}", self.flags);
        Ok(())
    }
}
