use async_trait::async_trait;
use tracing::{debug, trace, warn};

use ferrumc_macros::AutoGenName;

use crate::{ConnectionWrapper, net::drop_conn};
use crate::net::packets::outgoing::keep_alive::KeepAlivePacketOut;
use crate::net::systems::System;
use crate::state::GlobalState;
use crate::utils::components::keep_alive::KeepAlive;
use crate::utils::components::player::Player;

#[derive(AutoGenName)]
pub struct KeepAliveSystem;

#[async_trait]
impl System for KeepAliveSystem {
    async fn run(&self, state: GlobalState) {
        let sender = KeepAliveSystem::sender(state.clone());
        let receiver = KeepAliveSystem::receiver(state.clone());
        tokio::join!(sender, receiver);
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}
impl KeepAliveSystem {
    async fn sender(state: GlobalState) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));
        loop {
            interval.tick().await;

            let keep_alive_data = {
                state
                    .write()
                    .await
                    .world
                    .query_mut::<(Player, KeepAlive, ConnectionWrapper)>()
                    .iter_mut()
                    .map(|(_, (player, keep_alive, conn))| {
                        keep_alive.data += 1;
                        keep_alive.last_sent = std::time::Instant::now();
                        (
                            player.get_username().to_string(),
                            keep_alive.data,
                            conn.0.clone(),
                        )
                    })
                    .collect::<Vec<_>>()
            };

            for (player, data, conn) in keep_alive_data {
                let keep_alive_out = KeepAlivePacketOut::new_auto(data);
                let mut conn = conn.write().await;
                trace!("Sending keep alive packet to player: {:?}", player);
                if let Err(e) = conn.send_packet(keep_alive_out).await {
                    warn!("Error sending keep alive packet: {:?}", e);
                }
            }
        }
    }
    async fn receiver(state: GlobalState) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;

            let to_disconnect = {
                state
                    .write()
                    .await
                    .world
                    .query::<(KeepAlive, ConnectionWrapper)>()
                    .iter()
                    .filter_map(|(_, (keep_alive, conn_wrapper))| {
                        if keep_alive.last_sent.elapsed().as_secs() > 30 {
                            Some(conn_wrapper.0.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            };

            for conn in to_disconnect {
                let conn = conn.read().await;
                let entity = conn.metadata.entity.clone();
                let conn_id = conn.id;
                drop(conn);

                debug!("Dropping connection {} due to inactivity", conn_id);
                if let Err(err) = drop_conn(conn_id, state.clone()).await {
                    warn!("Error dropping connection {}: {:?}", conn_id, err);
                }

                if let Err(err) = state.write().await.world.delete_entity(&entity) {
                    warn!("Error deleting entity: {:?}", err);
                }
            }
        }
    }
}
