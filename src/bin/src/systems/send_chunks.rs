use crate::errors::BinaryError;
use bevy_ecs::prelude::{Entity, Mut, Query, Res};
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::chunk_batch_finish::ChunkBatchFinish;
use ferrumc_net::packets::outgoing::chunk_batch_start::ChunkBatchStart;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use tracing::debug;

const CHUNK_RADIUS: i32 = 8;

pub struct ChunkSender;

pub fn chunk_sender_system(query: Query<(Entity, &mut ChunkReceiver, &Position, &mut StreamWriter)>, state: Res<GlobalStateResource>) {
    for (eid, mut recv, pos, mut conn) in query {
        let mut chunks_to_send = vec![];
        {
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
            if let Err(err) = send_chunks(state.clone(), chunks_to_send, &mut conn, &mut recv) {
                debug!("ChunkSender: Failed to send chunks to {}: {:?}", eid, err);
            }
        }
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
    state: GlobalStateResource,
    mut chunk_coords: Vec<(i32, i32, String)>,
    conn: &mut Mut<StreamWriter>,
    recv: &mut Mut<ChunkReceiver>,
) -> Result<(), BinaryError> {
    let (center_x, center_z, _) = recv.last_chunk;

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

    for (x, z, dim) in chunk_coords.clone() {
        let chunk = if state.0.world.chunk_exists(x, z, &dim)? {
            state.0.world.load_chunk(x, z, &dim)?
        } else {
            let generated_chunk = state.0.terrain_generator.generate_chunk(x, z)?;
            // TODO: Remove this clone
            state.0.world.save_chunk(generated_chunk.clone())?;
            generated_chunk
        };
        let packet = ChunkAndLightData::from_chunk(&chunk)?;
        conn.send_packet(packet)?;
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
    conn.send_packet(batch_end_packet)?;

    Ok(())
}
