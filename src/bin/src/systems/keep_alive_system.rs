use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use async_trait::async_trait;
use tracing::info;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::GlobalState;
use crate::systems::definition::System;

pub struct KeepAliveSystem {
    shutdown: AtomicBool
}

impl KeepAliveSystem {
    pub const fn new() -> Self {
        Self {
            shutdown: AtomicBool::new(false)
        }
    }
}

#[async_trait]
impl System for KeepAliveSystem {
    async fn start(self: Arc<Self>, state: GlobalState) {
        loop {
            if self.shutdown.load(Ordering::Relaxed) {
                break;
            }
            
            let online_players = state.universe.query::<&PlayerIdentity>();
            info!("Online players: {}", online_players.count());

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    async fn stop(self: Arc<Self>, _state: GlobalState) {
        tracing::debug!("Stopping keep alive system...");
        self.shutdown.store(true, Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "keep_alive"
    }
}

