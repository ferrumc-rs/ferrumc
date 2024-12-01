use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::GlobalState;
use std::ops::Div;
use std::simd::num::SimdFloat;
use std::simd::{f64x2, StdFloat};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info};

const CHUNK_RADIUS: i32 = 12;

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
            debug!("Sending chunks to players");
            let players = state
                .universe
                .query::<(&PlayerIdentity, &Position, &mut StreamWriter)>();
            // TODO: This is so ass. Please fix this.
            for (_entity, (player, position, mut conn)) in players {
                debug!(
                    "Sending chunks to player: {} @ {}",
                    player.username, position
                );
                // Haha SIMD go brrrrt
                let [chunk_x, chunk_z] = f64x2::from_array([position.x, position.z])
                    .floor()
                    .div(f64x2::from_array([16f64, 16f64]))
                    .cast::<i32>()
                    .to_array();
                if let Err(e) = conn
                    .send_packet(
                        &SetCenterChunk::new(chunk_x, chunk_z),
                        &NetEncodeOpts::WithLength,
                    )
                    .await
                {
                    error!(
                        "Unable to set the center chunk for {} @ {}, {}: {}",
                        &player.username, chunk_x, chunk_z, e
                    );
                    continue;
                }
                let start = std::time::Instant::now();
                let mut chunk_range = (chunk_x - CHUNK_RADIUS..chunk_x + CHUNK_RADIUS)
                    .flat_map(|z| {
                        (chunk_z - CHUNK_RADIUS..chunk_z + CHUNK_RADIUS).map(move |x| (x, z))
                    })
                    .collect::<Vec<_>>();

                chunk_range.sort_by_key(|&(x, z)| {
                    let dx = x - chunk_x;
                    let dz = z - chunk_z;
                    (((dx ^ 2) + (dz ^ 2)) as f64).sqrt() as i32
                });

                match state.world.load_chunk_batch(chunk_range).await {
                    Ok(chunks) => {
                        for chunk in chunks {
                            match ChunkAndLightData::from_chunk(&chunk) {
                                Ok(data) => {
                                    if let Err(e) =
                                        conn.send_packet(&data, &NetEncodeOpts::WithLength).await
                                    {
                                        error!(
                                            "Unable to send chunk data to {} @ {}, {}: {}",
                                            &player.username, chunk.x, chunk.z, e
                                        );
                                        if let Err(e) = conn
                                            .send_packet(
                                                &ChunkAndLightData::empty(chunk.x, chunk.z),
                                                &NetEncodeOpts::WithLength,
                                            )
                                            .await
                                        {
                                            error!(
                                                "Unable to send empty chunk data to {} @ {}, {}: {}",
                                                &player.username, chunk.x, chunk.z, e
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!(
                                        "Unable to convert chunk to chunk and light data for {} @ {}, {}: {}",
                                        &player.username, chunk.x, chunk.z, e
                                    );
                                    if let Err(e) = conn
                                        .send_packet(
                                            &ChunkAndLightData::empty(chunk.x, chunk.z),
                                            &NetEncodeOpts::WithLength,
                                        )
                                        .await
                                    {
                                        error!(
                                            "Unable to send empty chunk data to {} @ {}, {}: {}",
                                            &player.username, chunk.x, chunk.z, e
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "Unable to load chunks for {} @ {}, {}: {}",
                            &player.username, chunk_x, chunk_z, e
                        );
                    }
                }

                debug!(
                    "Sent {} chunks to player: {} @ {:.2},{:.2} in {:?}",
                    (CHUNK_RADIUS * 2) * 2,
                    player.username,
                    position.x,
                    position.z,
                    start.elapsed()
                );
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
