use std::sync::Arc;
use crate::net::systems::System;
use crate::net::{Connection, ConnectionWrapper};
use crate::state::GlobalState;
use crate::utils::components::player::Player;
use crate::utils::encoding::position::Position;
use async_trait::async_trait;
use ferrumc_macros::AutoGenName;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use crate::ecs::component::ComponentRef;
use crate::net::packets::outgoing::chunk_and_light_data::ChunkDataAndUpdateLight;
use crate::net::packets::outgoing::set_center_chunk::SetCenterChunk;
use crate::utils::prelude::*;

#[derive(AutoGenName)]
pub struct ChunkSender;

#[async_trait]
impl System for ChunkSender {
    async fn run(&self, state: GlobalState) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;

            info!("Sending chunks to players");
            let mut query = state.world.query::<(&Player, &Position, &ConnectionWrapper)>();

            while let Some((_, (player, pos, conn))) = query.next().await {
                info!("Sending chunk to player: {}", player.get_username());
                if let Err(e) = send_set_center_chunk(&*player, &*pos, conn.0.clone()).await {
                    error!("Failed to send center chunk to player: {}", e);
                    continue;
                }
                if let Err(e) = send_chunks_to_player(state.clone(), &*player, &*pos, conn.0.clone()).await {
                    error!("Failed to send chunk to player: {}", e);
                    continue;
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}

async fn send_chunks_to_player(state: GlobalState, player: &Player, pos: &Position, conn: Arc<RwLock<Connection>>) -> Result<()> {
    let mut write_guard = conn.write().await;

    const CHUNK_RADIUS: i32 = 16;

    for x in -CHUNK_RADIUS..CHUNK_RADIUS {
        for z in -CHUNK_RADIUS..CHUNK_RADIUS {
            let packet = ChunkDataAndUpdateLight::new(
                state.clone(),
                (pos.x >> 4) + x,
                (pos.z >> 4) + z
            ).await?;

            if let Err(e) = write_guard.send_packet(packet).await {
                warn!("Failed to send chunk to player: {}", e);
            };

        }
    }

    Ok(())
}
async fn send_set_center_chunk(player: &Player, pos: &Position, conn: Arc<RwLock<Connection>>) -> Result<()> {
    let packet = SetCenterChunk::new(pos.x >> 4, pos.z >> 4);

    let mut write_guard = conn.write().await;

    debug!("Sending center chunk to player: {} with position: {:?}", player.get_username(), pos);

    if let Err(e) = write_guard.send_packet(packet).await {
        warn!("Failed to send chunk to player: {}", e);
    };

    Ok(())
}