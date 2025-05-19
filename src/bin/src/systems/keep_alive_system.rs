use crate::errors::BinaryError;
use crate::systems::definition::System;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::incoming::keep_alive::IncomingKeepAlivePacket;
use ferrumc_net::packets::outgoing::keep_alive::OutgoingKeepAlivePacket;
use ferrumc_state::GlobalState;
use std::sync::Arc;
use tracing::{error, trace, warn};

pub struct KeepAliveSystem;

impl KeepAliveSystem {
    pub const fn new() -> Self {
        Self
    }
}

impl System for KeepAliveSystem {
    fn run(self: Arc<Self>, state: GlobalState, _tick: u128) -> Result<(), BinaryError> {
        // Get the times before the queries, since it's possible a query takes more than a millisecond with a lot of entities.

        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;

        let entities = state
            .universe
            .query::<&mut StreamWriter>()
            .into_entities()
            .into_iter()
            .filter_map(|entity| {
                let keep_alive = state
                    .universe
                    .get_mut::<IncomingKeepAlivePacket>(entity)
                    .ok()?;

                if current_time - keep_alive.timestamp >= 15000 {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if !entities.is_empty() {
            trace!("there are {:?} players to keep alive", entities.len());

            // I know this is the second iteration of the entities vector, but it has to be done since terminate_connection is async
            for entity in entities.iter() {
                let keep_alive = state
                    .universe
                    .get_mut::<IncomingKeepAlivePacket>(*entity)
                    .ok()
                    .unwrap();

                if (current_time - keep_alive.timestamp) >= 30000 {
                    // two iterations missed
                    if let Err(e) = terminate_connection(
                        state.clone(),
                        *entity,
                        "Keep alive timeout".to_string(),
                    ) {
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

            let broadcast_opts =
                BroadcastOptions::default()
                    .only(entities)
                    .with_callback(move |entity, state| {
                        let Ok(mut keep_alive) =
                            state.universe.get_mut::<OutgoingKeepAlivePacket>(entity)
                        else {
                            warn!(
                                "Failed to get <OutgoingKeepAlive> component for entity {}",
                                entity
                            );
                            return;
                        };

                        *keep_alive = packet.clone();
                    });

            if let Err(e) = state.broadcast(
                &OutgoingKeepAlivePacket {
                    timestamp: current_time,
                },
                broadcast_opts,
            ) {
                error!("Error sending keep alive packet: {}", e);
            };
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "keep_alive"
    }
}
