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
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
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

        let (player, c_pos, c_conn) = state
            .world
            .get_components::<(Player, Position, ConnectionWrapper)>(entity_id)
            .await?;

        let pos = c_pos.clone();
        let conn = c_conn.0.clone();

        drop(c_pos);
        drop(c_conn);

        debug!(
            "Sending chunks to player: {} @ {:?}",
            player.get_username(),
            pos
        );

        ChunkSender::send_set_center_chunk(&pos, conn.clone()).await?;
        ChunkSender::send_chunk_data_to_player(state.clone(), &pos, conn.clone()).await?;

        Ok(())
    }

    async fn send_chunk_data_to_player(
        state: GlobalState,
        pos: &Position,
        conn: Arc<RwLock<Connection>>,
    ) -> Result<()> {
        let mut write_guard = conn.write().await;

        const CHUNK_RADIUS: i32 = 16;

        let mut break_loop = false;

        for x in -CHUNK_RADIUS..=CHUNK_RADIUS {
            for z in -CHUNK_RADIUS..=CHUNK_RADIUS {
                let packet =
                    ChunkDataAndUpdateLight::new(state.clone(), (pos.x >> 4) + x, (pos.z >> 4) + z)
                        .await?;

                if let Err(e) = write_guard.send_packet(packet).await {
                    warn!("Failed to send chunk to player: {}", e);
                    break_loop = true;
                };
                if break_loop {
                    break;
                }
            }
            if break_loop {
                break;
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
