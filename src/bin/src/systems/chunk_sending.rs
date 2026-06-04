use bevy_ecs::prelude::{Entity, Query, Res};
use bevy_math::{IVec2, IVec3};
use ferrumc_components::player::client_information::ClientInformationComponent;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::chunks::chunk_receiver::{effective_view_radius, ChunkReceiver};
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
use std::cmp::max;
use std::sync::atomic::Ordering;

// Just take the needed chunks from the ChunkReceiver and send them
// calculating which chunks are required is figured out elsewhere
// TODO: Respect chunks_per_tick limit
pub fn handle(
    mut query: Query<(
        Entity,
        &StreamWriter,
        &mut ChunkReceiver,
        &Position,
        &ClientInformationComponent,
    )>,
    state: Res<GlobalStateResource>,
) {
    'entity: for (eid, conn, mut chunk_receiver, pos, client_info) in query.iter_mut() {
        if !state.0.players.is_connected(eid) {
            continue 'entity; // Skip if the player is not connected
        }

        let chunk_per_tick = match get_global_config().performance.chunks_per_tick {
            0 => max(
                chunk_receiver.loading.len() / 3,
                get_global_config().performance.chunks_per_tick_min as usize,
            ),
            -1 => usize::MAX,
            hard_limit => hard_limit as usize,
        };

        if chunk_receiver.dirty.is_empty() && chunk_receiver.loading.is_empty() {
            continue;
        }

        let chunk_receiver = &mut *chunk_receiver;

        let mut dirty_chunks = Vec::new();
        let mut sent_chunks = 0;

        // First handle dirty chunks
        while let Some(coords) = &chunk_receiver.dirty.pop_front() {
            dirty_chunks.push(*coords);
            sent_chunks += 1;
            if sent_chunks >= chunk_per_tick {
                break;
            }
        }

        let mut needed_chunks: Vec<(i32, i32)> = Vec::new();

        if sent_chunks < chunk_per_tick {
            // Then handle loading chunks
            while let Some(coords) = chunk_receiver.loading.pop_front() {
                needed_chunks.push(coords);
                sent_chunks += 1;
                if sent_chunks >= chunk_per_tick {
                    break;
                };
            }
        }

        needed_chunks.extend(dirty_chunks);

        if needed_chunks.is_empty() {
            continue;
        };

        let mut batch = state.0.thread_pool.batch();

        if conn.send_packet(ChunkBatchStart {}).is_err() {
            continue 'entity;
        }

        let center_chunk: IVec3 = pos.coords.floor().as_ivec3() >> 4;

        if conn
            .send_packet(SetCenterChunk {
                x: center_chunk.x.into(),
                z: center_chunk.z.into(),
            })
            .is_err()
        {
            continue 'entity;
        }

        for coordinates in needed_chunks
            .into_iter()
            .filter(|coord| {
                // Keep only chunks within the player's effective view radius, using the SAME
                // metric (Chebyshev / square) and the SAME radius the calculator queued with.
                // Previously this used a Euclidean circle (distance_squared <= r^2) while the
                // calculator queued a square, so the square's corner chunks were popped here,
                // failed this filter, were dropped without ever entering `loaded`, and got
                // re-queued forever — wasting a tick's send budget and leaving the corners void.
                let player_chunk_pos = IVec2::new(
                    pos.coords.x.floor() as i32 >> 4,
                    pos.coords.z.floor() as i32 >> 4,
                );
                let chunk_pos = IVec2::new(coord.0, coord.1);
                let radius = effective_view_radius(
                    get_global_config().chunk_render_distance as i32,
                    client_info.view_distance as i32,
                );
                chunk_pos.chebyshev_distance(player_chunk_pos) <= radius as u32
            })
            .map(|c| ChunkPos::new(c.0, c.1))
        {
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
            if conn.send_raw_packet(packet).is_err() {
                continue 'entity;
            }
        }

        if conn
            .send_packet(ChunkBatchFinish {
                batch_size: packets_len.into(),
            })
            .is_err()
        {
            continue 'entity;
        }

        // Tell the client to unload chunks that are no longer needed
        while let Some(coords) = &chunk_receiver.unloading.pop_front() {
            let packet = ferrumc_net::packets::outgoing::unload_chunk::UnloadChunk {
                x: coords.0,
                z: coords.1,
            };
            if conn.send_packet(packet).is_err() {
                continue 'entity;
            }
        }
    }
}
