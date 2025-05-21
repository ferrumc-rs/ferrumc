use crate::errors::BinaryError;
use bevy_ecs::prelude::{Mut, Query, Res};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::chunk_batch_finish::ChunkBatchFinish;
use ferrumc_net::packets::outgoing::chunk_batch_start::ChunkBatchStart;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;
use tracing::trace;

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

    for (x, z, dim) in chunk_coords {
        let chunk = if state.world.chunk_exists(x, z, &dim)? {
            state.world.load_chunk(x, z, &dim)?
        } else {
            trace!("Generating chunk {}x{} in dimension {}", x, z, dim);
            let generated_chunk = state.terrain_generator.generate_chunk(x, z)?;
            // TODO: Remove this clone
            state.world.save_chunk(generated_chunk.clone())?;
            generated_chunk
        };
        assert_eq!(chunk.x, x);
        assert_eq!(chunk.z, z);
        let packet = ChunkAndLightData::from_chunk(&chunk)?;
        conn.send_packet(packet)?;
        // This never actually gets emptied out so if someone goes to enough new chunks and doesn't
        // leave the server, this will eventually run out of memory. Should probably be fixed.
        // recv.seen.insert((x, z, dim.clone()));
        // recv.needs_reload.remove(&(x, z, dim));
        chunks_sent += 1;
    }

    let batch_end_packet = ChunkBatchFinish {
        batch_size: VarInt::new(chunks_sent),
    };
    conn.send_packet(batch_end_packet)?;

    Ok(())
}
