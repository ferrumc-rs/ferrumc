use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::{ConnectionState, StreamWriter};
use ferrumc_net::packets::outgoing::keep_alive::{KeepAlive, KeepAlivePacket};
use ferrumc_net::utils::broadcast::{BroadcastOptions, BroadcastToAll};
use ferrumc_net::GlobalState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{error, info, trace, warn};

pub struct KeepAliveSystem {
    shutdown: AtomicBool,
}

impl KeepAliveSystem {
    pub const fn new() -> Self {
        Self {
            shutdown: AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl System for KeepAliveSystem {
    async fn start(self: Arc<Self>, state: GlobalState) {
        info!("Started keep_alive");
        let mut last_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;
        loop {
            if self.shutdown.load(Ordering::Relaxed) {
                break;
            }

            let online_players = state.universe.query::<&PlayerIdentity>();

            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as i64;

            if current_time - last_time >= 5000 {
                info!("Online players: {}", online_players.count());
                last_time = current_time;
            }

            let fifteen_seconds_ms = 15000; // 15 seconds in milliseconds

            let entities = state
                .universe
                .query::<(&mut StreamWriter, &ConnectionState, &KeepAlive)>()
                .into_entities()
                .into_iter()
                .filter_map(|entity| {
                    let conn_state = state.universe.get::<ConnectionState>(entity).ok()?;
                    let keep_alive = state.universe.get_mut::<KeepAlive>(entity).ok()?;

                    if matches!(*conn_state, ConnectionState::Play)
                        && (current_time - keep_alive.id) >= fifteen_seconds_ms
                    {
                        Some(entity)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if !entities.is_empty() {
                trace!("there are {:?} players to keep alive", entities.len());
            }

            let packet = KeepAlivePacket::default();

            let broadcast_opts = BroadcastOptions::default()
                .only(entities)
                .with_sync_callback(move |entity, state| {
                    let Ok(mut keep_alive) = state.universe.get_mut::<KeepAlive>(entity) else {
                        warn!("Failed to get <KeepAlive> component for entity {}", entity);
                        return;
                    };

                    *keep_alive = KeepAlive::from(current_time);
                });

            if let Err(e) = state.broadcast(&packet, broadcast_opts).await {
                error!("Error sending keep alive packet: {}", e);
            };
            // TODO, this should be configurable as some people may have bad network so the clients may end up disconnecting from the server moments before the keep alive is sent
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    }

    async fn stop(self: Arc<Self>, _state: GlobalState) {
        tracing::debug!("Stopping keep alive system...");
        self.shutdown.store(true, Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "keep_alive"
    }
}
