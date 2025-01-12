use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::{ConnectionState, StreamWriter};
use ferrumc_net::packets::incoming::keep_alive::IncomingKeepAlivePacket;
use ferrumc_net::packets::outgoing::keep_alive::OutgoingKeepAlivePacket;
use ferrumc_net::utils::broadcast::{BroadcastOptions, BroadcastToAll};
use ferrumc_net::utils::state::terminate_connection;
use ferrumc_state::GlobalState;
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
        loop {
            if self.shutdown.load(Ordering::Relaxed) {
                break;
            }

            // Get the times before the queries, since it's possible a query takes more than a millisecond with a lot of entities.

            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as i64;

            let online_players = state
                .universe
                .query::<&PlayerIdentity>()
                .await
                .into_entities();
            info!("Online players: {}", online_players.len());

            let mut entities = Vec::new();
            let query_result = state
                .universe
                .query::<(&mut StreamWriter, &ConnectionState)>()
                .await
                .into_entities();

            for entity in query_result {
                if let Ok(conn_state) = state.universe.get::<ConnectionState>(entity).await {
                    if let Ok(keep_alive) = state
                        .universe
                        .get_mut::<IncomingKeepAlivePacket>(entity)
                        .await
                    {
                        if matches!(*conn_state, ConnectionState::Play)
                            && (current_time - keep_alive.timestamp) >= 15000
                        {
                            entities.push(entity);
                        }
                    }
                }
            }
            if !entities.is_empty() {
                trace!("there are {:?} players to keep alive", entities.len());

                // I know this is the second iteration of the entities vector, but it has to be done since terminate_connection is async
                for entity in entities.iter() {
                    let keep_alive = state
                        .universe
                        .get_mut::<IncomingKeepAlivePacket>(*entity)
                        .await
                        .ok()
                        .unwrap();

                    if (current_time - keep_alive.timestamp) >= 30000 {
                        // two iterations missed
                        if let Err(e) = terminate_connection(
                            state.clone(),
                            *entity,
                            "Keep alive timeout".to_string(),
                        )
                        .await
                        {
                            warn!(
                                "Failed to terminate connection for entity {:?} , Err : {:?}",
                                entity, e
                            );
                        }
                    }
                }
                let packet = OutgoingKeepAlivePacket {
                    timestamp: current_time,
                };

                let broadcast_opts = BroadcastOptions::default()
                    .only(entities)
                    .with_async_callback(move |entity, state| {
                        let state = state.clone();
                        let packet = packet.clone();
                        async move {
                            let Ok(mut keep_alive) = state
                                .universe
                                .get_mut::<OutgoingKeepAlivePacket>(entity)
                                .await
                            else {
                                warn!(
                                    "Failed to get <OutgoingKeepAlive> component for entity {}",
                                    entity
                                );
                                return;
                            };

                            *keep_alive = packet.clone();
                        }
                    });

                if let Err(e) = state
                    .broadcast(
                        &OutgoingKeepAlivePacket {
                            timestamp: current_time,
                        },
                        broadcast_opts,
                    )
                    .await
                {
                    error!("Error sending keep alive packet: {}", e);
                };
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
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
