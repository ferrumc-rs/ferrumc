use crate::errors::BinaryError;
use bevy_ecs::prelude::{Query, Res};
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use ferrumc_net::ChunkBatchAckReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::error;

pub fn handle(
    events: Res<ChunkBatchAckReceiver>,
    state: Res<GlobalStateResource>,
    mut query: Query<(&mut ChunkReceiver, &mut Position, &mut StreamWriter)>,
) {
    if events.0.is_empty() {
        return;
    }
    for (event, eid) in events.0.try_iter() {
        let res: Result<(), BinaryError> = try {
            // The first chunk batch should be the ones sent when the player first joins the server.
            // This just moves them to their spawn position when all their chunks are done loading,
            // preventing them from falling into the floor.

            // If we upgrade to 1.21.5 there should be a packet for this
            let mut move_to_spawn = false;
            let (mut chunk_recv, pos, conn) = query.get_mut(eid)?;
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
            let head_block = state.0.world.get_block_and_fetch(
                pos.x as i32,
                pos.y as i32 - 1,
                pos.z as i32,
                "overworld",
            )?;
            if head_block.name == "minecraft:air" {
                move_to_spawn = false;
            }
            if move_to_spawn {
                conn.send_packet(SynchronizePlayerPositionPacket::default())?;
            }
        };
        if let Err(e) = &res {
            error!("Failed to handle chunk batch ack: {:?}", e);
        }
    }
}
