use async_trait::async_trait;
use ferrumc_macros::AutoGenName;
use tracing::info;

use crate::net::systems::System;
use crate::state::GlobalState;
use crate::utils::components::player::Player;
use crate::utils::components::rotation::Rotation;
use crate::utils::encoding::position::Position;

#[derive(AutoGenName)]
pub struct TickSystem;

#[async_trait]
impl System for TickSystem {
    async fn run(&self, state: GlobalState) {
        let mut query = state.world.query::<(&Player, &Position, &Rotation)>();

        loop {
            while let Some((idx, (player, position, rotation))) = query.next().await {
                info!(
                    "[{idx}] @ Player = {:?}\tPosition = {}\tRotation = {:?}",
                    player.username, *position, *rotation,
                );
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}
