use crate::errors::BinaryError;
use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_state::GlobalState;
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
            let mut taskset: JoinSet<Result<(), BinaryError>> = JoinSet::new();
            let players = state
                .universe
                .query::<(&PlayerIdentity, &mut ChunkReceiver)>();
            for (_eid, (_, chunk_recv)) in players {
                let state = state.clone();
                //taskset.spawn(async move {
                for mut chunks in chunk_recv.needed_chunks.iter_mut() {
                    let (key, chunk) = chunks.pair_mut();
                    if chunk.is_none() {
                        trace!("Fetching chunk: {:?}", key);
                        let fetched_chunk = state
                            .world
                            .load_chunk(key.0, key.1, &key.2.clone())
                            .await
                            .unwrap();
                        *chunk = Some(fetched_chunk);
                    }
                }
                // Ok(())
                //});
            }
            while let Some(result) = taskset.join_next().await {
                if let Err(e) = result {
                    error!("Error fetching chunk: {:?}", e);
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
