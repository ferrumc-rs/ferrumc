use crate::errors::BinaryError;
use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_state::GlobalState;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::{error, info, trace};

pub struct ChunkFetcher {
    stop: AtomicBool,
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
                            let (key, chunk) = chunk.pair();
                            if chunk.is_none() {
                                copied_chunks.insert(key.clone(), None);
                            }
                        }
                        copied_chunks
                    };
                    // Fetch the chunks
                    for (key, chunk) in copied_chunks.iter_mut() {
                        let fetched_chunk =
                            state.world.load_chunk(key.0, key.1, &key.2.clone()).await?;
                        *chunk = Some(fetched_chunk);
                    }
                    // Insert the fetched chunks back into the component
                    {
                        let Ok(chunk_recv) = state.universe.get::<ChunkReceiver>(eid) else {
                            trace!("A player disconnected before we could get the ChunkReceiver");
                            return Ok(());
                        };
                        for (key, chunk) in copied_chunks.iter() {
                            chunk_recv.needed_chunks.insert(key.clone(), chunk.clone());
                        }
                    }
                    Ok(())
                });
            }
            while let Some(result) = task_set.join_next().await {
                match result {
                    Ok(task_res) => {
                        if let Err(e) = task_res {
                            error!("Error fetching chunk: {:?}", e);
                        }
                    }
                    Err(e) => {
                        error!("Error fetching chunk: {:?}", e);
                    }
                }
            }
            // tokio::time::sleep(std::time::Duration::from_nanos(50)).await;
        }
    }

    async fn stop(self: Arc<Self>, _: GlobalState) {
        self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "Chunk Fetcher"
    }
}
