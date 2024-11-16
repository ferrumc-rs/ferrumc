use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::{ConnectionControl, ConnectionState, StreamWriter};
use ferrumc_net::packets::incoming::keep_alive::IncomingKeepAlivePacket;
use ferrumc_net::packets::outgoing::disconnect::Disconnect;
use ferrumc_net::packets::outgoing::keep_alive::OutgoingKeepAlivePacket;

use ferrumc_net::utils::broadcast::{BroadcastOptions, BroadcastToAll};
use ferrumc_net::GlobalState;
use ferrumc_net_codec::encode::NetEncodeOpts;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{debug, error, info, trace, warn};

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
        while !self.shutdown.load(Ordering::Relaxed) {
            trace!("starting to check keep alive");


            let online_players = state.universe.query::<&PlayerIdentity>();

            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as i64;

            if current_time - last_time >= 5000 {
                info!("Online players: {}", online_players.count());
                last_time = current_time;
            }


            let entities = state
                .universe
                .query::<(&mut StreamWriter, &ConnectionState)>()
                .into_entities();

            let entities_to_keep_alive = entities
                .iter()
                .filter_map(|entity| {
                    let conn_state = state.universe.get::<ConnectionState>(*entity).ok()?;
                    let keep_alive = state
                        .universe
                        .get_mut::<IncomingKeepAlivePacket>(*entity)
                        .ok()?;

                    if matches!(*conn_state, ConnectionState::Play) {
                        let time_diff = current_time - keep_alive.id;
                        if time_diff >= 30000
                        // the client did not reciprocate the last keep alive 15 seconds ago, therefore it must be kicked see https://wiki.vg/Protocol#Clientbound_Keep_Alive_.28configuration.29
                        {
                            let mut ident =
                                state.universe.get_mut::<PlayerIdentity>(*entity).ok()?;
                            ident.failed_keep_alive = true;
                            None
                        } else if time_diff >= 15000 {
                            Some(*entity)
                        } else {
                            None
                        }

                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            // Kick players with failed keep alive
            let entities_to_kick = entities
                .iter()
                .filter_map(|entity| {
                    let Ok(ident) = state.universe.get::<PlayerIdentity>(*entity) else {
                        warn!(
                            "Failed to get the <PlayerIdentity> Component for entity with id {:?}",
                            *entity
                        );
                        return None;
                    };
                    if ident.failed_keep_alive {
                        Some(*entity)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let kick_packet = Disconnect::from_string("Timeout".to_string());
            for entity in entities_to_kick {
                debug!("Kicking player with entity id {:?} for a timeout", entity);
                let Ok(mut writer) = state.universe.get_mut::<StreamWriter>(entity) else {
                    warn!(
                        "Failed to get the <StreamWriter> Component for entity with id {:?}",
                        entity
                    );
                    continue;
                };
                match writer
                    .send_packet(&kick_packet, &NetEncodeOpts::WithLength)
                    .await
                {
                    Ok(_) => {
                        trace!("kick packet sent for entity {:?} for timeout", entity);
                        if let Ok(control) = state.universe.get_mut::<ConnectionControl>(entity) {
                            control.should_disconnect.store(true, Ordering::Relaxed);
                            debug!("Requested disconnect for entity : {:?}", entity);
                        } else {
                            debug!("failed to get <ConnectionControl> for entity {:?}", entity);
                        }
                    }
                    Err(err) => {
                        warn!(
                            "Failed to kick entity {:?} for timeout\n Error : {:?}",
                            entity, err
                        );
                        continue;
                    }
                }
            }

            if !entities_to_keep_alive.is_empty() {
                trace!(
                    "there are {:?} players to keep alive",
                    entities_to_keep_alive.len()
                );

                let packet = OutgoingKeepAlivePacket { id: current_time };

                let broadcast_opts = BroadcastOptions::default()
                    .only(entities_to_keep_alive)
                    .with_sync_callback(move |entity, state| {
                        let Ok(mut outgoing_keep_alive) =
                            state.universe.get_mut::<OutgoingKeepAlivePacket>(entity)
                        else {
                            warn!(
                                "Failed to get <OutgoingKeepAlive> component for entity {:?}",
                                entity
                            );
                            return;
                        };

                        *outgoing_keep_alive = OutgoingKeepAlivePacket { id: current_time };
                    });

                if let Err(e) = state.broadcast(&packet, broadcast_opts).await {
                    error!("Error sending keep alive packet: {}", e);
                };
            }
            trace!("finished checking keep alives, waiting 15 secs...");
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
