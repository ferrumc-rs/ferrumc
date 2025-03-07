use crate::errors::BinaryError;
use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::chunks::chunk_receiver::ChunkSendState::{Fetching, Sending};
use ferrumc_state::GlobalState;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::vanilla_chunk_format::BlockData;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::{debug, info, trace};

pub struct ChunkFetcher {
    stop: AtomicBool,
}

fn generate_chunk(x: i32, z: i32) -> Chunk {
    let mut new_chunk = Chunk::new(x, z, "overworld".to_string());
    for x in 0..16 {
        for z in 0..16 {
            let mut base_y = (x as f64).sin() / 32.0;
            base_y += (z as f64).cos() / 32.0;
            let scaled_y = ((base_y + 1.0) * 64.0) as i32;
            for y in 0..scaled_y {
                new_chunk
                    .set_block(
                        x,
                        y,
                        z,
                        BlockData {
                            name: "minecraft:stone".to_string(),
                            properties: None,
                        },
                    )
                    .unwrap();
            }
        }
    }
    new_chunk
}

impl ChunkFetcher {
    pub(crate) fn new() -> Self {
        Self {
            stop: AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl System for ChunkFetcher {
    async fn start(self: Arc<Self>, state: GlobalState) {
        info!("Chunk fetcher system started");

        while !self.stop.load(std::sync::atomic::Ordering::Relaxed) {
            let mut task_set: JoinSet<Result<(), BinaryError>> = JoinSet::new();
            let players = state.universe.query::<&mut ChunkReceiver>().into_entities();
            for eid in players {
                let state = state.clone();
                task_set.spawn(async move {
                    // Copy the chunks into a new map so we don't lock the component while fetching
                    let mut copied_chunks = {
                        let Ok(chunk_recv) = state.universe.get::<ChunkReceiver>(eid) else {
                            trace!("A player disconnected before we could get the ChunkReceiver");
                            return Ok(());
                        };
                        let mut copied_chunks = HashMap::new();
                        for chunk in chunk_recv.needed_chunks.iter() {
                            let (key, chunk) = chunk;
                            if let Fetching = chunk {
                                copied_chunks.insert(key.clone(), None);
                            }
                        }
                        copied_chunks
                    };
                    // Fetch the chunks
                    for (key, chunk) in copied_chunks.iter_mut() {
                        let fetched_chunk = if state
                            .world
                            .chunk_exists(key.0, key.1, &key.2.clone())
                            .await?
                        {
                            debug!("Chunk found, loading chunk");
                            state.world.load_chunk(key.0, key.1, &key.2.clone()).await?
                        } else {
                            debug!("Chunk not found, creating new chunk");
                            let new_chunk = generate_chunk(key.0, key.1);

                            state.world.save_chunk(new_chunk.clone()).await?;
                            new_chunk
                        };
                        *chunk = Some(fetched_chunk);
                    }
                    state.world.sync().await?;
                    // Insert the fetched chunks back into the component
                    {
                        let Ok(mut chunk_recv) = state.universe.get_mut::<ChunkReceiver>(eid)
                        else {
                            trace!("A player disconnected before we could get the ChunkReceiver");
                            return Ok(());
                        };
                        for (key, chunk) in copied_chunks {
                            if let Some(chunk) = chunk {
                                chunk_recv.needed_chunks.insert(key.clone(), Sending(chunk));
                            }
                        }
                    }
                    Ok(())
                });
            }
            while let Some(result) = task_set.join_next().await {
                match result {
                    Ok(task_res) => {
                        if let Err(e) = task_res {
                            debug!("Error fetching chunk: {:?}", e);
                        }
                    }
                    Err(e) => {
                        debug!("Error fetching chunk: {:?}", e);
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(45)).await;
        }
    }

    async fn stop(self: Arc<Self>, _: GlobalState) {
        self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "Chunk Fetcher"
    }
}
