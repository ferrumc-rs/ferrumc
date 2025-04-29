use crate::errors::BinaryError;
use crate::systems::definition::System;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::chunk_batch_finish::ChunkBatchFinish;
use ferrumc_net::packets::outgoing::chunk_batch_start::ChunkBatchStart;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalState;
use std::sync::Arc;
use tracing::debug;

const CHUNK_RADIUS: i32 = 8;

pub struct ChunkSender;

impl System for ChunkSender {
    fn run(self: Arc<Self>, state: GlobalState, tick: u128) -> Result<(), BinaryError> {
        let query = state
            .universe
            .query::<(&mut ChunkReceiver, &Position, &mut StreamWriter)>()
            .into_entities();

        for eid in query {
            let mut chunks_to_send = vec![];
            {
                let pos = state.universe.get_mut::<Position>(eid)?;
                let mut recv = state.universe.get_mut::<ChunkReceiver>(eid)?;
                let current_chunk = (
                    pos.x as i32 >> 4,
                    pos.z as i32 >> 4,
                    "overworld".to_string(),
                );
                recv.last_chunk = current_chunk.clone();

                for x in -CHUNK_RADIUS..=CHUNK_RADIUS {
                    for z in -CHUNK_RADIUS..=CHUNK_RADIUS {
                        chunks_to_send.push((
                            current_chunk.0 + x,
                            current_chunk.1 + z,
                            "overworld".to_string(),
                        ));
                    }
                }
                // recv.seen.retain(|(x, z, _)| {
                //     let dx = (current_chunk.0 - x).abs();
                //     let dz = (current_chunk.1 - z).abs();
                //     dx <= CHUNK_RADIUS && dz <= CHUNK_RADIUS
                // });
                chunks_to_send.retain(|(x, z, dim)| {
                    !recv.seen.contains(&(*x, *z, dim.clone()))
                        || recv.needs_reload.contains(&(*x, *z, dim.clone()))
                });
            }
            if !chunks_to_send.is_empty() {
                debug!("ChunkSender: {} needs {} chunks", eid, chunks_to_send.len());
                send_chunks(state.clone(), eid, chunks_to_send)?;
            }
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "ChunkSender"
    }
}

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
pub fn send_chunks(
    state: GlobalState,
    eid: usize,
    mut chunk_coords: Vec<(i32, i32, String)>,
) -> Result<(), BinaryError> {
    let mut recv = state.universe.get_mut::<ChunkReceiver>(eid)?;

    let (center_x, center_z, _) = recv.last_chunk;

    // Sort the chunks by distance from the center
    chunk_coords.sort_by(|(x1, z1, _), (x2, z2, _)| {
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

    for (x, z, dim) in chunk_coords.clone() {
        let chunk = if state.world.chunk_exists(x, z, &dim)? {
            state.world.load_chunk(x, z, &dim)?
        } else {
            let generated_chunk = state.terrain_generator.generate_chunk(x, z)?;
            // TODO: Remove this clone
            state.world.save_chunk(generated_chunk.clone())?;
            generated_chunk
        };
        let packet = ChunkAndLightData::from_chunk(&chunk)?;
        conn.send_packet(packet, &NetEncodeOpts::WithLength)?;
        // This never actually gets emptied out so if someone goes to enough new chunks and doesn't
        // leave the server, this will eventually run out of memory. Should probably be fixed.
        recv.seen.insert((x, z, dim.clone()));
        recv.needs_reload.remove(&(x, z, dim));
        chunks_sent += 1;
    }

    debug!("Chunks sent {}", chunks_sent);

    let batch_end_packet = ChunkBatchFinish {
        batch_size: VarInt::new(chunks_sent),
    };
    conn.send_packet(batch_end_packet, &NetEncodeOpts::WithLength)?;

    Ok(())
}
