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
use tracing::error;

/// Sends chunks to players without ever blocking the tick thread on generation or compression.
///
/// The work is split into two non-blocking phases per connected player:
///
/// 1. **Submit** — pop queued/dirty chunk coordinates (up to the per-tick budget) and hand each to
///    the thread pool as a fire-and-forget job. The job loads or generates the chunk, builds the
///    `ChunkAndLightData` packet, compresses it, and pushes the encoded bytes onto the receiver's
///    lock-free `results` queue when finished (possibly several ticks later). In-flight coordinates
///    are recorded in `pending` so they are neither resubmitted here nor re-queued by the chunk
///    calculator.
/// 2. **Drain** — collect whatever encoded chunks finished since the last tick and send them to the
///    client wrapped in a single chunk batch.
///
/// Previously this system blocked the tick on `batch.wait()` until every submitted chunk had been
/// generated and compressed, so a burst of new chunks (a player joining, or moving quickly) would
/// overrun the tick budget. Moving the wait off the tick thread keeps each tick cheap regardless of
/// how much terrain is in flight; the trade-off is that freshly queued chunks arrive a few ticks
/// later instead of within the same tick.
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

        let chunk_receiver = &mut *chunk_receiver;

        let player_chunk = IVec2::new(
            pos.coords.x.floor() as i32 >> 4,
            pos.coords.z.floor() as i32 >> 4,
        );
        let radius = effective_view_radius(
            get_global_config().chunk_render_distance as i32,
            client_info.view_distance as i32,
        );

        // ── Phase 1: submit new chunk jobs to the thread pool (fire-and-forget) ──────────────
        let chunk_per_tick = match get_global_config().performance.chunks_per_tick {
            0 => max(
                chunk_receiver.loading.len() / 3,
                get_global_config().performance.chunks_per_tick_min as usize,
            ),
            -1 => usize::MAX,
            hard_limit => hard_limit as usize,
        };

        let is_compressed = conn.compress.load(Ordering::Relaxed);
        let mut submitted = 0;
        while submitted < chunk_per_tick {
            // Dirty chunks (already sent once, needing a resend) take priority over first-time
            // loads, matching the previous ordering.
            let Some(coords) = chunk_receiver
                .dirty
                .pop_front()
                .or_else(|| chunk_receiver.loading.pop_front())
            else {
                break;
            };

            // Skip anything already in flight or already sent.
            if chunk_receiver.pending.contains(&coords) || chunk_receiver.loaded.contains(&coords) {
                continue;
            }

            chunk_receiver.pending.insert(coords);
            submitted += 1;

            let state_arc = state.0.clone();
            let results = chunk_receiver.results.clone();
            // `oneshot` runs the job on the thread pool and the returned handle is dropped, leaving
            // the job to complete in the background and report back through `results`.
            drop(state.0.thread_pool.oneshot(move || {
                let pos = ChunkPos::new(coords.0, coords.1);
                if let Err(e) =
                    ferrumc_utils::world::load_or_generate_chunk(&state_arc, pos, "overworld")
                {
                    error!("Failed to load or generate chunk {:?}: {}", coords, e);
                    results.push((coords, None));
                    return;
                }

                // Settle generated "hanging" fluids here on the worker thread, before the chunk is
                // encoded and sent, so the chunk arrives already flowed and the game-tick thread does
                // no fluid simulation for it. Flow contained within the chunk is fully resolved;
                // cross-chunk seams are left to the on-load settle pass.
                let fluids = &get_global_config().fluids;
                if fluids.settle_on_generate {
                    if let Some(mut chunk) = state_arc.world.cached_chunk_mut(pos, "overworld") {
                        ferrumc_world::fluid::settle::settle_chunk(
                            &mut chunk,
                            pos,
                            ferrumc_world::dimension::Dimension::Overworld,
                            fluids.algorithm,
                            fluids.max_settle_changes as usize,
                        );
                    }
                }

                let Some(chunk) = state_arc.world.cached_chunk(pos, "overworld") else {
                    error!("Chunk {:?} vanished from cache after generation", coords);
                    results.push((coords, None));
                    return;
                };
                let packet = match ChunkAndLightData::from_chunk(pos, &chunk) {
                    Ok(packet) => packet,
                    Err(e) => {
                        error!("Failed to build chunk packet for {:?}: {}", coords, e);
                        results.push((coords, None));
                        return;
                    }
                };
                match compress_packet(
                    &packet,
                    is_compressed,
                    &NetEncodeOpts::WithLength,
                    get_global_config().network_compression_threshold as usize,
                ) {
                    Ok(bytes) => results.push((coords, Some(bytes))),
                    Err(e) => {
                        error!("Failed to compress chunk packet for {:?}: {}", coords, e);
                        results.push((coords, None));
                    }
                }
            }));
        }

        // ── Phase 2: drain finished chunks and send them in a single batch ───────────────────
        let mut ready: Vec<Vec<u8>> = Vec::new();
        while let Some((coords, maybe_bytes)) = chunk_receiver.results.pop() {
            chunk_receiver.pending.remove(&coords);
            let Some(bytes) = maybe_bytes else {
                continue; // Job failed; leave it unloaded so the calculator can re-queue it.
            };
            // Drop chunks the player has since moved away from; they will be re-queued if they come
            // back into view. Uses the same Chebyshev metric the calculator queues with.
            if IVec2::new(coords.0, coords.1).chebyshev_distance(player_chunk) > radius as u32 {
                continue;
            }
            chunk_receiver.loaded.insert(coords);
            ready.push(bytes);
        }

        if !ready.is_empty() {
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

            let batch_size = ready.len();
            for bytes in ready {
                if conn.send_raw_packet(bytes).is_err() {
                    continue 'entity;
                }
            }

            if conn
                .send_packet(ChunkBatchFinish {
                    batch_size: batch_size.into(),
                })
                .is_err()
            {
                continue 'entity;
            }
        }

        // ── Phase 3: tell the client to unload chunks that are no longer needed ──────────────
        while let Some(coords) = chunk_receiver.unloading.pop_front() {
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
