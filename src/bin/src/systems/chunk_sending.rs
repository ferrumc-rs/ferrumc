use bevy_ecs::prelude::{Entity, Query, Res};
use bevy_math::IVec3;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::transform::position::Position;
use ferrumc_net::compression::compress_packet;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::chunk_batch_finish::ChunkBatchFinish;
use ferrumc_net::packets::outgoing::chunk_batch_start::ChunkBatchStart;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::pos::ChunkPos;
use std::sync::atomic::Ordering;

// Just take the needed chunks from the ChunkReceiver and send them
// calculating which chunks are required is figured out elsewhere
// TODO: Respect chunks_per_tick limit
pub fn handle(
    mut query: Query<(Entity, &StreamWriter, &mut ChunkReceiver, &Position)>,
    state: Res<GlobalStateResource>,
) {
    for (eid, conn, mut chunk_receiver, pos) in query.iter_mut() {
        if !state.0.players.is_connected(eid) {
            continue; // Skip if the player is not connected
        }

        if chunk_receiver.dirty.is_empty() && chunk_receiver.loading.is_empty() {
            continue;
        }

        let chunk_receiver = &mut *chunk_receiver;

        let mut dirty_chunks = Vec::new();

        // First handle dirty chunks
        while let Some(coords) = &chunk_receiver.dirty.pop_front() {
            dirty_chunks.push(*coords);
        }

        let mut needed_chunks: Vec<(i32, i32)> = Vec::new();

        // Then handle loading chunks
        while let Some(coords) = chunk_receiver.loading.pop_front() {
            needed_chunks.push(coords);
        }

        needed_chunks.extend(dirty_chunks);

        if needed_chunks.is_empty() {
            continue;
        }

        let mut batch = state.0.thread_pool.batch();

        conn.send_packet(ChunkBatchStart {})
            .expect("Failed to send ChunkBatchStart");

        let center_chunk: IVec3 = pos.coords.floor().as_ivec3() >> 4;

        conn.send_packet(SetCenterChunk {
            x: center_chunk.x.into(),
            z: center_chunk.z.into(),
        })
        .expect("Failed to send SetCenterChunk");

        for coordinates in needed_chunks.into_iter().map(|c| ChunkPos::new(c.0, c.1)) {
            chunk_receiver
                .loaded
                .insert((coordinates.x(), coordinates.z()));
            let state = state.clone();
            let is_compressed = conn.compress.load(Ordering::Relaxed);
            batch.execute({
                move || {
                    let chunk = ferrumc_utils::world::load_or_generate_chunk(
                        &state.0,
                        coordinates,
                        "overworld",
                    )
                    .expect("Failed to load or generate chunk");
                    let packet = ChunkAndLightData::from_chunk(coordinates, &chunk)
                        .expect("Failed to create ChunkAndLightData");
                    compress_packet(
                        &packet,
                        is_compressed,
                        &NetEncodeOpts::WithLength,
                        get_global_config().network_compression_threshold as usize,
                    )
                    .expect("Failed to compress ChunkAndLightData packet")
                }
            });
        }
        let packets = batch.wait();
        let packets_len = packets.len();
        for packet in packets {
            conn.send_raw_packet(packet)
                .expect("Failed to send ChunkAndLightData");
        }

        conn.send_packet(ChunkBatchFinish {
            batch_size: packets_len.into(),
        })
        .expect("Failed to send ChunkBatchFinish");

        // Tell the client to unload chunks that are no longer needed

        while let Some(coords) = &chunk_receiver.unloading.pop_front() {
            let packet = ferrumc_net::packets::outgoing::unload_chunk::UnloadChunk {
                x: coords.0,
                z: coords.1,
            };
            conn.send_packet(packet)
                .expect("Failed to send UnloadChunk packet");
        }
    }
}
