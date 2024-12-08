use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_state::GlobalState;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tracing::{debug, info};

pub struct ChunkFetcher {
    stop: AtomicBool,
}

#[async_trait]
impl System for ChunkFetcher {
    async fn start(self: Arc<Self>, state: GlobalState) {
        info!("Chunk fetcher system started");

        while !self.stop.load(std::sync::atomic::Ordering::Relaxed) {}
    }

    async fn stop(self: Arc<Self>, state: GlobalState) {
        self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "Chunk Fetcher"
    }
}
