use std::sync::Arc;

use async_trait::async_trait;
use ferrumc_codec::enc::NetEncode;
use tokio::sync::RwLock;
use tracing::{debug, error, warn};

use crate::net::packets::incoming::client_info::ClientInfo;
use crate::net::packets::outgoing::chunk_and_light_data::ChunkDataAndUpdateLight;
use crate::net::packets::outgoing::set_center_chunk::SetCenterChunk;
use crate::net::systems::System;
use crate::net::{Connection, ConnectionWrapper};
use crate::state::GlobalState;
use crate::utils::components::last_chunk_tx_pos::LastChunkTxPos;
use crate::utils::components::player::Player;
use crate::utils::encoding::position::Position;
use crate::utils::prelude::*;
use ferrumc_macros::AutoGenName;

pub const DEFAULT_CHUNK_RADIUS: i8 = 16;
const CHUNK_TX_INTERVAL_MS: u64 = 50000;

#[derive(AutoGenName)]
pub struct ChunkSender;

#[async_trait]
impl System for ChunkSender {
    async fn run(&self, state: GlobalState) {
        let mut interval =
            tokio::time::interval(std::time::Duration::from_millis(CHUNK_TX_INTERVAL_MS));
        loop {
            interval.tick().await;

            // Get all the Players, instead of all the *entities*. The player is just a filter.
            let query = state.world.query::<&Player>();
            let send_to = query.iter().await.collect::<Vec<_>>();

            send_to.into_iter().for_each(|(entity_id, player)| {
                debug!("Sending chunk to player: {}", player.get_username());
                drop(player);
                let state = state.clone();
                tokio::spawn(async move {
                    if let Err(e) = ChunkSender::send_chunks_to_player(state, entity_id).await {
                        error!("Failed to send chunk to player: {}", e);
                    }
                });
            });
        }
    }

    fn name(&self) -> &'static str {
        Self::type_name()
    }
}

impl ChunkSender {
    pub async fn send_chunks_to_player_if_needed(
        state: GlobalState,
        entity_id: impl TryInto<usize>,
        current_pos: (i32, i32),
    ) -> Result<()> {
        let entity_id = entity_id.try_into().map_err(|_| Error::ConversionError)?;

        let mut last_chunk_tx_pos = state
            .world
            .get_component_storage()
            .get_mut_or_insert_with::<LastChunkTxPos>(entity_id, Default::default)
            .await;

        let client_info = state.world.get_component::<ClientInfo>(entity_id).await?;

        let distance = last_chunk_tx_pos.distance_to(current_pos.0, current_pos.1);

        if distance < (client_info.view_distance as f64 / 5f64) {
            return Ok(());
        }

        last_chunk_tx_pos.set_last_chunk_tx_pos(current_pos.0, current_pos.1);

        let state_clone = state.clone();
        tokio::spawn(async move {
            if let Err(e) = ChunkSender::send_chunks_to_player(state_clone, entity_id).await {
                error!("Failed to send chunk to player: {}", e);
            }
        });

        Ok(())
    }
    pub async fn send_chunks_to_player(
        state: GlobalState,
        entity_id: impl TryInto<usize>,
    ) -> Result<()> {
        let entity_id = entity_id.try_into().map_err(|_| Error::ConversionError)?;

        let (player, c_pos, c_conn) = state
            .world
            .get_components::<(Player, Position, ConnectionWrapper)>(entity_id)
            .await?;

        let c_info = state
            .world
            .get_component::<ClientInfo>(entity_id)
            .await
            .ok();

        let pos = c_pos.clone();
        let view_distance: i8 = c_info
            .as_ref()
            .map_or(DEFAULT_CHUNK_RADIUS, |c| c.view_distance);
        let conn = c_conn.0.clone();

        drop(c_pos);
        drop(c_conn);

        debug!(
            "Sending chunks to player: {} @ {:?}",
            player.get_username(),
            pos
        );

        drop(player);

        ChunkSender::send_set_center_chunk(&pos, conn.clone()).await?;
        ChunkSender::send_chunk_data_to_player(state.clone(), &pos, view_distance, conn.clone())
            .await?;

        Ok(())
    }

    async fn send_chunk_data_to_player(
        state: GlobalState,
        pos: &Position,
        player_view_distance: i8,
        conn: Arc<RwLock<Connection>>,
    ) -> Result<()> {
        let start = std::time::Instant::now();

        let pos_x = pos.x;
        let pos_z = pos.z;

        let chunk_radius = player_view_distance as i32;

        'x: for x in -chunk_radius..=chunk_radius {
            for z in -chunk_radius..=chunk_radius {
                let Ok(packet) =
                    ChunkDataAndUpdateLight::new(state.clone(), (pos_x >> 4) + x, (pos_z >> 4) + z)
                        .await
                else {
                    continue;
                };
                let conn_read = conn.read().await;
                if let Err(e) = conn_read.send_packet(packet).await {
                    warn!("Failed to send chunk to player: {} ; Cancelling.", e);
                    break 'x;
                }
            }
        }

        // check the size of a single chunk and multiply it by the number of chunks sent
        let sample_chunk =
            ChunkDataAndUpdateLight::new(state.clone(), pos_x >> 4, pos_z >> 4).await?;
        let mut vec = vec![];
        sample_chunk.net_encode(&mut vec).await?;
        let chunk_rad_axis = chunk_radius * 2 + 1;
        debug!(
                "Send {}({}x{}) chunks to player in {:?}. Approximately {} kb of data (~{} kb per chunk)",
                chunk_rad_axis * chunk_rad_axis,
                chunk_rad_axis,
                chunk_rad_axis,
                start.elapsed(),
                vec.len() as i32 * chunk_rad_axis * chunk_rad_axis / 1024,
                vec.len() as i32 / 1024
            );

        Ok(())
    }
    async fn send_set_center_chunk(pos: &Position, conn: Arc<RwLock<Connection>>) -> Result<()> {
        let packet = SetCenterChunk::new(pos.x >> 4, pos.z >> 4);

        let read_guard = conn.read().await;

        read_guard.send_packet(packet).await?;

        Ok(())
    }
}
