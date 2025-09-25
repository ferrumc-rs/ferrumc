use crate::errors::BinaryError;
use bevy_ecs::prelude::Mut;
use bevy_math::IVec2;
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
use tracing::{error, trace};

pub fn send_chunks(
    state: GlobalState,
    mut chunk_coords: Vec<(IVec2, String)>,
    conn: &mut Mut<StreamWriter>,
    center_chunk: IVec2,
) -> Result<(), BinaryError> {
    // Sort the chunks by distance from the center
    chunk_coords.sort_by(|(p1, _), (p2, _)| {
        let d1 = p1.distance_squared(center_chunk);
        let d2 = p2.distance_squared(center_chunk);
        d1.partial_cmp(&d2).unwrap()
    });

    let center_chunk_packet = SetCenterChunk::new(center_chunk);
    conn.send_packet_ref(&center_chunk_packet)?;

    let batch_start_packet = ChunkBatchStart {};
    conn.send_packet_ref(&batch_start_packet)?;

    let mut chunks_sent = 0;

    let mut batch = state.thread_pool.batch();

    let is_compressed = conn.compress.load(Ordering::Relaxed);

    for (pos, dim) in chunk_coords {
        let state_clone = state.clone();
        batch.execute(move || {
            let (packet, pos) = if state_clone.world.chunk_exists(pos, &dim).unwrap_or(false) {
                let chunk = state_clone
                    .world
                    .load_chunk(pos, &dim)
                    .map_err(|err| NetError::Misc(err.to_string()))?;
                Ok::<(Result<ChunkAndLightData, NetError>, IVec2), NetError>((
                    ChunkAndLightData::from_chunk(&chunk),
                    pos,
                ))
            } else {
                trace!("Generating chunk {} in dimension {}", pos, dim);
                // Don't bother saving the chunk if it hasn't been edited yet
                let chunk = state_clone
                    .terrain_generator
                    .generate_chunk(pos)
                    .map_err(|err| NetError::Misc(err.to_string()))?;
                Ok((ChunkAndLightData::from_chunk(&chunk), pos))
            }?;
            match packet {
                Ok(packet) => {
                    if is_compressed {
                        // Compress the packet if compression is enabled
                        let compressed_packet = compress_packet(&packet, true, &WithLength)?;
                        Ok((compressed_packet, pos))
                    } else {
                        let mut buffer = Vec::new();
                        packet
                            .encode(&mut buffer, &WithLength)
                            .map_err(|e| NetError::Misc(e.to_string()))?;
                        Ok((buffer, pos))
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
            Ok((packet, pos)) => {
                trace!("Sending chunk data for chunk at coordinates ({})", pos);
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
