use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_ecs::errors::ECSError;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;
use tracing::{debug, error, info, trace};

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
            let players = state
                .universe
                .query::<(&mut ChunkReceiver, &mut StreamWriter)>();
            let mut task_set: JoinSet<Result<(), ECSError>> = JoinSet::new();
            for (eid, (_, _)) in players {
                let state = state.clone();
                task_set.spawn(async move {
                    let chunk_recv = state
                        .universe
                        .get_mut::<ChunkReceiver>(eid)
                        .expect("ChunkReceiver not found");
                    let mut conn = state
                        .universe
                        .get_mut::<StreamWriter>(eid)
                        .expect("StreamWriter not found");
                    let mut to_drop = Vec::new();
                    if let Some(last_chunk) = &chunk_recv.last_chunk {
                        let packet = SetCenterChunk {
                            x: VarInt::from(last_chunk.0),
                            z: VarInt::from(last_chunk.1),
                        };
                        if let Err(e) = conn.send_packet(&packet, &NetEncodeOpts::WithLength).await
                        {
                            error!("Error sending chunk: {:?}", e);
                        }
                    } else {
                        debug!("No last chunk found");
                    }
                    for possible_chunk in chunk_recv.needed_chunks.iter_mut() {
                        if let Some(chunk) = possible_chunk.pair().1 {
                            let key = possible_chunk.pair().0;
                            to_drop.push(key.clone());
                            match ChunkAndLightData::from_chunk(&chunk.clone()) {
                                Ok(packet) => {
                                    let player = state.universe.get::<PlayerIdentity>(eid).unwrap();
                                    trace!(
                                        "Sending chunk {}, {} to {}",
                                        key.0,
                                        key.1,
                                        player.username
                                    );
                                    if let Err(e) =
                                        conn.send_packet(&packet, &NetEncodeOpts::WithLength).await
                                    {
                                        error!("Error sending chunk: {:?}", e);
                                    }
                                }
                                Err(e) => {
                                    error!("Error sending chunk: {:?}", e);
                                }
                            }
                        }
                    }
                    for key in to_drop {
                        chunk_recv.needed_chunks.remove(&key);
                    }
                    Ok(())
                });
            }
            while let Some(result) = task_set.join_next().await {
                if let Err(e) = result {
                    error!("Error sending chunk: {:?}", e);
                }
            }

            tokio::time::sleep(Duration::from_millis(200)).await;
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
