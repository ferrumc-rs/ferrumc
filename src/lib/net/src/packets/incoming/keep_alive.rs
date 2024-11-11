use crate::packets::outgoing::keep_alive::OutgoingKeepAlivePacket;
use crate::packets::IncomingPacket;
use crate::{NetResult, ServerState};
use ferrumc_ecs::components::storage::ComponentRefMut;
use ferrumc_ecs::errors::ECSError;
use ferrumc_macros::{packet, NetDecode};
use std::sync::Arc;
use tracing::{debug, warn};

#[derive(NetDecode)]
#[packet(packet_id = 0x18, state = "play")]
pub struct IncomingKeepAlivePacket {
    pub id: i64,
}

impl IncomingPacket for IncomingKeepAlivePacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        // TODO handle errors.
        let last_keep_alive = state.universe.get_mut::<OutgoingKeepAlivePacket>(conn_id)?;

        if self.id != last_keep_alive.id {
            debug!(
                "Invalid keep alive packet received from entity {:?} with id {:?} (expected {:?})",
                conn_id, self.id, last_keep_alive.id
            );
            return NetResult::Err(crate::errors::NetError::Packet(
                crate::errors::PacketError::InvalidState(0x18),
            ));
            // TODO Kick player
        }

        let result = state.universe.get_mut::<IncomingKeepAlivePacket>(conn_id);

        if result.is_err() {
            let err = result.as_ref().err().unwrap();
            if matches!(err, ECSError::ComponentTypeNotFound)
                || matches!(err, ECSError::ComponentRetrievalError)
            {
                state
                    .universe
                    .add_component(conn_id, IncomingKeepAlivePacket { id: self.id })?;
                let mut last_received_keep_alive = state.universe.get_mut(conn_id)?;
                *last_received_keep_alive = self;
                debug!("Added <IncomingKeepAlive> component to entity {}", conn_id);
                return Ok(());
            } else {
                warn!(
                    "Failed to get or create <IncomingKeepAlive> component: {:?}",
                    err
                );
                return Err(crate::errors::NetError::ECSError(result.err().unwrap()));
            }
        } else {
            let mut last_received_keep_alive: ComponentRefMut<'_, IncomingKeepAlivePacket> =
                result.unwrap();

            *last_received_keep_alive = self;
        }

        Ok(())
    }
}
