use std::sync::Arc;

use async_trait::async_trait;
use ferrumc_codec::enc::NetEncode;
use tokio::sync::RwLock;
use tracing::{debug, error, warn};

use crate::net::packets::outgoing::chunk_and_light_data::ChunkDataAndUpdateLight;
use crate::net::packets::outgoing::set_center_chunk::SetCenterChunk;
use crate::net::systems::System;
use crate::net::utils::packet_queue::PacketQueue;
use crate::net::{Connection, ConnectionWrapper};
use crate::state::GlobalState;
use crate::utils::components::player::Player;
use crate::utils::encoding::position::Position;
use crate::utils::prelude::*;
use ferrumc_macros::AutoGenName;
use tokio::task::JoinHandle;

const CHUNK_RADIUS: i32 = 16;
const CHUNK_TX_INTERVAL_MS: u64 = 1000;

#[derive(AutoGenName)]
pub struct ChunkSender;

#[async_trait]
impl System for ChunkSender {
    async fn run(&self, state: GlobalState) {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(CHUNK_TX_INTERVAL_MS));
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
        let start = std::time::Instant::now();
        let mut packet_queue = PacketQueue::new();


        let mut break_loop = false;

        'x: for x in -CHUNK_RADIUS..=CHUNK_RADIUS {
            for z in -CHUNK_RADIUS..=CHUNK_RADIUS {
                let start = std::time::Instant::now();
                let packet =
                    ChunkDataAndUpdateLight::new(state.clone(), (pos.x >> 4) + x, (pos.z >> 4) + z)
                        .await?;

                if let Err(e) = packet_queue.queue(packet).await {
                    warn!("Failed to send chunk to player: {}", e);
                    break 'x;
                };
            }
        }

        tokio::spawn(async move {
            let start = std::time::Instant::now();
            debug!("Getting read guard");
            let read_guard = conn.read().await;
            debug!("Sending packet");
            if let Err(e) = read_guard.send_packet(packet_queue).await {
                error!("Failed to send chunk data to player: {}", e);
            }
            debug!("Packet sent; took: {:?}", start.elapsed());
        });


        debug!(
            "Send {} chunks to player in {:?}",
            (CHUNK_RADIUS * 2 + 1) * (CHUNK_RADIUS * 2 + 1),
            start.elapsed()
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
