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
                        recv.can_see.insert((
                            current_chunk.0 + x,
                            current_chunk.1 + z,
                            "overworld".to_string(),
                        ));
                    }
                }
            }
            send_chunks(state.clone(), eid)?
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
pub fn send_chunks(state: GlobalState, eid: usize) -> Result<(), BinaryError> {
    let mut chunk_coords = Vec::new();

    let mut recv = state.universe.get_mut::<ChunkReceiver>(eid)?;
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

    debug!(
        "ChunkSender: Sending {} chunks to player {}",
        chunk_coords.len(),
        eid
    );

    let center_chunk_packet = SetCenterChunk::new(center_x, center_z);
    let mut conn = state.universe.get_mut::<StreamWriter>(eid)?;
    conn.send_packet(center_chunk_packet, &NetEncodeOpts::WithLength)?;

    let batch_start_packet = ChunkBatchStart {};
    conn.send_packet(batch_start_packet, &NetEncodeOpts::WithLength)?;

    let mut chunks_sent = 0;

    for (x, z) in chunk_coords {
        let chunk = if state.world.chunk_exists(x, z, "overworld")? {
            state.world.load_chunk(x, z, "overworld")?
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
        recv.seen.insert((x, z, "overworld".to_string()));
        chunks_sent += 1;
    }

    let batch_end_packet = ChunkBatchFinish {
        batch_size: VarInt::new(chunks_sent),
    };
    conn.send_packet(batch_end_packet, &NetEncodeOpts::WithLength)?;

    Ok(())
}
