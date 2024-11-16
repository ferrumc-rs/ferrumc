use crate::connection::StreamWriter;
use crate::packets::outgoing::disconnect::Disconnect;
use crate::packets::outgoing::keep_alive::OutgoingKeepAlivePacket;

use crate::packets::IncomingPacket;
// use crate::player_ext::PlayerExt;
use crate::{NetResult, ServerState};
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::encode::NetEncodeOpts;
use std::sync::Arc;
use tracing::{debug, info, warn};


#[derive(NetDecode)]
#[packet(packet_id = 0x18, state = "play")]
pub struct IncomingKeepAlivePacket {
    pub id: i64,
}

impl IncomingPacket for IncomingKeepAlivePacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        // TODO handle errors.
        let last_keep_alive = state.universe.get_mut::<OutgoingKeepAlivePacket>(conn_id)?;
        let mut writer = state.universe.get_mut::<StreamWriter>(conn_id)?;


        if self.id != last_keep_alive.id {
            debug!(
                "Invalid keep alive packet received from {:?} with id {:?} (expected {:?})",
                conn_id, self.id, last_keep_alive.id
            );
            let packet = Disconnect::from_string("Invalid Keep Alive".to_string());
            match writer
                .send_packet(&packet, &NetEncodeOpts::WithLength)
                .await
            {
                Ok(_) => {
                    info!(
                        "Disconnected entity {:?} for an invalid keep alive",
                        conn_id
                    )
                }
                Err(err) => {
                    debug!("Failed to disconnect player : {:?}", err)
                }
            }
            return NetResult::Err(crate::errors::NetError::Packet(
                crate::errors::PacketError::InvalidState(0x18),
            ));
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
            *last_keep_alive = KeepAlive::from(self.id);
        }

        Ok(())
    }
}
