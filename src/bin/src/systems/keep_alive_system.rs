use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use async_trait::async_trait;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::GlobalState;
use crate::systems::definition::System;

pub struct KeepAliveSystem;

static KILLED: AtomicBool = AtomicBool::new(false);

#[async_trait]
impl System for KeepAliveSystem {
    async fn start(&self, state: GlobalState) {
        loop {
            if KILLED.load(Ordering::Relaxed) {
                break;
            }

            let online_players = state.universe.query::<&PlayerIdentity>();
            drop(online_players);
            // tracing::debug!("Total of {} online players", online_players.count());

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    async fn stop(&self, _state: GlobalState) {
        tracing::debug!("Stopping keep alive system...");
        KILLED.store(true, Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "keep_alive"
    }
}
