use crate::packets::outgoing::keep_alive::OutgoingKeepAlive;
use crate::packets::IncomingPacket;
use crate::utils::state::terminate_connection;
use crate::{NetResult, ServerState};
use ferrumc_macros::{packet, NetDecode};
use std::sync::Arc;
use tracing::debug;

#[derive(NetDecode)]
#[packet(packet_id = 0x18, state = "play")]
pub struct IncomingKeepAlive {
    pub timestamp: i64,
}

impl IncomingPacket for IncomingKeepAlive {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        let last_sent_keep_alive = state.universe.get::<OutgoingKeepAlive>(conn_id)?;
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
            let mut last_rec_keep_alive = state.universe.get_mut::<IncomingKeepAlive>(conn_id)?;
            *last_rec_keep_alive = self;
        }

        Ok(())
    }
}
