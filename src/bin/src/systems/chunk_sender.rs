use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_ecs::errors::ECSError;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::chunk_batch_finish::ChunkBatchFinish;
use ferrumc_net::packets::outgoing::chunk_batch_start::ChunkBatchStart;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;
use std::io::Cursor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;
use tracing::{error, info, trace};

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
                    let mut packets = Vec::new();
                    let mut centre_coords = (0, 0);
                    trace!("Getting chunk_recv 1 for sender");
                    {
                        let Ok(chunk_recv) = state.universe.get::<ChunkReceiver>(eid) else {
                            trace!("A player disconnected before we could get the ChunkReceiver");
                            return Ok(());
                        };
                        trace!("Got chunk_recv 1 for sender");
                        if chunk_recv.needed_chunks.is_empty() {
                            return Ok(());
                        }
                    }
                    // We can't delete from the map while iterating, so we collect the keys to drop
                    // and then drop them after sending the chunks
                    let mut to_drop = Vec::new();
                    {
                        trace!("Getting chunk_recv 2 for sender");
                        let Ok(chunk_recv) = state.universe.get::<ChunkReceiver>(eid) else {
                            trace!("A player disconnected before we could get the ChunkReceiver");
                            return Ok(());
                        };
                        trace!("Got chunk_recv 2 for sender");
                        {
                            trace!("Getting conn 1 for sender");
                            trace!("Got conn 1 for sender");
                            if let Some(chunk) = &chunk_recv.last_chunk {
                                centre_coords = (chunk.0, chunk.1);
                            }
                        }
                    }
                    let mut sent_chunks = 0;
                    trace!("Getting chunk_recv 3 for sender");
                    {
                        let chunk_recv = state
                            .universe
                            .get_mut::<ChunkReceiver>(eid)
                            .expect("ChunkReceiver not found");
                        trace!("Got chunk_recv 3 for sender");
                        for possible_chunk in chunk_recv.needed_chunks.iter_mut() {
                            if let Some(chunk) = possible_chunk.pair().1 {
                                let key = possible_chunk.pair().0;
                                to_drop.push(key.clone());
                                match ChunkAndLightData::from_chunk(&chunk.clone()) {
                                    Ok(packet) => {
                                        packets.push(packet);
                                        sent_chunks += 1;
                                    }
                                    Err(e) => {
                                        error!("Error sending chunk: {:?}", e);
                                    }
                                }
                            }
                        }
                    }
                    {
                        trace!("Getting chunk_recv 4 for sender");
                        let chunk_recv = state
                            .universe
                            .get_mut::<ChunkReceiver>(eid)
                            .expect("ChunkReceiver not found");
                        trace!("Got chunk_recv 4 for sender");
                        for key in to_drop {
                            chunk_recv.needed_chunks.remove(&key);
                        }
                    }

                    {
                        if packets.is_empty() {
                            return Ok(());
                        }
                        trace!("Getting conn 2 for sender");
                        let Ok(mut conn) = state.universe.get_mut::<StreamWriter>(eid) else {
                            error!("Could not get StreamWriter");
                            return Ok(());
                        };
                        trace!("Got conn 2 for sender");
                        if let Err(e) = conn
                            .send_packet(
                                &SetCenterChunk {
                                    x: VarInt::new(centre_coords.0),
                                    z: VarInt::new(centre_coords.1),
                                },
                                &NetEncodeOpts::WithLength,
                            )
                            .await
                        {
                            error!("Error sending chunk: {:?}", e);
                        }
                        if let Err(e) = conn
                            .send_packet(&ChunkBatchStart {}, &NetEncodeOpts::WithLength)
                            .await
                        {
                            error!("Error sending chunk: {:?}", e);
                        }
                        for packet in packets {
                            if let Err(e) =
                                conn.send_packet(&packet, &NetEncodeOpts::WithLength).await
                            {
                                error!("Error sending chunk: {:?}", e);
                            }
                        }
                        if let Err(e) = conn
                            .send_packet(
                                &ChunkBatchFinish {
                                    batch_size: VarInt::new(sent_chunks),
                                },
                                &NetEncodeOpts::WithLength,
                            )
                            .await
                        {
                            error!("Error sending chunk: {:?}", e);
                        }
                    }

                    Ok(())
                });
            }
            while let Some(result) = task_set.join_next().await {
                match result {
                    Ok(task_res) => {
                        if let Err(e) = task_res {
                            error!("Error sending chunk: {:?}", e);
                        }
                    }
                    Err(e) => {
                        error!("Error sending chunk: {:?}", e);
                    }
                }
            }

            // tokio::time::sleep(Duration::from_nanos(50)).await;
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
