use crate::errors::BinaryError;
use bevy_ecs::prelude::Mut;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::chunk_batch_finish::ChunkBatchFinish;
use ferrumc_net::packets::outgoing::chunk_batch_start::ChunkBatchStart;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;
use ferrumc_world_gen::errors::WorldGenError::WorldError;
use tracing::{error, trace};

pub fn send_chunks(
    state: GlobalState,
    mut chunk_coords: Vec<(i32, i32, String)>,
    conn: &mut Mut<StreamWriter>,
    // recv: &mut Mut<ChunkReceiver>,
    center_chunk: (i32, i32),
) -> Result<(), BinaryError> {
    let (center_x, center_z) = center_chunk;

    // Sort the chunks by distance from the center
    chunk_coords.sort_by(|(x1, z1, _), (x2, z2, _)| {
        let dist1 = (((center_x - x1).pow(2) + (center_z - z1).pow(2)) as f64).sqrt();
        let dist2 = (((center_x - x2).pow(2) + (center_z - z2).pow(2)) as f64).sqrt();
        (dist1 as i32).cmp(&(dist2 as i32))
    });

    let center_chunk_packet = SetCenterChunk::new(center_x, center_z);
    conn.send_packet(center_chunk_packet)?;

    let batch_start_packet = ChunkBatchStart {};
    conn.send_packet(batch_start_packet)?;

    let mut chunks_sent = 0;

    let mut batch = state.thread_pool.batch();

    for (x, z, dim) in chunk_coords {
        let state_clone = state.clone();
        batch.execute(move || {
            let chunk = if state_clone.world.chunk_exists(x, z, &dim).unwrap_or(false) {
                state_clone
                    .world
                    .load_chunk(x, z, &dim)
                    .map_err(WorldError)?
            } else {
                trace!("Generating chunk {}x{} in dimension {}", x, z, dim);
                // Don't bother saving the chunk if it hasn't been edited yet
                state_clone.terrain_generator.generate_chunk(x, z)?
            };
            Ok((ChunkAndLightData::from_chunk(&chunk), x, z))
        })
    }

    let packets = batch.wait();

    for packet in packets {
        match packet {
            Ok((packet, x, z)) => {
                trace!("Sending chunk data for chunk at coordinates ({}, {})", x, z);
                conn.send_packet(packet?)?;
                chunks_sent += 1;
            }
            Err(WorldError(e)) => {
                error!("Failed to generate or load chunk: {:?}", e);
                continue;
            }
            Err(e) => {
                error!("Unexpected error while processing chunk: {:?}", e);
                continue;
            }
        }
    }

    let batch_end_packet = ChunkBatchFinish {
        batch_size: VarInt::new(chunks_sent),
    };
    conn.send_packet(batch_end_packet)?;

    Ok(())
}
