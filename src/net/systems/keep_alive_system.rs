use async_trait::async_trait;

use ferrumc_macros::AutoGenName;
use tracing::{trace, warn};
use crate::net::{ConnectionWrapper, GET_WORLD};
use crate::net::packets::outgoing::keep_alive::KeepAlivePacketOut;
use crate::net::systems::System;
use crate::utils::components::keep_alive::KeepAlive;
use crate::utils::components::player::Player;

#[derive(AutoGenName)]
pub struct KeepAliveSystem;

#[async_trait]
impl System for KeepAliveSystem {
    async fn run(&self) {
        let sender = KeepAliveSystem::sender();
        let receiver = KeepAliveSystem::receiver();
        tokio::join!(sender, receiver);
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}
impl KeepAliveSystem {
    async fn sender() {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));
        let world = GET_WORLD();
        let mut query = world.query::<(&Player, &mut KeepAlive, &ConnectionWrapper)>();

        loop {
            interval.tick().await;

            while let Some((_, (player, mut keep_alive, conn))) = query.next().await {
                if keep_alive.last_sent.elapsed().as_secs() > 30 {
                    let conn = conn.0.read().await;
                    warn!("Dropping connection {} due to inactivity", conn.id);
                    if let Err(err) = conn.drop_connection().await {
                        warn!("Error dropping connection {:?}: {:?}", conn.player_uuid, err);
                    }
                    continue;
                }

                keep_alive.data += 1;
                keep_alive.last_sent = std::time::Instant::now();

                let keep_alive_out = KeepAlivePacketOut::new_auto(keep_alive.data);
                let mut conn = conn.0.write().await;

                trace!("Sending keep alive packet to player: {:?}", player);
                if let Err(e) = conn.send_packet(keep_alive_out).await {
                    warn!("Error sending keep alive packet: {:?}", e);
                }
            }
        }
    }
    async fn receiver() {

    }
    /*async fn sender() {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));
        loop {
            interval.tick().await;

            let keep_alive_data = {
                let mut world = GET_WORLD().write().await;
                world
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
    async fn receiver() {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;

            let to_disconnect = {
                let world = GET_WORLD().read().await;
                world
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
                if let Err(err) = drop_conn(conn_id).await {
                    warn!("Error dropping connection {}: {:?}", conn_id, err);
                }

                let mut world = GET_WORLD().write().await;
                if let Err(err) = world.delete_entity(&entity) {
                    warn!("Error deleting entity: {:?}", err);
                }
            }
        }
    }*/
}
