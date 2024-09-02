use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{debug, error, warn};

use ferrumc_macros::AutoGenName;

use crate::net::packets::outgoing::chunk_and_light_data::ChunkDataAndUpdateLight;
use crate::net::packets::outgoing::set_center_chunk::SetCenterChunk;
use crate::net::systems::System;
use crate::net::{Connection, ConnectionWrapper};
use crate::state::GlobalState;
use crate::utils::components::player::Player;
use crate::utils::encoding::position::Position;
use crate::utils::prelude::*;

#[derive(AutoGenName)]
pub struct ChunkSender;

#[async_trait]
impl System for ChunkSender {
    async fn run(&self, state: GlobalState) {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(750));
        loop {
            interval.tick().await;

            let mut query = state.world.query::<&Player>();

            while let Some((entity_id, player)) = query.next().await {
                debug!("Sending chunk to player: {}", player.get_username());
                drop(player);
                if let Err(e) = ChunkSender::send_chunks_to_player(state.clone(), entity_id).await {
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

impl ChunkSender {
    pub async fn send_chunks_to_player(
        state: GlobalState,
        entity_id: impl TryInto<usize>,
    ) -> Result<()> {
        let entity_id = entity_id.try_into().map_err(|_| Error::ConversionError)?;

        let (player, pos, conn) = state
            .world
            .get_components::<(Player, Position, ConnectionWrapper)>(entity_id)
            .await?;

        debug!(
            "Sending chunks to player: {} @ {:?}",
            player.get_username(),
            *pos
        );

        ChunkSender::send_set_center_chunk(&*pos, conn.0.clone()).await?;
        ChunkSender::send_chunk_data_to_player(state.clone(), &*pos, conn.0.clone()).await?;

        Ok(())
    }

    async fn send_chunk_data_to_player(
        state: GlobalState,
        pos: &Position,
        conn: Arc<RwLock<Connection>>,
    ) -> Result<()> {
        let mut write_guard = conn.write().await;

        const CHUNK_RADIUS: i32 = 32;

        for x in -CHUNK_RADIUS..=CHUNK_RADIUS {
            for z in -CHUNK_RADIUS..=CHUNK_RADIUS {
                let packet =
                    ChunkDataAndUpdateLight::new(state.clone(), (pos.x >> 4) + x, (pos.z >> 4) + z)
                        .await?;

                if let Err(e) = write_guard.send_packet(packet).await {
                    warn!("Failed to send chunk to player: {}", e);
                };
            }
        }

        Ok(())
    }
    async fn send_set_center_chunk(pos: &Position, conn: Arc<RwLock<Connection>>) -> Result<()> {
        let packet = SetCenterChunk::new(pos.x >> 4, pos.z >> 4);

        let mut write_guard = conn.write().await;

        write_guard.send_packet(packet).await?;

        Ok(())
    }
}
