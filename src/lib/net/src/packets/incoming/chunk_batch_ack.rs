use crate::connection::StreamWriter;
use crate::errors::NetError;
use crate::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use crate::packets::IncomingPacket;

use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::transform::position::Position;
use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::ServerState;
use std::sync::Arc;

#[derive(NetDecode)]
#[packet(packet_id = "chunk_batch_received", state = "play")]
pub struct ChunkBatchAck {
    chunks_per_tick: f32,
}

impl IncomingPacket for ChunkBatchAck {
    fn handle(self, conn_id: usize, state: Arc<ServerState>) -> Result<(), NetError> {
        // The first chunk batch should be the ones sent when the player first joins the server.
        // This just moves them to their spawn position when all their chunks are done loading,
        // preventing them from falling into the floor.
        let mut move_to_spawn = false;
        {
            let mut chunk_recv = state.universe.get_mut::<ChunkReceiver>(conn_id)?;
            chunk_recv.chunks_per_tick = self.chunks_per_tick;
            if !chunk_recv
                .has_loaded
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                move_to_spawn = true;
                chunk_recv
                    .has_loaded
                    .store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }
        {
            // If they aren't underground, don't move them to spawn
            let pos = state.universe.get_mut::<Position>(conn_id)?;
            let head_block = state.world.get_block_and_fetch(
                pos.x as i32,
                pos.y as i32 - 1,
                pos.z as i32,
                "overworld",
            )?;
            if head_block.name == "minecraft:air" {
                move_to_spawn = false;
            }
        }
        if move_to_spawn {
            let mut conn = state.universe.get_mut::<StreamWriter>(conn_id)?;
            conn.send_packet(
                SynchronizePlayerPositionPacket::default(),
                &NetEncodeOpts::WithLength,
            )?;
        }
        Ok(())
    }
}
