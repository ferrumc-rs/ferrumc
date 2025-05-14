#![feature(try_blocks)]

use bevy_ecs::prelude::Query;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use ferrumc_net::ChunkBatchAckReceiver;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::ServerState;
use std::sync::Arc;

pub fn handle(events: ChunkBatchAckReceiver, state: Arc<ServerState>, mut query: Query<(&mut ChunkReceiver, &mut Position, &mut StreamWriter)>) {
    for (event, eid) in events.0 {
        let res = try {
            // The first chunk batch should be the ones sent when the player first joins the server.
            // This just moves them to their spawn position when all their chunks are done loading,
            // preventing them from falling into the floor.

            // If we upgrade to 1.21.5 there should be a packet for this
            let mut move_to_spawn = false;
            let (chunk_recv, pos, mut conn) = query.get_mut(eid)?;
            chunk_recv.chunks_per_tick = event.chunks_per_tick;
            if !chunk_recv
                .has_loaded
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                move_to_spawn = true;
                chunk_recv
                    .has_loaded
                    .store(true, std::sync::atomic::Ordering::Relaxed);
            }
            // If they aren't underground, don't move them to spawn
            let head_block = state.world.get_block_and_fetch(
                pos.x as i32,
                pos.y as i32 - 1,
                pos.z as i32,
                "overworld",
            )?;
            if head_block.name == "minecraft:air" {
                move_to_spawn = false;
            }
            if move_to_spawn {
                conn.send_packet(
                    SynchronizePlayerPositionPacket::default(),
                    &NetEncodeOpts::WithLength,
                )?;
            }
            Ok(())
        };
    }
}