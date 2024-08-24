use async_trait::async_trait;
use tokio::sync::RwLockReadGuard;
use tracing::{trace, warn};

use ferrumc_macros::AutoGenName;

use crate::net::packets::outgoing::keep_alive::KeepAlivePacketOut;
use crate::net::systems::System;
use crate::net::{Connection, ConnectionWrapper};
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
        let mut query = state
            .world
            .query::<(&Player, &mut KeepAlive, &ConnectionWrapper)>();

        loop {
            interval.tick().await;

            while let Some((_, (player, mut keep_alive, conn))) = query.next().await {
                if keep_alive.last_sent.elapsed().as_secs() > 30 {
                    let conn = conn.0.read().await;
                    warn!("Dropping connection {} due to inactivity", conn.id);
                    if let Err(err) = conn.drop_connection(state.clone()).await {
                        warn!(
                            "Error dropping connection {:?}: {:?}",
                            conn.player_uuid, err
                        );
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
    async fn receiver(state: GlobalState) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        let mut query = state.world.query::<(&KeepAlive, &ConnectionWrapper)>();

        loop {
            interval.tick().await;

            while let Some((_, (keep_alive, conn_wrapper))) = query.next().await {
                if keep_alive.last_sent.elapsed().as_secs() <= 30 {
                    continue;
                }

                let conn = conn_wrapper.0.read().await;
                let player = state
                    .world
                    .get_component::<Player>(conn.id)
                    .await;

                let username = player
                    .as_ref()
                    .map(|p| p.username.as_str())
                    .unwrap_or("Unknown<!>Player");

                Self::drop_connection(conn, username, state.clone()).await;
            }
        }
    }

    async fn drop_connection(
        conn: RwLockReadGuard<'_, Connection>,
        username: &str,
        state: GlobalState,
    ) {
        warn!(
            "Dropping player `{}`'s connection due to inactivity",
            username
        );
        if let Err(err) = conn.drop_connection(state).await {
            warn!(
                "Error dropping connection {:?}: {:?}",
                conn.player_uuid, err
            );
        }
    }
}
