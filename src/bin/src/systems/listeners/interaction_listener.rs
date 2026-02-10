use bevy_ecs::prelude::*;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::transform::position::Position;
use ferrumc_messages::{BlockCoords, BlockInteractMessage};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::BlockPos;
use tracing::{debug, error};

use crate::systems::interaction::block_interactions::{try_interact, InteractionResult};

pub fn handle_block_interact(
    mut events: MessageReader<BlockInteractMessage>,
    state: Res<GlobalStateResource>,
    player_query: Query<(Entity, &StreamWriter, &Position)>,
) {
    for event in events.read() {
        let pos = BlockPos::of(event.position.x, event.position.y, event.position.z);

        // 1. Load the chunk
        let chunk_result =
            ferrumc_utils::world::load_or_generate_mut(&state.0, pos.chunk(), "overworld");
        let mut chunk = match chunk_result {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to load chunk for interaction: {:?}", e);
                continue;
            }
        };

        // 2. Try to interact with the clicked block
        let clicked_state = chunk.get_block(pos.chunk_block_pos());
        let new_state = match try_interact(clicked_state) {
            InteractionResult::Toggled(s) => s,
            InteractionResult::NotInteractive | InteractionResult::InvalidBlock => continue,
        };

        debug!(
            "Player {:?} toggled block at ({}, {}, {}) -> new state: {}",
            event.player, pos.pos.x, pos.pos.y, pos.pos.z, new_state
        );

        // 3. Update the block in the chunk
        chunk.set_block(pos.chunk_block_pos(), new_state);

        // 4. Handle door other half
        let other_half_update = handle_door_other_half(&mut chunk, new_state, &pos);

        // Drop chunk lock before iterating players
        drop(chunk);

        // 5. Send ACK to the player who clicked
        if let Ok((_, conn, _)) = player_query.get(event.player) {
            let ack = BlockChangeAck {
                sequence: event.sequence,
            };
            if let Err(e) = conn.send_packet_ref(&ack) {
                error!("Failed to send block change ack: {:?}", e);
            }
        }

        // 6. Broadcast block updates to nearby players
        let primary = BlockUpdate {
            location: NetworkPosition {
                x: pos.pos.x,
                y: pos.pos.y as i16,
                z: pos.pos.z,
            },
            block_state_id: VarInt::from(new_state),
        };

        broadcast_block_updates_nearby(
            &player_query,
            &BlockCoords {
                x: pos.pos.x,
                y: pos.pos.y,
                z: pos.pos.z,
            },
            &primary,
            other_half_update.as_ref(),
        );
    }
}

/// If the toggled block is a door, also toggle the other half and return its BlockUpdate.
fn handle_door_other_half(
    chunk: &mut ferrumc_world::MutChunk,
    new_state: BlockStateId,
    pos: &BlockPos,
) -> Option<BlockUpdate> {
    let data = new_state.to_block_data()?;
    if !data.name.ends_with("_door") {
        return None;
    }
    let props = data.properties.as_ref()?;
    let half = props.get("half")?;
    let y_offset: i32 = match half.as_str() {
        "lower" => 1,
        "upper" => -1,
        _ => return None,
    };

    let other_pos = *pos + (0, y_offset, 0);
    let other_state = chunk.get_block(other_pos.chunk_block_pos());
    if let InteractionResult::Toggled(other_new) = try_interact(other_state) {
        chunk.set_block(other_pos.chunk_block_pos(), other_new);
        debug!(
            "Also toggled other door half at ({}, {}, {}) -> {}",
            other_pos.pos.x, other_pos.pos.y, other_pos.pos.z, other_new
        );
        Some(BlockUpdate {
            location: NetworkPosition {
                x: other_pos.pos.x,
                y: other_pos.pos.y as i16,
                z: other_pos.pos.z,
            },
            block_state_id: VarInt::from(other_new),
        })
    } else {
        None
    }
}

/// Broadcast one or two BlockUpdate packets to all players within render distance.
fn broadcast_block_updates_nearby(
    query: &Query<(Entity, &StreamWriter, &Position)>,
    block_pos: &BlockCoords,
    primary: &BlockUpdate,
    secondary: Option<&BlockUpdate>,
) {
    let block_chunk_x = block_pos.x >> 4;
    let block_chunk_z = block_pos.z >> 4;
    let render_distance = get_global_config().chunk_render_distance as i32;

    for (_, conn, player_pos) in query.iter() {
        let player_chunk = player_pos.chunk();
        let (pcx, pcz) = (player_chunk.x, player_chunk.y);

        if (block_chunk_x - pcx).abs() <= render_distance
            && (block_chunk_z - pcz).abs() <= render_distance
        {
            if let Err(e) = conn.send_packet_ref(primary) {
                error!("Failed to send block update: {:?}", e);
            }
            if let Some(sec) = secondary {
                if let Err(e) = conn.send_packet_ref(sec) {
                    error!("Failed to send block update: {:?}", e);
                }
            }
        }
    }
}
