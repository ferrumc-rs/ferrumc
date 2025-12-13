//! Async chunk loader task for per-player chunk streaming.
//!
//! Each player gets their own dedicated Tokio task that handles chunk loading.
//! This task sleeps (0% CPU) when the player is stationary and only wakes up
//! when it receives a command via the channel.
//!
//! This architecture keeps the main ECS tick fast by offloading chunk IO and
//! serialization to background tasks.

use ferrumc_components::chunks::ChunkCommand;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net::packets::outgoing::chunk_batch_finish::ChunkBatchFinish;
use ferrumc_net::packets::outgoing::chunk_batch_start::ChunkBatchStart;
use ferrumc_net::packets::outgoing::set_center_chunk::SetCenterChunk;
use ferrumc_state::GlobalState;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error, trace};

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

    // Main command loop - sleeps when no commands are pending
    while let Some(command) = rx.recv().await {
        match command {
            ChunkCommand::UpdateCenter {
                chunk_x,
                chunk_z,
                radius,
            } => {
                if let Err(()) =
                    handle_update_center(&conn, &state, &player_name, chunk_x, chunk_z, radius)
                {
                    // Connection failed, exit the task
                    break;
                }
            }
            ChunkCommand::BatchReceived(rate) => {
                // Client acknowledged a batch. This can be used for throttling.
                // For now, just log it. Later, this rate can be used to pace
                // chunk sending to match client processing speed.
                trace!(
                    "{} acknowledged chunk batch. Desired rate: {:.1} chunks/tick",
                    player_name,
                    rate
                );
            }
            ChunkCommand::Stop => {
                debug!("Stopping chunk loader for {}", player_name);
                break;
            }
        }
    }

    debug!("Chunk loader task ended for {}", player_name);
}

/// Handles the UpdateCenter command by loading and sending chunks to the player.
///
/// Returns `Err(())` if the connection is dead and the task should exit.
fn handle_update_center(
    conn: &StreamWriter,
    _state: &GlobalState,
    player_name: &str,
    chunk_x: i32,
    chunk_z: i32,
    radius: u8,
) -> Result<(), ()> {
    trace!(
        "{} moved to chunk ({}, {}). Loading radius: {}",
        player_name,
        chunk_x,
        chunk_z,
        radius
    );

    // 1. Send Set Center Chunk packet (required by Minecraft protocol)
    if conn
        .send_packet(SetCenterChunk::new(chunk_x, chunk_z))
        .is_err()
    {
        debug!(
            "Failed to send SetCenterChunk to {}, connection likely dead",
            player_name
        );
        return Err(());
    }

    // 2. Start the chunk batch
    if conn.send_packet(ChunkBatchStart {}).is_err() {
        return Err(());
    }

    // 3. Send chunks within the radius
    // TODO: For now, send flat bedrock chunks. Later, load from state.world
    // TODO: Track which chunks the player already has to avoid re-sending
    let mut chunk_count: i32 = 0;
    let r = radius as i32;

    for x in (chunk_x - r)..=(chunk_x + r) {
        for z in (chunk_z - r)..=(chunk_z + r) {
            // Create a flat chunk for testing
            // TODO: Replace with actual world data: state.world.load_chunk(...)
            // 8 sections = Y -64 to 64
            match ChunkAndLightData::flat(x, z, 8) {
                Ok(packet) => {
                    if conn.send_packet(packet).is_err() {
                        debug!(
                            "Failed to send chunk ({}, {}) to {}, connection likely dead",
                            x, z, player_name
                        );
                        return Err(());
                    }
                    chunk_count += 1;
                }
                Err(e) => {
                    error!("Failed to create chunk packet for ({}, {}): {:?}", x, z, e);
                }
            }
        }
    }

    // 4. Finish the chunk batch
    if conn
        .send_packet(ChunkBatchFinish {
            batch_size: chunk_count.into(),
        })
        .is_err()
    {
        return Err(());
    }

    trace!(
        "Sent {} chunks to {} centered at ({}, {})",
        chunk_count,
        player_name,
        chunk_x,
        chunk_z
    );

    Ok(())
}
