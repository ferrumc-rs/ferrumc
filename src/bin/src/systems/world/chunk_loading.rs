//! Async chunk loader task for per-player chunk streaming.
//!
//! Each player gets their own dedicated Tokio task that handles chunk loading.
//! This task sleeps (0% CPU) when the player is stationary and only wakes up
//! when it receives a command via the channel.
//!
//! ## Architecture
//!
//! The chunk loader implements a "Sliding Window" algorithm with queue invalidation:
//! - On first join, all chunks in the view radius are queued in spiral order
//! - On movement, stale chunks are removed from the queue and new chunks are added
//! - Chunks are sent in batches, waiting for client acknowledgment between batches
//! - Before sending, each chunk is validated to still be within the current radius
//!
//! ## Non-Blocking Architecture
//!
//! CPU-heavy work (generation, serialization, compression) is offloaded to blocking
//! tasks via `tokio::task::spawn_blocking`. The async task only:
//! - Decides which chunks to send
//! - Spawns workers for heavy computation
//! - Awaits compressed bytes
//! - Writes raw bytes to the connection
//!
//! This keeps the Tokio runtime responsive and allows chunk work to utilize
//! the blocking thread pool.
//!
//! ## Fast Movement Handling
//!
//! When the player moves faster than chunks can be sent:
//! 1. Stale chunks (outside new radius) are purged from the queue
//! 2. Duplicate chunks are prevented via HashSet tracking
//! 3. Pre-send validation skips chunks that became stale mid-batch
//!
//! ## Protocol Flow
//!
//! ```text
//! Server                          Client
//!   |                                |
//!   |-- SetCenterChunk ------------->|
//!   |-- ChunkBatchStart ------------>|
//!   |-- ChunkAndLightData[] -------->|
//!   |-- ChunkBatchFinish ----------->|
//!   |                                |
//!   |<-------- ChunkBatchReceived ---|
//!   |                                |
//!   |-- (next batch if queue) ------>|
//! ```

use ferrumc_components::chunks::ChunkCommand;
use ferrumc_net::compression::compress_packet;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::chunk_batch_finish::ChunkBatchFinish;
use ferrumc_net::packets::outgoing::chunk_batch_start::ChunkBatchStart;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net::packets::outgoing::unload_chunk::UnloadChunk;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_state::GlobalState;
use ferrumc_world::pos::ChunkPos;
use ferrumc_world::structure::FerrumcChunk;
use std::collections::{HashSet, VecDeque};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error, info, trace};

// ============================================================================
// Constants
// ============================================================================

/// Default chunks per batch before we receive client feedback.
const DEFAULT_CHUNKS_PER_BATCH: f32 = 32.0;

/// Maximum chunks per batch to prevent packet flooding.
const MAX_CHUNKS_PER_BATCH: f32 = 64.0;

/// Minimum chunks per batch to ensure progress.
const MIN_CHUNKS_PER_BATCH: f32 = 50.0;

// ============================================================================
// Chunk Loader State
// ============================================================================

/// Internal state for the chunk loader task.
///
/// Tracks the player's view center, pending chunks, and rate limiting.
struct ChunkLoaderState {
    /// Current center position (None = not yet initialized)
    center: Option<(i32, i32)>,

    /// Current view radius in chunks
    radius: u8,

    /// Ordered queue of chunks to send (closest first)
    queue: VecDeque<(i32, i32)>,

    /// Set of chunks currently in queue (for O(1) dedup checks)
    queued_set: HashSet<(i32, i32)>,

    /// Set of chunks already sent to client (avoid re-sending)
    sent_chunks: HashSet<(i32, i32)>,

    /// Chunks per batch (dynamically adjusted based on client feedback)
    chunks_per_batch: f32,

    /// Whether we're waiting for client acknowledgment
    awaiting_ack: bool,
}

impl ChunkLoaderState {
    fn new() -> Self {
        Self {
            center: None,
            radius: 0,
            queue: VecDeque::with_capacity(1024),
            queued_set: HashSet::with_capacity(1024),
            sent_chunks: HashSet::with_capacity(2048),
            chunks_per_batch: DEFAULT_CHUNKS_PER_BATCH,
            awaiting_ack: false,
        }
    }

    /// Adds a chunk to the queue if not already queued or sent.
    #[inline]
    fn enqueue(&mut self, chunk: (i32, i32)) {
        if !self.queued_set.contains(&chunk) && !self.sent_chunks.contains(&chunk) {
            self.queue.push_back(chunk);
            self.queued_set.insert(chunk);
        }
    }

    /// Removes and returns the next chunk from the queue.
    #[inline]
    fn dequeue(&mut self) -> Option<(i32, i32)> {
        if let Some(chunk) = self.queue.pop_front() {
            self.queued_set.remove(&chunk);
            Some(chunk)
        } else {
            None
        }
    }

    /// Checks if a chunk is within the current view radius.
    #[inline]
    fn is_in_view(&self, chunk_x: i32, chunk_z: i32) -> bool {
        if let Some((cx, cz)) = self.center {
            let r = self.radius as i32;
            chunk_x >= cx - r && chunk_x <= cx + r && chunk_z >= cz - r && chunk_z <= cz + r
        } else {
            false
        }
    }

    /// Purges stale chunks from the queue that are outside the current view.
    /// Returns the number of chunks removed.
    fn purge_stale_chunks(&mut self) -> usize {
        let before = self.queue.len();

        // Extract center/radius to avoid borrow issues
        let Some((cx, cz)) = self.center else {
            return 0;
        };
        let r = self.radius as i32;

        // Helper closure that doesn't borrow self
        let in_view =
            |x: i32, z: i32| -> bool { x >= cx - r && x <= cx + r && z >= cz - r && z <= cz + r };

        // Retain only chunks still in view
        self.queue.retain(|(x, z)| in_view(*x, *z));

        // Rebuild the queued set
        self.queued_set.clear();
        for chunk in &self.queue {
            self.queued_set.insert(*chunk);
        }

        // Also purge sent_chunks that are no longer in view (memory cleanup)
        self.sent_chunks.retain(|(x, z)| in_view(*x, *z));

        before - self.queue.len()
    }

    /// Clears all state for a fresh start (e.g., teleport).
    fn reset(&mut self) {
        self.queue.clear();
        self.queued_set.clear();
        self.sent_chunks.clear();
        self.awaiting_ack = false;
    }
}

// ============================================================================
// Main Task
// ============================================================================

/// Async task that handles chunk loading for a single player.
///
/// Runs independently of the ECS tick. Sleeps when idle (0% CPU).
pub async fn chunk_loader_task(
    mut rx: Receiver<ChunkCommand>,
    conn: StreamWriter,
    state: GlobalState,
    player_name: String,
) {
    debug!("[{}] Chunk loader started", player_name);

    let mut loader = ChunkLoaderState::new();
    let mut last_command_time = Instant::now();

    while let Some(command) = rx.recv().await {
        let time_since_last = last_command_time.elapsed();
        last_command_time = Instant::now();
        
        match command {
            ChunkCommand::UpdateCenter {
                chunk_x,
                chunk_z,
                radius,
            } => {
                debug!(
                    "[{}] CMD: UpdateCenter({}, {}, r={}) after {:?}",
                    player_name, chunk_x, chunk_z, radius, time_since_last
                );
                
                if handle_center_update(
                    &conn,
                    &state,
                    &player_name,
                    &mut loader,
                    chunk_x,
                    chunk_z,
                    radius,
                )
                .is_err()
                {
                    break;
                }

                // Send batch if not waiting for ack
                if !loader.awaiting_ack && !loader.queue.is_empty() {
                    info!(
                        "[{}] Sending batch (queue={}, not awaiting ack)",
                        player_name, loader.queue.len()
                    );
                    if send_batch(&conn, &state, &player_name, &mut loader).await.is_err() {
                        break;
                    }
                } else if loader.awaiting_ack {
                    debug!(
                        "[{}] Skipping batch: awaiting_ack=true, queue={}",
                        player_name, loader.queue.len()
                    );
                } else if loader.queue.is_empty() {
                    debug!("[{}] Skipping batch: queue empty", player_name);
                }
            }

            ChunkCommand::BatchReceived(desired_rate) => {
                info!(
                    "[{}] CMD: BatchReceived(rate={:.1}) after {:?}, queue={}, awaiting_ack={}",
                    player_name, desired_rate, time_since_last, loader.queue.len(), loader.awaiting_ack
                );
                
                handle_batch_ack(&player_name, &mut loader, desired_rate);

                // Continue sending if queue has items
                if !loader.queue.is_empty() {
                    info!(
                        "[{}] Sending next batch after ACK (queue={})",
                        player_name, loader.queue.len()
                    );
                    if send_batch(&conn, &state, &player_name, &mut loader).await.is_err() {
                        break;
                    }
                } else {
                    info!("[{}] Queue empty after ACK, waiting for movement", player_name);
                }
            }

            ChunkCommand::Stop => {
                debug!("[{}] Chunk loader stopping", player_name);
                break;
            }
        }
    }

    debug!("[{}] Chunk loader ended", player_name);
}

// ============================================================================
// Command Handlers
// ============================================================================

/// Handles a center position update (movement or teleport).
fn handle_center_update(
    conn: &StreamWriter,
    _state: &GlobalState,
    player_name: &str,
    loader: &mut ChunkLoaderState,
    new_x: i32,
    new_z: i32,
    new_radius: u8,
) -> Result<(), ()> {
    let old_center = loader.center;
    let old_radius = loader.radius as i32;
    let new_radius_i32 = new_radius as i32;

    // Update state first (needed for is_in_view checks)
    loader.center = Some((new_x, new_z));
    loader.radius = new_radius;

    // Send SetCenterChunk (required by protocol)
    if conn.send_packet(SetCenterChunk::new(new_x, new_z)).is_err() {
        debug!("[{}] Connection dead (SetCenterChunk failed)", player_name);
        return Err(());
    }

    match old_center {
        None => {
            // First join - queue all chunks in spiral order
            let chunks = generate_spiral(new_x, new_z, new_radius_i32);
            debug!(
                "[{}] Initial load: queueing {} chunks",
                player_name,
                chunks.len()
            );
            for chunk in chunks {
                loader.enqueue(chunk);
            }
        }

        Some((old_x, old_z)) => {
            // Check if this is a teleport (large distance)
            let dx = (new_x - old_x).abs();
            let dz = (new_z - old_z).abs();
            let is_teleport = dx > new_radius_i32 * 2 || dz > new_radius_i32 * 2;

            if is_teleport {
                // Teleport: reset everything and load fresh
                debug!(
                    "[{}] Teleport detected ({} chunks away), resetting state",
                    player_name,
                    dx.max(dz)
                );
                loader.reset();

                // Unload all old chunks
                for x in (old_x - old_radius)..=(old_x + old_radius) {
                    for z in (old_z - old_radius)..=(old_z + old_radius) {
                        let _ = conn.send_packet(UnloadChunk::new(x, z));
                    }
                }

                // Queue new chunks
                for chunk in generate_spiral(new_x, new_z, new_radius_i32) {
                    loader.enqueue(chunk);
                }
            } else {
                // Normal movement: calculate delta
                let purged = loader.purge_stale_chunks();
                if purged > 0 {
                    debug!(
                        "[{}] Purged {} stale chunks from queue",
                        player_name, purged
                    );
                }

                // Unload chunks that left the view
                let mut unloaded = 0;
                for x in (old_x - old_radius)..=(old_x + old_radius) {
                    for z in (old_z - old_radius)..=(old_z + old_radius) {
                        if !loader.is_in_view(x, z) {
                            if conn.send_packet(UnloadChunk::new(x, z)).is_err() {
                                debug!("[{}] Connection dead (UnloadChunk failed)", player_name);
                                return Err(());
                            }
                            loader.sent_chunks.remove(&(x, z));
                            unloaded += 1;
                        }
                    }
                }

                // Queue new chunks that entered the view (sorted by distance)
                let mut new_chunks: Vec<(i32, i32)> = Vec::new();
                for x in (new_x - new_radius_i32)..=(new_x + new_radius_i32) {
                    for z in (new_z - new_radius_i32)..=(new_z + new_radius_i32) {
                        let chunk = (x, z);
                        if !loader.sent_chunks.contains(&chunk)
                            && !loader.queued_set.contains(&chunk)
                        {
                            new_chunks.push(chunk);
                        }
                    }
                }

                // Sort by distance to center (closest first)
                new_chunks.sort_by_key(|(x, z)| {
                    let dx = x - new_x;
                    let dz = z - new_z;
                    dx * dx + dz * dz
                });

                let queued = new_chunks.len();
                for chunk in new_chunks {
                    loader.enqueue(chunk);
                }

                if unloaded > 0 || queued > 0 {
                    trace!(
                        "[{}] Movement: unloaded {}, queued {}, queue size: {}",
                        player_name,
                        unloaded,
                        queued,
                        loader.queue.len()
                    );
                }
            }
        }
    }

    Ok(())
}

/// Handles a batch acknowledgment from the client.
fn handle_batch_ack(player_name: &str, loader: &mut ChunkLoaderState, desired_rate: f32) {
    loader.awaiting_ack = false;

    let new_rate = desired_rate.clamp(MIN_CHUNKS_PER_BATCH, MAX_CHUNKS_PER_BATCH);

    debug!(
        "[{}] Batch ACK received. Rate: {:.1} -> {:.1}, queue: {}",
        player_name,
        loader.chunks_per_batch,
        new_rate,
        loader.queue.len()
    );

    loader.chunks_per_batch = new_rate;
}

// ============================================================================
// Batch Sending
// ============================================================================

/// Result of preparing a single chunk.
struct PreparedChunk {
    /// Chunk coordinates
    coords: (i32, i32),
    /// Pre-compressed packet bytes ready to send
    raw_bytes: Vec<u8>,
    /// If this chunk was newly generated, contains the chunk data for DB persistence.
    /// None if loaded from DB (no need to save again).
    generated_chunk: Option<FerrumcChunk>,
}

/// Sends a batch of chunks to the client using massively parallel architecture.
///
/// All CPU-heavy work (generation, serialization, compression) runs in parallel
/// on `ferrumc_threadpool`. The async task only writes pre-compressed bytes.
///
/// ## Performance Architecture
///
/// ```text
/// Async Task                      Thread Pool (N cores)
///     |                                  |
///     |-- spawn_blocking --------------->|
///     |                                  |-- batch.execute(chunk 0)
///     |                                  |-- batch.execute(chunk 1)
///     |                                  |-- ...
///     |                                  |-- batch.execute(chunk N)
///     |                                  |
///     |                                  |-- batch.wait() [PARALLEL]
///     |                                  |     |
///     |                                  |     +-- Thread 0: gen+serialize+compress
///     |                                  |     +-- Thread 1: gen+serialize+compress
///     |                                  |     +-- Thread 2: gen+serialize+compress
///     |                                  |     +-- ...
///     |                                  |
///     |<-- Vec<PreparedChunk> -----------|
///     |
///     |-- send_raw_packet(chunk 0)
///     |-- send_raw_packet(chunk 1)
///     |-- ...
/// ```
async fn send_batch(
    conn: &StreamWriter,
    state: &GlobalState,
    player_name: &str,
    loader: &mut ChunkLoaderState,
) -> Result<(), ()> {
    if loader.queue.is_empty() {
        return Ok(());
    }

    let target_batch_size = loader.chunks_per_batch.ceil() as usize;
    let compress = conn.compress.load(Ordering::Relaxed);

    trace!(
        "[{}] Sending batch (target: {}, queue: {})",
        player_name,
        target_batch_size,
        loader.queue.len()
    );

    // Start batch
    if conn.send_packet(ChunkBatchStart {}).is_err() {
        debug!("[{}] Connection dead (ChunkBatchStart failed)", player_name);
        return Err(());
    }

    // Collect chunks to prepare (while validating they're in view)
    let mut chunks_to_prepare: Vec<(i32, i32)> = Vec::with_capacity(target_batch_size);
    let mut skipped = 0;

    for _ in 0..target_batch_size {
        let Some((cx, cz)) = loader.dequeue() else {
            break;
        };

        // Skip if chunk is no longer in view (player moved fast)
        if !loader.is_in_view(cx, cz) {
            skipped += 1;
            continue;
        }

        // Skip if already sent (shouldn't happen, but safety check)
        if loader.sent_chunks.contains(&(cx, cz)) {
            skipped += 1;
            continue;
        }

        chunks_to_prepare.push((cx, cz));
    }

    // Offload entire batch to thread pool for PARALLEL execution
    // Phase 1: Generate/Load + Serialize + Compress (PARALLEL - no DB writes!)
    // Phase 2: Collect results
    // Phase 3: Save newly generated chunks to DB (SEQUENTIAL - avoids lock contention)
    let state_clone = Arc::clone(state);
    let chunks = chunks_to_prepare.clone();
    let batch_size = chunks.len();
    
    let batch_start = Instant::now();
    let prepared_chunks = tokio::task::spawn_blocking(move || {
        let blocking_start = Instant::now();
        
        // ================================================================
        // PHASE 1: Parallel generation/loading + serialization + compression
        // NO DB WRITES HERE - avoids LMDB lock contention
        // ================================================================
        let mut batch = state_clone.thread_pool.batch::<Result<PreparedChunk, String>>();

        for (cx, cz) in chunks {
            let state_inner = Arc::clone(&state_clone);
            batch.execute(move || {
                let chunk_start = Instant::now();
                let chunk_pos = ChunkPos::new(cx, cz);
                
                // 1. Try to load from database first (read-only, no lock contention)
                let load_start = Instant::now();
                let (chunk, was_generated) = match state_inner.world.load_ferrumc_chunk(chunk_pos, "overworld") {
                    Ok(loaded) => (loaded, false),
                    Err(ferrumc_world::errors::WorldError::ChunkNotFound) => {
                        // 2. Generate if not found (CPU-intensive noise calculations)
                        let generated = state_inner
                            .terrain_generator
                            .generate_chunk(chunk_pos)
                            .map_err(|e| format!("Generation failed ({}, {}): {:?}", cx, cz, e))?;
                        
                        // DON'T save here - we'll batch save after parallel work completes
                        (generated, true)
                    }
                    Err(e) => {
                        return Err(format!("DB load failed ({}, {}): {:?}", cx, cz, e));
                    }
                };
                let load_gen_time = load_start.elapsed();

                // 3. Create packet (serialization)
                let ser_start = Instant::now();
                let packet = ChunkAndLightData::from_ferrumc_chunk(&chunk)
                    .map_err(|e| format!("Packet creation failed ({}, {}): {:?}", cx, cz, e))?;
                let ser_time = ser_start.elapsed();

                // 4. Compress packet (Zlib compression)
                let comp_start = Instant::now();
                let raw_bytes = compress_packet(&packet, compress, &NetEncodeOpts::WithLength, 512)
                    .map_err(|e| format!("Compression failed ({}, {}): {:?}", cx, cz, e))?;
                let comp_time = comp_start.elapsed();
                
                let total_chunk_time = chunk_start.elapsed();
                
                // Per-chunk timing (trace level to avoid spam)
                trace!(
                    "Chunk ({}, {}) - {}: {:?}, Ser: {:?}, Comp: {:?}, Total: {:?}, Size: {} bytes",
                    cx, cz,
                    if was_generated { "Gen" } else { "Load" },
                    load_gen_time,
                    ser_time,
                    comp_time,
                    total_chunk_time,
                    raw_bytes.len()
                );

                Ok(PreparedChunk {
                    coords: (cx, cz),
                    raw_bytes,
                    // Only include chunk if newly generated (needs DB save)
                    generated_chunk: if was_generated { Some(chunk) } else { None },
                })
            });
        }

        // Wait for ALL parallel work to complete
        let results = batch.wait();
        let parallel_elapsed = blocking_start.elapsed();
        
        // ================================================================
        // PHASE 2 & 3: Collect results and save new chunks SEQUENTIALLY
        // This avoids LMDB write lock contention between threads
        // ================================================================
        let save_start = Instant::now();
        let mut chunks_saved = 0;
        
        // Collect successfully prepared chunks and save any newly generated ones
        let final_results: Vec<Result<PreparedChunk, String>> = results
            .into_iter()
            .map(|result| {
                if let Ok(ref prepared) = result {
                    // Save newly generated chunks to DB (sequential, no lock contention)
                    if let Some(ref chunk) = prepared.generated_chunk {
                        let pos = ChunkPos::new(prepared.coords.0, prepared.coords.1);
                        if let Err(e) = state_clone.world.save_ferrumc_chunk(pos, "overworld", chunk) {
                            tracing::warn!("Failed to save chunk {:?}: {:?}", prepared.coords, e);
                        } else {
                            chunks_saved += 1;
                        }
                    }
                }
                result
            })
            .collect();
        
        let save_elapsed = save_start.elapsed();
        let total_elapsed = blocking_start.elapsed();
        
        debug!(
            "Batch complete: parallel={:?}, saves={} in {:?}, total={:?}",
            parallel_elapsed, chunks_saved, save_elapsed, total_elapsed
        );
        
        final_results
    })
    .await
    .map_err(|e| {
        error!("[{}] Blocking task panicked: {:?}", player_name, e);
    })?;
    
    let batch_prep_time = batch_start.elapsed();
    debug!(
        "[{}] Batch prepared: {} chunks in {:?} ({:.1} chunks/sec)",
        player_name,
        batch_size,
        batch_prep_time,
        batch_size as f64 / batch_prep_time.as_secs_f64()
    );

    // Send all prepared chunks (already compressed, just write bytes)
    let mut sent = 0i32;
    let mut total_send_time = std::time::Duration::ZERO;
    let mut total_bytes_sent = 0usize;
    let send_start = Instant::now();

    for result in prepared_chunks {
        match result {
            Ok(prepared) => {
                let packet_size = prepared.raw_bytes.len();
                let send_one_start = Instant::now();
                if conn.send_raw_packet(prepared.raw_bytes).is_err() {
                    debug!("[{}] Connection dead (chunk send failed)", player_name);
                    return Err(());
                }
                let send_one_time = send_one_start.elapsed();
                total_send_time += send_one_time;
                total_bytes_sent += packet_size;
                
                // Log slow sends (potential network backpressure)
                if send_one_time.as_millis() > 5 {
                    trace!(
                        "[{}] Slow send: chunk ({}, {}) took {:?} ({} bytes)",
                        player_name,
                        prepared.coords.0,
                        prepared.coords.1,
                        send_one_time,
                        packet_size
                    );
                }
                
                loader.sent_chunks.insert(prepared.coords);
                sent += 1;
            }
            Err(e) => {
                error!("[{}] Chunk preparation failed: {}", player_name, e);
            }
        }
    }
    
    let total_send_elapsed = send_start.elapsed();
    debug!(
        "[{}] Network send: {} chunks, {} bytes in {:?} ({:.2} MB/s)",
        player_name,
        sent,
        total_bytes_sent,
        total_send_elapsed,
        (total_bytes_sent as f64 / 1_000_000.0) / total_send_elapsed.as_secs_f64()
    );

    // Finish batch
    if conn
        .send_packet(ChunkBatchFinish {
            batch_size: sent.into(),
        })
        .is_err()
    {
        debug!(
            "[{}] Connection dead (ChunkBatchFinish failed)",
            player_name
        );
        return Err(());
    }

    trace!(
        "[{}] Batch sent: {} chunks, {} skipped, {} remaining",
        player_name,
        sent,
        skipped,
        loader.queue.len()
    );

    loader.awaiting_ack = true;
    Ok(())
}

// ============================================================================
// Chunk Generation Utilities
// ============================================================================

/// Generates chunk coordinates in spiral order from center outward.
///
/// Spiral order ensures the player sees chunks directly under them first,
/// providing the best perceived loading experience.
fn generate_spiral(center_x: i32, center_z: i32, radius: i32) -> Vec<(i32, i32)> {
    let size = (radius * 2 + 1) * (radius * 2 + 1);
    let mut chunks = Vec::with_capacity(size as usize);

    // Start at center
    chunks.push((center_x, center_z));

    // Spiral outward layer by layer
    for r in 1..=radius {
        // Top edge (left to right)
        for x in -r..=r {
            chunks.push((center_x + x, center_z - r));
        }
        // Right edge (excluding top corner)
        for z in (-r + 1)..=r {
            chunks.push((center_x + r, center_z + z));
        }
        // Bottom edge (excluding right corner)
        for x in ((-r)..r).rev() {
            chunks.push((center_x + x, center_z + r));
        }
        // Left edge (excluding corners)
        for z in ((-r + 1)..r).rev() {
            chunks.push((center_x - r, center_z + z));
        }
    }

    chunks
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_generation() {
        let chunks = generate_spiral(0, 0, 1);
        assert_eq!(chunks.len(), 9); // 3x3
        assert_eq!(chunks[0], (0, 0)); // Center first
    }

    #[test]
    fn test_spiral_radius_2() {
        let chunks = generate_spiral(0, 0, 2);
        assert_eq!(chunks.len(), 25); // 5x5
        assert_eq!(chunks[0], (0, 0));
    }

    #[test]
    fn test_loader_state_enqueue_dedup() {
        let mut state = ChunkLoaderState::new();
        state.center = Some((0, 0));
        state.radius = 2;

        state.enqueue((0, 0));
        state.enqueue((0, 0)); // Duplicate
        state.enqueue((1, 1));

        assert_eq!(state.queue.len(), 2);
    }

    #[test]
    fn test_loader_state_purge() {
        let mut state = ChunkLoaderState::new();
        state.center = Some((0, 0));
        state.radius = 2;

        // Queue some chunks - all within radius 2 of (0,0)
        state.enqueue((0, 0));
        state.enqueue((1, 1));
        state.enqueue((2, 2));

        assert_eq!(state.queue.len(), 3);

        // Move center to (5, 5) with radius 2
        // New view: x in [3,7], z in [3,7]
        // (0,0), (1,1), (2,2) are all outside this range
        state.center = Some((5, 5));

        let purged = state.purge_stale_chunks();
        assert_eq!(purged, 3); // All 3 chunks are now out of range
        assert_eq!(state.queue.len(), 0);
    }

    #[test]
    fn test_is_in_view() {
        let mut state = ChunkLoaderState::new();
        state.center = Some((0, 0));
        state.radius = 2;

        assert!(state.is_in_view(0, 0));
        assert!(state.is_in_view(2, 2));
        assert!(!state.is_in_view(3, 0));
    }
}
