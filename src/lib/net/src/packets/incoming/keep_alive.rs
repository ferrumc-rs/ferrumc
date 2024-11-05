use crate::packets::outgoing::keep_alive::KeepAlive;
use crate::packets::IncomingPacket;
use crate::NetResult; use ferrumc_state::ServerState;
use ferrumc_macros::{packet, NetDecode};
use std::sync::Arc;
use tracing::debug;

#[derive(NetDecode)]
#[packet(packet_id = 0x18, state = "play")]
pub struct IncomingKeepAlivePacket {
    pub id: i64,
}

impl IncomingPacket for IncomingKeepAlivePacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let mut last_keep_alive = state.universe.get_mut::<KeepAlive>(conn_id)?;
        if self.id != last_keep_alive.id {
            debug!(
                "Invalid keep alive packet received from {:?} with id {:?} (expected {:?})",
                conn_id, self.id, last_keep_alive.id
            );
            // TODO Kick player
        } else {
            *last_keep_alive = KeepAlive::from(self.id);
        }

        Ok(())
    }
}
