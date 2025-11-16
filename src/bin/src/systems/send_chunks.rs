use crate::errors::BinaryError;
use bevy_ecs::prelude::Mut;
use ferrumc_net::compression::compress_packet;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::errors::NetError;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::chunk_batch_finish::ChunkBatchFinish;
use ferrumc_net::packets::outgoing::chunk_batch_start::ChunkBatchStart;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net_codec::encode::NetEncode;
use ferrumc_net_codec::encode::NetEncodeOpts::WithLength;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;
use std::sync::atomic::Ordering;
use tracing::{debug, error, trace};

pub fn send_chunks(
    state: GlobalState,
    mut chunk_coords: Vec<(i32, i32, String)>,
    conn: &mut Mut<StreamWriter>,
    // recv: &mut Mut<ChunkReceiver>,
    center_chunk: (i32, i32),
) -> Result<(), BinaryError> {
    debug!("sending chunks {chunk_coords:?} to {conn:?}");
    let (center_x, center_z) = center_chunk;

    // Sort the chunks by distance from the center
    chunk_coords.sort_by(|(x1, z1, _), (x2, z2, _)| {
        let dist1 = (((center_x - x1).pow(2) + (center_z - z1).pow(2)) as f64).sqrt();
        let dist2 = (((center_x - x2).pow(2) + (center_z - z2).pow(2)) as f64).sqrt();
        (dist1 as i32).cmp(&(dist2 as i32))
    });

    let center_chunk_packet = SetCenterChunk::new(center_x, center_z);
    conn.send_packet_ref(&center_chunk_packet)?;

    let batch_start_packet = ChunkBatchStart {};
    conn.send_packet_ref(&batch_start_packet)?;

    let mut chunks_sent = 0;

    let mut batch = state.thread_pool.batch();

    let is_compressed = conn.compress.load(Ordering::Relaxed);

    for (x, z, dim) in chunk_coords {
        let state_clone = state.clone();
        batch.execute(move || {
            let (packet, x, z) = if state_clone.world.chunk_exists(x, z, &dim).unwrap_or(false) {
                let chunk = state_clone
                    .world
                    .load_chunk(x, z, &dim)
                    .map_err(|err| NetError::Misc(err.to_string()))?;
                Ok::<(Result<ChunkAndLightData, NetError>, i32, i32), NetError>((
                    ChunkAndLightData::from_chunk(&chunk),
                    x,
                    z,
                ))
            } else {
                trace!("Generating chunk {}x{} in dimension {}", x, z, dim);
                // Don't bother saving the chunk if it hasn't been edited yet
                let chunk = state_clone
                    .terrain_generator
                    .generate_chunk(x, z)
                    .map_err(|err| NetError::Misc(err.to_string()))?;
                Ok((ChunkAndLightData::from_chunk(&chunk), x, z))
            }?;
            match packet {
                Ok(packet) => {
                    if is_compressed {
                        // Compress the packet if compression is enabled
                        let compressed_packet = compress_packet(&packet, true, &WithLength)?;
                        Ok((compressed_packet, x, z))
                    } else {
                        let mut buffer = Vec::new();
                        packet
                            .encode(&mut buffer, &WithLength)
                            .map_err(|e| NetError::Misc(e.to_string()))?;
                        Ok((buffer, x, z))
                    }
                }
                Err(e) => {
                    error!("Failed to create chunk packet: {:?}", e);
                    Err(e)
                }
            }
        })
    }

    let packets = batch.wait();

    for packet in packets {
        match packet {
            Ok((packet, x, z)) => {
                trace!("Sending chunk data for chunk at coordinates ({}, {})", x, z);
                conn.send_raw_packet(packet)?;
                chunks_sent += 1;
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
    conn.send_packet_ref(&batch_end_packet)?;

    Ok(())
}
