use crate::net::systems::System;
use crate::net::ConnectionWrapper;
use crate::state::GlobalState;
use crate::utils::components::player::Player;
use crate::utils::encoding::position::Position;
use async_trait::async_trait;
use ferrumc_macros::AutoGenName;
use tracing::{debug, error, info, warn};
use crate::net::packets::outgoing::chunk_and_light_data::ChunkDataAndUpdateLight;

#[derive(AutoGenName)]
pub struct ChunkSender;

#[async_trait]
impl System for ChunkSender {
    async fn run(&self, state: GlobalState) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;

            info!("Sending chunks to players");
            let mut query = state.world.query::<(&Player, &Position, &ConnectionWrapper)>();

            while let Some((_, (player, pos, conn))) = query.next().await {
                info!("Sending chunk to player: {}", player.get_username());
                let packet = ChunkDataAndUpdateLight::new(
                    state.clone(),
                    &*pos,
                ).await;

                let packet = match packet {
                    Ok(packet) => packet,
                    Err(e) => {
                        warn!("Failed to send chunk to player: {}", e);
                        continue;
                    }
                };

                debug!("Trying to get write_guard for connection");
                let mut write_guard = conn.0.write().await;
                debug!("Sending chunk to player: {} with position: {:?}", player.get_username(), *pos);

                if let Err(e) = write_guard.send_packet(packet).await {
                    warn!("Failed to send chunk to player: {}", e);
                    continue;
                };
            }
        }
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}