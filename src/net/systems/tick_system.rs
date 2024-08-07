use async_trait::async_trait;
use tracing::{debug, info};

use ferrumc_macros::AutoGenName;

use crate::net::systems::System;
use crate::state::GlobalState;
use crate::utils::components::player::Player;
use crate::utils::encoding::position::Position;

#[derive(AutoGenName)]
pub struct TickSystem;

#[async_trait]
impl System for TickSystem {
    async fn run(&self, state: GlobalState) {
        let mut query = state.world.query::<(&Player, &Position)>();

        loop {
            let tick_start = std::time::Instant::now();

            while let Some((idx, (player, position))) = query.next().await {
                info!(
                    "[{idx}] @ Player = {} \t Position = {}",
                    player.get_username(),
                    *position
                );
            }

            debug!("Time taken to run tick system: {:?}", tick_start.elapsed());
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}
