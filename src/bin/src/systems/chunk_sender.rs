use std::io::Cursor;
use crate::systems::definition::System;
use ferrumc_net::GlobalState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use tracing::{debug, info};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::{ConnectionState, StreamWriter};
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;

pub(super) struct ChunkSenderSystem {
    pub stop: AtomicBool,
}

impl ChunkSenderSystem {
    pub const fn new() -> Self {
        Self {
            stop: AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl System for ChunkSenderSystem {
    async fn start(self: Arc<Self>, state: GlobalState) {
        info!("Chunk sender system started");

        while !self.stop.load(Ordering::Relaxed) {
            let players = state.universe.query::<(&PlayerIdentity, &Position, &mut StreamWriter)>();

            for (player, position, conn) in players {
                debug!("Sending chunks to player: {player:?} @ {position:?}");
                for z in position.z.floor() as i32 - 5..position.z.ceil() as i32 + 5 {
                    for x in position.x.floor() as i32 - 5..position.x.ceil() as i32 + 5 {
                        match state.world.load_chunk(x, z).await {
                            Ok(chunk) => {
                                todo!("Send chunk to player");
                            }
                            Err(e) => {
                                debug!("Could not load chunk at {x}, {z}: {e}");
                            }
                        }
                }
            }
            }
            
            
            
            
            
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    async fn stop(self: Arc<Self>, _state: GlobalState) {
        info!("Stopping chunk sender system");
        self.stop.store(true, Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "chunk_sender"
    }
}