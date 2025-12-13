//! Async chunk loader task for per-player chunk streaming.
//!
//! Each player gets their own dedicated Tokio task that handles chunk loading.
//! This task sleeps (0% CPU) when the player is stationary and only wakes up
//! when it receives a command via the channel.
//!
//! ## Architecture
//!
//! The chunk loader implements a "Sliding Window" algorithm:
//! - On first join, all chunks in the view radius are queued in spiral order
//! - On movement, only the delta (new chunks to load, old chunks to unload) is processed
//! - Chunks are sent in batches, waiting for client acknowledgment between batches
//! - This prevents overwhelming the client at high render distances (e.g., 32)
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
//!   |-- (next batch if queue) ----->|
//! ```

use ferrumc_components::chunks::ChunkCommand;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::chunk_batch_finish::ChunkBatchFinish;
use ferrumc_net::packets::outgoing::chunk_batch_start::ChunkBatchStart;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_net::packets::outgoing::unload_chunk::UnloadChunk;
use ferrumc_state::GlobalState;
use std::collections::VecDeque;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error, trace};

/// Number of vertical sections in a chunk (controls Y range).
/// 24 sections = Y -64 to 320 (full 1.18+ world height)
/// Although, the 8 here represents the default flat world height.
///  8 => Y -64 to 64. That's the bedrock floor. 
const CHUNK_SECTIONS: usize = 8;

/// Default chunks per batch before we receive client feedback.
const DEFAULT_CHUNKS_PER_BATCH: f32 = 10.0;

/// Maximum chunks per batch to prevent packet flooding.
/// 100 is basically 10*10 so like a square of 10x10 chunks at once.
const MAX_CHUNKS_PER_BATCH: f32 = 100.0;

/// Minimum chunks per batch to ensure progress.
const MIN_CHUNKS_PER_BATCH: f32 = 1.0;

/// Internal state for the chunk loader task.
struct ChunkLoaderState {
    /// Last known center position (None = first join)
    last_center: Option<(i32, i32)>,
    /// Current view radius
    current_radius: u8,
    /// Queue of chunks waiting to be sent
    queue: VecDeque<(i32, i32)>,
    /// Number of chunks to send per batch (dynamically adjusted)
    chunks_per_batch: f32,
    /// Whether we're waiting for a batch acknowledgment
    awaiting_ack: bool,
}

impl ChunkLoaderState {
    fn new() -> Self {
        Self {
            last_center: None,
            current_radius: 0,
            queue: VecDeque::new(),
            chunks_per_batch: DEFAULT_CHUNKS_PER_BATCH,
            awaiting_ack: false,
        }
    }
}

/// Async task that handles chunk loading for a single player.
///
/// This task runs independently of the ECS tick and processes chunk commands
/// sent via the channel. When no commands are pending, it sleeps (0% CPU).
///
/// # Arguments
/// * `rx` - Receiver end of the channel for incoming `ChunkCommand`s
/// * `conn` - Clone of the player's `StreamWriter` for sending packets
/// * `state` - Global server state for accessing world data
/// * `player_name` - Player's username for logging purposes
pub async fn chunk_loader_task(
    mut rx: Receiver<ChunkCommand>,
    conn: StreamWriter,
    state: GlobalState,
    player_name: String,
) {
    debug!("Async chunk loader started for {}", player_name);

    let mut loader_state = ChunkLoaderState::new();

    // Main command loop - sleeps when no commands are pending
    while let Some(command) = rx.recv().await {
        match command {
            ChunkCommand::UpdateCenter {
                chunk_x,
                chunk_z,
                radius,
            } => {
                if let Err(()) = handle_update_center(
                    &conn,
                    &state,
                    &player_name,
                    &mut loader_state,
                    chunk_x,
                    chunk_z,
                    radius,
                ) {
                    break;
                }

                // Try to send a batch if we're not waiting for ack
                if !loader_state.awaiting_ack && !loader_state.queue.is_empty() {
                    if let Err(()) = send_chunk_batch(&conn, &state, &player_name, &mut loader_state)
                    {
                        break;
                    }
                }
            }
            ChunkCommand::BatchReceived(desired_rate) => {
                debug!(
                    "[{}] Client acknowledged batch. Desired rate: {:.1} chunks/tick",
                    player_name, desired_rate
                );
                handle_batch_received(&mut loader_state, desired_rate);

                // Send next batch if queue has items
                if !loader_state.queue.is_empty() {
                    if let Err(()) = send_chunk_batch(&conn, &state, &player_name, &mut loader_state)
                    {
                        break;
                    }
                }
            }
            ChunkCommand::Stop => {
                debug!("Stopping chunk loader for {}", player_name);
                break;
            }
        }
    }

    debug!("Chunk loader task ended for {}", player_name);
}

/// Handles the UpdateCenter command by calculating delta and queueing new chunks.
fn handle_update_center(
    conn: &StreamWriter,
    _state: &GlobalState,
    player_name: &str,
    loader_state: &mut ChunkLoaderState,
    new_x: i32,
    new_z: i32,
    new_radius: u8,
) -> Result<(), ()> {
    trace!(
        "{} moved to chunk ({}, {}). Radius: {}",
        player_name,
        new_x,
        new_z,
        new_radius
    );

    // Send SetCenterChunk first (required by protocol)
    if conn.send_packet(SetCenterChunk::new(new_x, new_z)).is_err() {
        debug!(
            "Failed to send SetCenterChunk to {}, connection dead",
            player_name
        );
        return Err(());
    }

    match loader_state.last_center {
        None => {
            // First join - load all chunks in spiral order
            let chunks = generate_spiral_chunks(new_x, new_z, new_radius as i32);
            debug!(
                "{} first join: queueing {} chunks in spiral order",
                player_name,
                chunks.len()
            );
            loader_state.queue.extend(chunks);
        }
        Some((old_x, old_z)) => {
            // Movement - calculate delta
            let old_radius = loader_state.current_radius as i32;
            let (to_load, to_unload) =
                calculate_chunk_delta(old_x, old_z, old_radius, new_x, new_z, new_radius as i32);

            // Unload old chunks immediately (cheap operation)
            if !to_unload.is_empty() {
                trace!(
                    "{}: unloading {} chunks, loading {} new chunks",
                    player_name,
                    to_unload.len(),
                    to_load.len()
                );
                for (cx, cz) in to_unload {
                    if conn.send_packet(UnloadChunk::new(cx, cz)).is_err() {
                        debug!("Failed to send UnloadChunk to {}", player_name);
                        return Err(());
                    }
                }
            }

            // Queue new chunks (sorted by distance to new center)
            let mut sorted_load = to_load;
            sorted_load.sort_by_key(|(cx, cz)| {
                let dx = cx - new_x;
                let dz = cz - new_z;
                dx * dx + dz * dz
            });
            loader_state.queue.extend(sorted_load);
        }
    }

    // Update state
    loader_state.last_center = Some((new_x, new_z));
    loader_state.current_radius = new_radius;

    Ok(())
}

/// Handles BatchReceived by updating the send rate.
fn handle_batch_received(loader_state: &mut ChunkLoaderState, desired_rate: f32) {
    loader_state.awaiting_ack = false;

    // Clamp the rate to reasonable bounds
    let new_rate = desired_rate.clamp(MIN_CHUNKS_PER_BATCH, MAX_CHUNKS_PER_BATCH);

    debug!(
        "Adjusting batch rate: client requested {:.1}, clamped to {:.1}",
        desired_rate,
        new_rate
    );

    loader_state.chunks_per_batch = new_rate;
}

/// Sends a batch of chunks from the queue.
fn send_chunk_batch(
    conn: &StreamWriter,
    _state: &GlobalState,
    player_name: &str,
    loader_state: &mut ChunkLoaderState,
) -> Result<(), ()> {
    if loader_state.queue.is_empty() {
        return Ok(());
    }

    // Calculate how many chunks to send this batch
    let batch_size = (loader_state.chunks_per_batch.ceil() as usize).min(loader_state.queue.len());

    debug!(
        "[{}] Sending batch: {} chunks (rate: {:.1}, queue: {})",
        player_name,
        batch_size,
        loader_state.chunks_per_batch,
        loader_state.queue.len()
    );

    // Start the batch
    if conn.send_packet(ChunkBatchStart {}).is_err() {
        debug!("Failed to send ChunkBatchStart to {}", player_name);
        return Err(());
    }

    // Send chunks
    let mut sent_count: i32 = 0;
    for _ in 0..batch_size {
        if let Some((cx, cz)) = loader_state.queue.pop_front() {
            // TODO: Load actual chunk data from state.world
            // For now, use flat chunks for testing
            match ChunkAndLightData::flat(cx, cz, CHUNK_SECTIONS) {
                Ok(packet) => {
                    if conn.send_packet(packet).is_err() {
                        debug!(
                            "Failed to send chunk ({}, {}) to {}, connection dead",
                            cx, cz, player_name
                        );
                        return Err(());
                    }
                    sent_count += 1;
                }
                Err(e) => {
                    error!("Failed to create chunk packet for ({}, {}): {:?}", cx, cz, e);
                }
            }
        }
    }

    // Finish the batch
    if conn
        .send_packet(ChunkBatchFinish {
            batch_size: sent_count.into(),
        })
        .is_err()
    {
        debug!("Failed to send ChunkBatchFinish to {}", player_name);
        return Err(());
    }

    debug!(
        "[{}] Batch sent: {} chunks delivered, {} remaining. Awaiting client ACK...",
        player_name,
        sent_count,
        loader_state.queue.len()
    );

    // Mark that we're waiting for acknowledgment
    loader_state.awaiting_ack = true;

    Ok(())
}

/// Generates chunks in spiral order from center outward.
///
/// This ensures the player sees chunks directly under and around them first,
/// providing the best perceived loading experience.
fn generate_spiral_chunks(center_x: i32, center_z: i32, radius: i32) -> Vec<(i32, i32)> {
    let mut chunks = Vec::with_capacity(((radius * 2 + 1) * (radius * 2 + 1)) as usize);

    // Start at center
    chunks.push((center_x, center_z));

    // Spiral outward
    for r in 1..=radius {
        // Top edge (left to right)
        for x in -r..=r {
            chunks.push((center_x + x, center_z - r));
        }
        // Right edge (top to bottom, excluding top corner)
        for z in (-r + 1)..=r {
            chunks.push((center_x + r, center_z + z));
        }
        // Bottom edge (right to left, excluding right corner)
        for x in ((-r)..r).rev() {
            chunks.push((center_x + x, center_z + r));
        }
        // Left edge (bottom to top, excluding both corners)
        for z in ((-r + 1)..r).rev() {
            chunks.push((center_x - r, center_z + z));
        }
    }

    chunks
}

/// Calculates which chunks need to be loaded and unloaded when moving.
///
/// Returns (chunks_to_load, chunks_to_unload).
fn calculate_chunk_delta(
    old_x: i32,
    old_z: i32,
    old_radius: i32,
    new_x: i32,
    new_z: i32,
    new_radius: i32,
) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut to_load = Vec::new();
    let mut to_unload = Vec::new();

    // Check all chunks in the new radius - if not in old radius, need to load
    for x in (new_x - new_radius)..=(new_x + new_radius) {
        for z in (new_z - new_radius)..=(new_z + new_radius) {
            if !is_in_radius(x, z, old_x, old_z, old_radius) {
                to_load.push((x, z));
            }
        }
    }

    // Check all chunks in the old radius - if not in new radius, need to unload
    for x in (old_x - old_radius)..=(old_x + old_radius) {
        for z in (old_z - old_radius)..=(old_z + old_radius) {
            if !is_in_radius(x, z, new_x, new_z, new_radius) {
                to_unload.push((x, z));
            }
        }
    }

    (to_load, to_unload)
}

/// Checks if a chunk is within the given radius of a center point.
#[inline]
fn is_in_radius(chunk_x: i32, chunk_z: i32, center_x: i32, center_z: i32, radius: i32) -> bool {
    chunk_x >= center_x - radius
        && chunk_x <= center_x + radius
        && chunk_z >= center_z - radius
        && chunk_z <= center_z + radius
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_generation() {
        let chunks = generate_spiral_chunks(0, 0, 1);
        // Should be 9 chunks for radius 1: center + 8 surrounding
        assert_eq!(chunks.len(), 9);
        // First chunk should be center
        assert_eq!(chunks[0], (0, 0));
    }

    #[test]
    fn test_spiral_order() {
        let chunks = generate_spiral_chunks(0, 0, 2);
        // Should be 25 chunks for radius 2
        assert_eq!(chunks.len(), 25);
        // First is center
        assert_eq!(chunks[0], (0, 0));
    }

    #[test]
    fn test_chunk_delta_no_movement() {
        let (load, unload) = calculate_chunk_delta(0, 0, 2, 0, 0, 2);
        assert!(load.is_empty());
        assert!(unload.is_empty());
    }

    #[test]
    fn test_chunk_delta_movement() {
        // Move one chunk east
        let (load, unload) = calculate_chunk_delta(0, 0, 2, 1, 0, 2);
        // Should load 5 new chunks on the east edge
        assert_eq!(load.len(), 5);
        // Should unload 5 old chunks on the west edge
        assert_eq!(unload.len(), 5);
    }

    #[test]
    fn test_is_in_radius() {
        assert!(is_in_radius(0, 0, 0, 0, 2));
        assert!(is_in_radius(2, 2, 0, 0, 2));
        assert!(!is_in_radius(3, 0, 0, 0, 2));
    }
}
