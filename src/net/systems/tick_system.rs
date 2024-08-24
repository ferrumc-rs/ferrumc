use async_trait::async_trait;
use ferrumc_macros::AutoGenName;
use tracing::info;
use crate::net::ConnectionWrapper;
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
        let mut query = state.world.query::<(&ConnectionWrapper)>();

        loop {
            /*while let Some((idx, (player, position, rotation))) = query.next().await {
                info!(
                    "[{idx}] @ Player = {:?}\tPosition = {}\tRotation = {:?}",
                    player.username, *position, *rotation,
                );
            }*/

            while let Some((id, (conn))) = query.next().await {
                let conn_locked = conn.0.try_write().is_err();
                info!(
                    "[{id}] @ Connection = {:?}",
                    conn_locked,
                );
            }


            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}
