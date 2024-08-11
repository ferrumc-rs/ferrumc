use async_trait::async_trait;
use ferrumc_macros::AutoGenName;
use tracing::{info};

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
            while let Some((idx, (player, position))) = query.next().await {
                info!(
                    "[{idx}] @ Player = {} \t Position = {}",
                    player.get_username(),
                    *position
                );
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}
