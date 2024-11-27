use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::GlobalState;
use ferrumc_net_codec::encode::NetEncodeOpts;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, info};

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
            // TODO: This is so ass. Please fix this.
            for (_entity, (player, position, mut conn)) in players {
                debug!("Sending chunks to player: {player:?} @ {position:?}");
                for z in ((position.z.floor() as i32) / 16) - 5..(position.z.ceil() as i32 / 16) + 5 {
                    for x in (position.x.floor() as i32 / 16) - 5..(position.x.ceil() as i32 / 16) + 5 {
                        match state.world.load_chunk(x, z).await {
                            Ok(chunk) => {
                                match ChunkAndLightData::from_chunk(&chunk).await {
                                    Ok(chunk_data) => {
                                        if let Err(e) = conn.send_packet(&chunk_data, &NetEncodeOpts::WithLength).await {
                                            debug!("Could not send chunk to player: {e}");
                                        }
                                    }
                                    Err(e) => {
                                        debug!("Could not convert chunk to chunk and light data: {e}");
                                        if let Err(e) = conn.send_packet(&ChunkAndLightData::empty(x, z).await, &NetEncodeOpts::WithLength).await {
                                            debug!("Could not send empty chunk to player: {e}");
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                debug!("Could not load chunk at {x}, {z}: {e}");
                                if let Err(e) = conn.send_packet(&ChunkAndLightData::empty(x, z).await, &NetEncodeOpts::WithLength).await {
                                    debug!("Could not send empty chunk to player: {e}");
                                }
                            }
                        }
                    }
                }
            }


            tokio::time::sleep(Duration::from_secs(5)).await;
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