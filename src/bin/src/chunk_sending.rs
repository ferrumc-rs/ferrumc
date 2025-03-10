use crate::BinaryError;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::chunk_batch_finish::ChunkBatchFinish;
use ferrumc_net::packets::outgoing::chunk_batch_start::ChunkBatchStart;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;

/// Fetches and sends chunks to the client
///
/// # Arguments
///
/// * `state` - The global state of the server.
/// * `eid` - The entity ID of the player.
///
/// # Returns
///
/// * `Ok(())` - If the chunks are successfully sent.
/// * `Err(BinaryError)` - If an error occurs while sending the chunks.
pub async fn send_chunks(state: GlobalState, eid: usize) -> Result<(), BinaryError> {
    let mut recv = state.universe.get_mut::<ChunkReceiver>(eid)?;
    let mut chunk_coords = Vec::new();
    for (x, z, _) in recv.can_see.iter() {
        chunk_coords.push((*x, *z));
    }
    chunk_coords.retain(|(x, z)| !recv.seen.contains(&(*x, *z, "overworld".to_string())));
    let (center_x, center_z, _) = recv.last_chunk;

    // Sort the chunks by distance from the center
    chunk_coords.sort_by(|(x1, z1), (x2, z2)| {
        let dist1 = (((center_x - x1).pow(2) + (center_z - z1).pow(2)) as f64).sqrt();
        let dist2 = (((center_x - x2).pow(2) + (center_z - z2).pow(2)) as f64).sqrt();
        (dist1 as i32).cmp(&(dist2 as i32))
    });

    let center_chunk_packet = SetCenterChunk::new(center_x, center_z);
    let mut conn = state.universe.get_mut::<StreamWriter>(eid)?;
    conn.send_packet(center_chunk_packet, &NetEncodeOpts::WithLength)?;

    let batch_start_packet = ChunkBatchStart {};
    conn.send_packet(batch_start_packet, &NetEncodeOpts::WithLength)?;

    let mut chunks_sent = 0;

    for (x, z) in chunk_coords {
        let chunk = if state.world.chunk_exists(x, z, "overworld").await? {
            state.world.load_chunk(x, z, "overworld").await?
        } else {
            let generated_chunk = state.terrain_generator.generate_chunk(x, z)?;
            // TODO: Remove this clone
            state.world.save_chunk(generated_chunk.clone()).await?;
            generated_chunk
        };
        let packet = ChunkAndLightData::from_chunk(&chunk)?;
        conn.send_packet(packet, &NetEncodeOpts::WithLength)?;
        recv.seen.insert((x, z, "overworld".to_string()));
        chunks_sent += 1;
    }

    let batch_end_packet = ChunkBatchFinish {
        batch_size: VarInt::new(chunks_sent),
    };
    conn.send_packet(batch_end_packet, &NetEncodeOpts::WithLength)?;

    Ok(())
}
