use tracing::trace;

use ferrumc_macros::{packet, NetDecode};

use crate::net::packets::{ConnectionId, IncomingPacket};
use crate::state::GlobalState;

#[derive(NetDecode)]
#[packet(packet_id = 0x1C, state = "play")]
pub struct PlayerAbilities {
    pub flags: u8,
}

impl IncomingPacket for PlayerAbilities {
    async fn handle(
        self,
        _: ConnectionId,
        _state: GlobalState,
    ) -> crate::utils::prelude::Result<()> {
        trace!("PlayerAbilities packet received");
        trace!("Flags: {}", self.flags);
        Ok(())
    }
}
