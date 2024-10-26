use std::sync::Arc;
use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_events::infrastructure::Event;
use ferrumc_net::packets::outgoing::update_time::TickEvent;
use ferrumc_net::GlobalState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::time::Instant;
use tracing::{debug, info};

pub struct TickingSystem;

static KILLED: AtomicBool = AtomicBool::new(false);

#[async_trait]
impl System for TickingSystem {
    async fn start(self: Arc<Self>, state: GlobalState) {
        // TODO game time must be loaded from a file
        let mut tick = 0;
        while !KILLED.load(Ordering::Relaxed) {
            let required_end = Instant::now() + Duration::from_millis(50);
            // TODO handle error
            let res = TickEvent::trigger(TickEvent::new(tick), state.clone()).await;

            if res.is_err() {
                debug!("error: {:?}", res);
            }
            let now = Instant::now();
            if required_end > now {
                tokio::time::sleep(required_end - now).await;
            } else {
                let time_debt = now - required_end;
                info!("Running behind by {:?}", time_debt);
            }

            tick += 200;
        }
    }

    async fn stop(self: Arc<Self>, _state: GlobalState) {
        debug!("Stopping ticking system...");
        KILLED.store(true, Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "keep_alive"
    }
}