use bevy_ecs::prelude::*;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::BinaryError;
use ferrumc_components::player::abilities::PlayerAbilities;
use ferrumc_components::player::gameplay_state::digging::PlayerDigging;
use ferrumc_data::blocks::types::Block;
use ferrumc_events::player_digging::*;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::{block_change_ack::BlockChangeAck, block_update::BlockUpdate};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use tracing::{debug, error, trace, warn};

// A query for just the components needed to acknowledge a dig packet
type DiggingPlayerQuery<'a> = (Entity, &'a StreamWriter, Option<&'a PlayerDigging>);

/// Handles the PlayerStartDiggingEvent.
/// This system starts the digging timer.
pub fn handle_start_digging(
    mut commands: Commands,
    mut events: EventReader<PlayerStartDiggingEvent>,
    mut player_query: Query<DiggingPlayerQuery, With<PlayerAbilities>>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        debug!(
            "Player {:?} started digging at {:?}",
            event.player, event.position
        );

        // --- 1. Get BlockStateId from the world ---
        let block_state_id = match state.0.world.get_block_and_fetch(
            event.position.x,
            event.position.y as i32,
            event.position.z,
            "overworld", // TODO: remove hardcoded dimension
        ) {
            Ok(id) => id,
            Err(e) => {
                warn!(
                    "StartDigging: Failed to get block at {:?}: {:?}",
                    event.position, e
                );
                continue;
            }
        };
        // --- 2. Get Block Name ---
        let Some(block_name) =
            ferrumc_registry::lookup_blockstate_name(&VarInt::from(block_state_id).0.to_string())
        else {
            warn!("Could not find block name for state {:?}", block_state_id);
            continue;
        };

        // --- 3. Get Hardness ---
        let block_id_u32 = block_state_id.0;

        // Get Hardness directly using the ID
        let Some(block_data) = Block::by_id(block_id_u32) else {
            warn!(
                "Could not find block data for BlockStateId: {}",
                block_id_u32
            );
            continue;
        };

        let hardness = block_data.hardness;

        // --- 4. Check for unbreakable block ---
        if hardness < 0.0 {
            debug!(
                "Player {:?} tried to dig an unbreakable block ({})",
                event.player, block_name
            );

            // We must still send an ACK to the client.
            // But we do not add the PlayerDigging component.
            if let Ok((_, writer, _)) = player_query.get_mut(event.player) {
                let ack_packet = BlockChangeAck {
                    sequence: event.sequence,
                };
                if let Err(e) = writer.send_packet_ref(&ack_packet) {
                    error!(
                        "Failed to send start_dig ACK to {:?}: {:?}",
                        event.player, e
                    );
                }
            }
            continue; // Move to the next event
        }

        // --- 5. Calculate break time ---
        // TODO: This is a placeholder. A real calculation would
        // check for tools, effects, etc.
        let break_time = if hardness == 0.0 {
            // Instabreak blocks like air, grass, flowers
            Duration::from_millis(0)
        } else {
            // Placeholder: 1.5s per hardness
            // TODO: replace with real formula
            Duration::from_secs_f32(hardness * 1.5)
        };

        // --- 6. Add the component ----
        commands.entity(event.player).insert(PlayerDigging {
            block_pos: event.position.clone(),
            start_time: Instant::now(),
            break_time,
        });

        // --- 7. Acknowledge the client ---
        if let Ok((_, writer, _)) = player_query.get_mut(event.player) {
            let ack_packet = BlockChangeAck {
                sequence: event.sequence,
            };
            if let Err(e) = writer.send_packet_ref(&ack_packet) {
                error!(
                    "Failed to send start_dig ACK to {:?}: {:?}",
                    event.player, e
                );
            }
        }
    }
}

/// Handles the PlayerCancelDiggingEvent.
/// This system stops the digging timer.
pub fn handle_cancel_digging(
    mut commands: Commands,
    mut events: EventReader<PlayerCancelDiggingEvent>,
    mut player_query: Query<DiggingPlayerQuery>,
) {
    for event in events.read() {
        debug!("Player {:?} cancelled digging.", event.player);

        // Remove the component to stop the timer.
        commands.entity(event.player).remove::<PlayerDigging>();

        // Acknowledge the cancellation.
        if let Ok((_, writer, _)) = player_query.get_mut(event.player) {
            let ack_packet = BlockChangeAck {
                sequence: event.sequence,
            };
            if let Err(e) = writer.send_packet_ref(&ack_packet) {
                error!(
                    "Failed to send cancel_dig ACK to {:?}: {:?}",
                    event.player, e
                );
            }
        }
    }
}

/// Handles the PlayerFinishDiggingEvent.
/// This system checks the timer and breaks the block.
pub fn handle_finish_digging(
    mut commands: Commands,
    mut events: EventReader<PlayerFinishDiggingEvent>,
    state: Res<GlobalStateResource>,
    mut player_query: Query<DiggingPlayerQuery>,
    broadcast_query: Query<(Entity, &StreamWriter)>, // For broadcasting the break
) {
    for event in events.read() {
        let Ok((_player_entity, writer, digging_opt)) = player_query.get_mut(event.player) else {
            warn!(
                "Player {:?} sent FinishDigging but query failed.",
                event.player
            );
            continue;
        };

        // Check if the player was actually digging
        let Some(digging) = digging_opt else {
            warn!(
                "Player {:?} finished digging without starting.",
                event.player
            );
            let ack_packet = BlockChangeAck {
                sequence: event.sequence,
            };
            if let Err(e) = writer.send_packet_ref(&ack_packet) {
                error!("Failed to send fail_dig ACK to {:?}: {:?}", event.player, e);
            }
            continue;
        };

        // --- 1. Validate the Dig ---
        if digging.block_pos != event.position {
            warn!(
                "Player {:?} finished digging the wrong block. (Expected {:?}, got {:?})",
                event.player, digging.block_pos, event.position
            );
            // Don't break the block, but still ACK
            let ack_packet = BlockChangeAck {
                sequence: event.sequence,
            };
            if let Err(e) = writer.send_packet_ref(&ack_packet) {
                error!("Failed to send fail_dig ACK to {:?}: {:?}", event.player, e);
            }
            commands.entity(event.player).remove::<PlayerDigging>();
            continue;
        }

        let elapsed = Instant::now().duration_since(digging.start_time);

        // --- 2. Check if enough time has passed ---
        if elapsed < digging.break_time {
            // --- ANTI-CHEAT ---
            warn!(
                "Player {:?} finished digging too fast! ({}ms < {}ms)",
                event.player,
                elapsed.as_millis(),
                digging.break_time.as_millis()
            );

            let real_block_state = match state.0.world.get_block_and_fetch(
                event.position.x,
                event.position.y as i32,
                event.position.z,
                "overworld",
            ) {
                Ok(id) => id,
                Err(e) => {
                    error!(
                        "Failed to get real block state for anti-cheat revert: {:?}",
                        e
                    );
                    BlockStateId::default()
                }
            };

            let revert_packet = BlockUpdate {
                location: event.position.clone(),
                block_state_id: VarInt::from(real_block_state),
            };

            if let Err(e) = writer.send_packet_ref(&revert_packet) {
                error!(
                    "Failed to send anti-cheat revert packet to {:?}: {:?}",
                    event.player, e
                );
            }
        } else {
            // --- 3. SUCCESS: Break the Block ---
            debug!(
                "Player {:?} finished digging at {:?}",
                event.player, event.position
            );

            // We wrap the block-breaking logic in its own function
            // to handle the errors cleanly (replaces `try` block).
            if let Err(e) = break_block(&state, &broadcast_query, &event.position) {
                error!("Error handling finished digging: {:?}", e);
            }
        }

        // --- 4. Acknowledge and Clean up (This now runs for *both* cases) ---
        let ack_packet = BlockChangeAck {
            sequence: event.sequence,
        };
        if let Err(e) = writer.send_packet_ref(&ack_packet) {
            error!(
                "Failed to send finish_dig ACK to {:?}: {:?}",
                event.player, e
            );
        }
        commands.entity(event.player).remove::<PlayerDigging>();
    }
}

/// Helper function to contain the block-breaking logic (replaces `try` block)
fn break_block(
    state: &Res<GlobalStateResource>,
    broadcast_query: &Query<(Entity, &StreamWriter)>,
    position: &ferrumc_net_codec::net_types::network_position::NetworkPosition,
) -> Result<(), BinaryError> {
    let mut chunk =
        match state
            .0
            .clone()
            .world
            .load_chunk_owned(position.x >> 4, position.z >> 4, "overworld")
        {
            Ok(chunk) => chunk,
            Err(e) => {
                trace!("Chunk not found, generating new chunk: {:?}", e);
                state
                    .0
                    .clone()
                    .terrain_generator
                    .generate_chunk(position.x >> 4, position.z >> 4)
                    .map_err(BinaryError::WorldGen)?
            }
        };
    let (relative_x, relative_y, relative_z) = (
        position.x.abs() % 16,
        position.y as i32,
        position.z.abs() % 16,
    );
    chunk
        .set_block(relative_x, relative_y, relative_z, BlockStateId::default())
        .map_err(BinaryError::World)?;
    state
        .0
        .world
        .save_chunk(Arc::new(chunk))
        .map_err(BinaryError::World)?;

    // Broadcast the block break to all players
    let block_update_packet = BlockUpdate {
        location: position.clone(),
        block_state_id: VarInt::from(BlockStateId::default()),
    };
    for (eid, conn) in broadcast_query {
        if !state.0.players.is_connected(eid) {
            continue;
        }
        conn.send_packet_ref(&block_update_packet)
            .map_err(BinaryError::Net)?;
    }
    Ok(())
}
