use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_ecs::errors::ECSError;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::GlobalState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info};

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
                .query::<(&mut ChunkReceiver, &mut StreamWriter)>();
            let mut task_set: tokio::task::JoinSet<Result<(), ECSError>> =
                tokio::task::JoinSet::new();
            for (eid, (_, _)) in players {
                let state = state.clone();
                task_set.spawn(async move {
                    let chunk_recv = state.universe.get_mut::<&mut ChunkReceiver>(eid)?;
                    for possible_chunk in chunk_recv.needed_chunks.iter_mut() {
                        let (key, possible_chunk) = possible_chunk.pair();
                        let _ = chunk_recv.needed_chunks.remove(key);
                        if let Some(chunk) = possible_chunk {
                            let packet = &ChunkAndLightData::from_chunk(chunk);
                            match packet {
                                Ok(packet) => {
                                    let mut conn =
                                        state.universe.get_mut::<&mut StreamWriter>(eid)?;
                                    if let Err(e) =
                                        conn.send_packet(packet, &NetEncodeOpts::WithLength).await
                                    {
                                        error!("Error sending chunk: {:?}", e);
                                    }
                                    return Ok(());
                                }
                                Err(e) => {
                                    error!("Error sending chunk: {:?}", e);
                                }
                            }
                        }
                    }
                    Ok(())
                });
            }
            while let Some(result) = task_set.join_next().await {
                if let Err(e) = result {
                    error!("Error sending chunk: {:?}", e);
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
