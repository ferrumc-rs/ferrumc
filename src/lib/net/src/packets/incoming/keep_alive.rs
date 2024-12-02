use crate::packets::outgoing::keep_alive::OutgoingKeepAlivePacket;
use crate::packets::IncomingPacket;
use crate::utils::state::terminate_connection;
use crate::NetResult;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_state::ServerState;
use std::sync::Arc;
use tracing::debug;

#[derive(NetDecode)]
#[packet(packet_id = 0x18, state = "play")]
pub struct IncomingKeepAlivePacket {
    pub timestamp: i64,
}

impl IncomingPacket for IncomingKeepAlivePacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let last_sent_keep_alive = state.universe.get::<OutgoingKeepAlivePacket>(conn_id)?;
        if self.timestamp != last_sent_keep_alive.timestamp {
            debug!(
                "Invalid keep alive packet received from {:?} with id {:?} (expected {:?})",
                conn_id, self.timestamp, last_sent_keep_alive.timestamp
            );
            if let Err(e) =
                terminate_connection(state, conn_id, "Invalid keep alive packet".to_string()).await
            {
                debug!("Error terminating connection: {:?}", e);
            }
        } else {
            let mut last_rec_keep_alive =
                state.universe.get_mut::<IncomingKeepAlivePacket>(conn_id)?;
            *last_rec_keep_alive = self;
        }

        Ok(())
    }
}
