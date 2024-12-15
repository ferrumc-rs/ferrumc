use crate::errors::BinaryError;
use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_state::GlobalState;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::{error, info};

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
            let players = state.universe.query::<&mut ChunkReceiver>();
            for (eid, _) in players {
                let state = state.clone();
                task_set.spawn(async move {
                    let mut copied_chunks = {
                        let chunk_recv = state
                            .universe
                            .get_mut::<ChunkReceiver>(eid)
                            .expect("ChunkReceiver not found");
                        let mut copied_chunks = HashMap::new();
                        for chunk in chunk_recv.needed_chunks.iter() {
                            let (key, chunk) = chunk.pair();
                            if chunk.is_none() {
                                copied_chunks.insert(key.clone(), None);
                            }
                        }
                        copied_chunks
                    };
                    for (key, chunk) in copied_chunks.iter_mut() {
                        let fetched_chunk = state
                            .world
                            .load_chunk(key.0, key.1, &key.2.clone())
                            .await
                            .unwrap();
                        *chunk = Some(fetched_chunk);
                    }
                    {
                        let chunk_recv = state
                            .universe
                            .get_mut::<ChunkReceiver>(eid)
                            .expect("ChunkReceiver not found");
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
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
    }

    async fn stop(self: Arc<Self>, _: GlobalState) {
        self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "Chunk Fetcher"
    }
}
