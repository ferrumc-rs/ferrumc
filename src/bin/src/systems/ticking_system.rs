use crate::systems::definition::System;
use async_trait::async_trait;
use ferrumc_events::infrastructure::Event;
use ferrumc_net::packets::outgoing::update_time::TickEvent;
use ferrumc_state::GlobalState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use tracing::{debug, info, trace};
pub struct TickingSystem;

static KILLED: AtomicBool = AtomicBool::new(false);

#[async_trait]
impl System for TickingSystem {
    async fn start(self: Arc<Self>, state: GlobalState) {
        // TODO game time must be loaded from a file
        let mut tick = 0;
        while !KILLED.load(Ordering::Relaxed) {
            let required_end = Instant::now() + Duration::from_millis(50);
            let res = {
                let start = Instant::now();
                let res = TickEvent::trigger(TickEvent::new(tick), state.clone()).await;
                trace!("Tick took {:?}", Instant::now() - start);

                res
            };
            if res.is_err() {
                debug!("error handling tick event: {:?}", res);
            }
            let now = Instant::now();

            if required_end > now {
                tokio::time::sleep(required_end - now).await;
            } else {
                let time_debt = now - required_end;
                info!("Running behind by {:?}", time_debt);
            }

            tick += 1;
        }
    }

    async fn stop(self: Arc<Self>, _state: GlobalState) {
        debug!("Stopping ticking system...");
        KILLED.store(true, Ordering::Relaxed);
    }

    fn name(&self) -> &'static str {
        "ticking"
    }
}
