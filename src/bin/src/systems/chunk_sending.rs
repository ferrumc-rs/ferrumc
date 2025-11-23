use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::chunk_batch_start::ChunkBatchStart;
use ferrumc_state::GlobalStateResource;
use std::iter;

// Just take the needed chunks from the ChunkReceiver and send them
// calculating which chunks are required is figured out elsewhere
pub fn handle(
    mut query: Query<(Entity, &StreamWriter, &mut ChunkReceiver)>,
    state: Res<GlobalStateResource>,
) {
    for (eid, conn, mut chunk_receiver) in query.iter_mut() {
        if !state.0.players.is_connected(eid) {
            continue; // Skip if the player is not connected
        }

        if chunk_receiver.dirty.is_empty() && chunk_receiver.loading.is_empty() {
            continue;
        }

        let chunk_receiver = &mut *chunk_receiver;

        let needed_chunks: Vec<_> = iter::chain(
            chunk_receiver.dirty.extract_if(.., |_| true),
            chunk_receiver.loading.extract_if(.., |_| true),
        )
        .take(chunk_receiver.chunks_per_tick.floor() as usize)
        .collect();

        if needed_chunks.is_empty() {
            continue;
        }

        let mut batch = state.0.thread_pool.batch();

        conn.send_packet(ChunkBatchStart {})
            .expect("Failed to send ChunkBatchStart");

        for coordinates in needed_chunks {
            let state = state.clone();
            let coordinates = coordinates.clone();
            batch.execute({
                move || {
                    let chunk =
                        match state
                            .0
                            .world
                            .load_chunk(coordinates.0, coordinates.1, "overworld")
                        {
                            Ok(c) => c,
                            Err(_) => state
                                .0
                                .terrain_generator
                                .generate_chunk(coordinates.0, coordinates.1)
                                .expect("Failed to generate chunk")
                                .into(),
                        };
                    ChunkAndLightData::from_chunk(&*chunk)
                        .expect("Failed to create ChunkAndLightData packet")
                }
            });
        }
        for packet in batch.wait() {
            conn.send_packet(packet)
                .expect("Failed to send ChunkAndLightData");
        }
    }
}
