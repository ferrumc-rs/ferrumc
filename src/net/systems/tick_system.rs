use tracing::debug;

use async_trait::async_trait;
use ferrumc_macros::AutoGenName;

use crate::net::systems::System;

#[derive(AutoGenName)]
pub struct TickSystem;

#[async_trait]
impl System for TickSystem {
    async fn run(&self) {
        loop {
            debug!("Tick");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}