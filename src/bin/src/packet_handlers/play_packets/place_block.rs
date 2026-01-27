use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::collisions::bounds::CollisionBounds;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net::PlaceBlockReceiver;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::pos::BlockPos;
use ferrumc_world::vanilla_chunk_format::BlockData;
use tracing::{debug, error, trace};

use ferrumc_config::server_config::get_global_config;
use ferrumc_core::mq;
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_text::{Color, NamedColor, TextComponentBuilder};
use ferrumc_world::block_state_id::BlockStateId;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;

const ITEM_TO_BLOCK_MAPPING_FILE: &str =
    include_str!("../../../../../assets/data/item_to_block_mapping.json");
static ITEM_TO_BLOCK_MAPPING: Lazy<HashMap<i32, BlockStateId>> = Lazy::new(|| {
    let str_form: HashMap<String, String> = serde_json::from_str(ITEM_TO_BLOCK_MAPPING_FILE)
        .expect("Failed to parse item_to_block_mapping.json");
    str_form
        .into_iter()
        .map(|(k, v)| {
            (
                i32::from_str(&k).unwrap(),
                BlockStateId::new(u32::from_str(&v).unwrap()),
            )
        })
        .collect()
});

pub fn handle(
    receiver: Res<PlaceBlockReceiver>,
    state: Res<GlobalStateResource>,
    query: Query<(Entity, &StreamWriter, &Inventory, &Hotbar, &Position)>,
    pos_q: Query<(&Position, &CollisionBounds)>,
) {
    'ev_loop: for (event, eid) in receiver.0.try_iter() {
        let Ok((entity, conn, inventory, hotbar, _)) = query.get(eid) else {
            debug!("Could not get connection for entity {:?}", eid);
            continue;
        };
        if !state.0.players.is_connected(entity) {
            trace!("Entity {:?} is not connected", entity);
            continue;
        }

        // First, check if the clicked block is interactive (like a door)
        let clicked_pos: BlockPos = event.position.clone().into();
        let clicked_block_state = state
            .0
            .world
            .get_block_and_fetch(clicked_pos, "overworld")
            .unwrap_or_default();
        let clicked_block_data = clicked_block_state.to_block_data();

        // Handle door interaction
        if let Some(ref data) = clicked_block_data {
            if data.name.ends_with("_door")
                && handle_door_interaction(&state, &query, conn, &event, &clicked_pos, data)
            {
                continue 'ev_loop;
            }
        }

        match event.hand.0 {
            0 => {
                let Ok(slot) = hotbar.get_selected_item(inventory) else {
                    error!("Could not fetch {:?}", eid);
                    continue 'ev_loop;
                };
                if let Some(selected_item) = slot {
                    let Some(item_id) = selected_item.item_id else {
                        error!("Selected item has no item ID");
                        continue 'ev_loop;
                    };
                    let Some(mapped_block_state_id) = ITEM_TO_BLOCK_MAPPING.get(&item_id.0 .0)
                    else {
                        // No block mapping - could be a non-placeable item, just ack and continue
                        let ack_packet = BlockChangeAck {
                            sequence: event.sequence,
                        };
                        let _ = conn.send_packet_ref(&ack_packet);
                        continue 'ev_loop;
                    };
                    debug!(
                        "Placing block with item ID: {}, mapped to block state ID: {}",
                        item_id.0, mapped_block_state_id
                    );
                    let pos: BlockPos = event.position.into();
                    if pos.pos.y >= 319 {
                        mq::queue(
                            TextComponentBuilder::new(
                                "Build limit is 319! Cannot place block here.".to_string(),
                            )
                            .color(Color::Named(NamedColor::Red))
                            .bold()
                            .build(),
                            true,
                            entity,
                        );
                        trace!("Block placement out of bounds: {}", pos);
                        continue 'ev_loop;
                    } else if pos.pos.y <= -64 {
                        mq::queue(
                            TextComponentBuilder::new(
                                "Cannot place block below Y=-64.".to_string(),
                            )
                            .color(Color::Named(NamedColor::Red))
                            .bold()
                            .build(),
                            true,
                            entity,
                        );
                        trace!("Block placement out of bounds: {}", pos);
                        continue 'ev_loop;
                    }
                    let offset_pos = pos
                        + match event.face.0 {
                            0 => (0, -1, 0),
                            1 => (0, 1, 0),
                            2 => (0, 0, -1),
                            3 => (0, 0, 1),
                            4 => (-1, 0, 0),
                            5 => (1, 0, 0),
                            _ => (0, 0, 0),
                        };

                    // Check if the block collides with any entities
                    let does_collide = {
                        pos_q.into_iter().any(|(pos, bounds)| {
                            bounds.collides(
                                (pos.x, pos.y, pos.z),
                                &CollisionBounds {
                                    x_offset_start: 0.0,
                                    x_offset_end: 1.0,
                                    y_offset_start: 0.0,
                                    y_offset_end: 1.0,
                                    z_offset_start: 0.0,
                                    z_offset_end: 1.0,
                                },
                                (
                                    offset_pos.pos.x as f64,
                                    offset_pos.pos.y as f64,
                                    offset_pos.pos.z as f64,
                                ),
                            )
                        })
                    };

                    if does_collide {
                        trace!("Block placement collided with entity");
                        continue 'ev_loop;
                    }

                    // Place the lower half in its own scope to release the lock
                    {
                        let mut chunk = ferrumc_utils::world::load_or_generate_mut(
                            &state.0,
                            offset_pos.chunk(),
                            "overworld",
                        )
                        .expect("Failed to load or generate chunk");
                        chunk.set_block(offset_pos.chunk_block_pos(), *mapped_block_state_id);
                    } // Lock released here

                    // Handle door placement - doors are two blocks tall
                    // When placing the lower half, we need to also place the upper half
                    let block_data = mapped_block_state_id.to_block_data();
                    let upper_door_packet = if let Some(ref data) = block_data {
                        if data.name.ends_with("_door") {
                            if let Some(ref props) = data.properties {
                                if props.get("half").map(|h| h == "lower").unwrap_or(false) {
                                    // Create the upper half with same properties but half=upper
                                    let mut upper_props = props.clone();
                                    upper_props.insert("half".to_string(), "upper".to_string());
                                    let upper_block_data = BlockData {
                                        name: data.name.clone(),
                                        properties: Some(upper_props),
                                    };
                                    let upper_block_id =
                                        BlockStateId::from_block_data(&upper_block_data);

                                    // Place the upper half one block above
                                    let upper_pos = offset_pos + (0, 1, 0);

                                    // Load chunk for upper position in its own scope
                                    {
                                        let mut upper_chunk =
                                            ferrumc_utils::world::load_or_generate_mut(
                                                &state.0,
                                                upper_pos.chunk(),
                                                "overworld",
                                            )
                                            .expect(
                                                "Failed to load or generate chunk for upper door",
                                            );
                                        upper_chunk
                                            .set_block(upper_pos.chunk_block_pos(), upper_block_id);
                                    } // Lock released here

                                    debug!(
                                        "Placed upper door half at ({}, {}, {}) with state {}",
                                        upper_pos.pos.x,
                                        upper_pos.pos.y,
                                        upper_pos.pos.z,
                                        upper_block_id
                                    );

                                    Some(BlockUpdate {
                                        location: NetworkPosition {
                                            x: upper_pos.pos.x,
                                            y: upper_pos.pos.y as i16,
                                            z: upper_pos.pos.z,
                                        },
                                        block_state_id: VarInt::from(upper_block_id),
                                    })
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    let ack_packet = BlockChangeAck {
                        sequence: event.sequence,
                    };

                    let chunk_packet = BlockUpdate {
                        location: NetworkPosition {
                            x: offset_pos.pos.x,
                            y: offset_pos.pos.y as i16,
                            z: offset_pos.pos.z,
                        },
                        block_state_id: VarInt::from(*mapped_block_state_id),
                    };

                    if let Err(err) = conn.send_packet_ref(&ack_packet) {
                        error!("Failed to send block change ack packet: {:?}", err);
                        continue 'ev_loop;
                    }

                    let offset_chunk = offset_pos.chunk();
                    let (offset_chunk_x, offset_chunk_z) = (offset_chunk.x(), offset_chunk.z());
                    let render_distance = get_global_config().chunk_render_distance as i32;
                    for (_, conn, _, _, pos) in query.iter() {
                        let chunk = pos.chunk();
                        let (chunk_x, chunk_z) = (chunk.x, chunk.y);

                        // Only send block update if the player is within the render distance of the block being updated
                        if (offset_chunk_x - chunk_x).abs() <= render_distance
                            && (offset_chunk_z - chunk_z).abs() <= render_distance
                        {
                            if let Err(err) = conn.send_packet_ref(&chunk_packet) {
                                error!("Failed to send block update packet: {:?}", err);
                            }
                            // Also send the upper door half packet if we placed a door
                            if let Some(ref upper_packet) = upper_door_packet {
                                if let Err(err) = conn.send_packet_ref(upper_packet) {
                                    error!(
                                        "Failed to send upper door block update packet: {:?}",
                                        err
                                    );
                                }
                            }
                        }
                    }
                }
            }
            1 => {
                trace!("Offhand block placement not implemented");
            }
            _ => {
                debug!("Invalid hand");
            }
        }
    }
}

/// Handle door interaction (open/close)
/// Returns true if the interaction was handled, false otherwise
fn handle_door_interaction(
    state: &Res<GlobalStateResource>,
    query: &Query<(Entity, &StreamWriter, &Inventory, &Hotbar, &Position)>,
    conn: &StreamWriter,
    event: &ferrumc_net::packets::incoming::place_block::PlaceBlock,
    clicked_pos: &BlockPos,
    door_data: &BlockData,
) -> bool {
    let Some(ref props) = door_data.properties else {
        return false;
    };

    // Toggle the open state
    let current_open = props.get("open").map(|s| s == "true").unwrap_or(false);
    let new_open = if current_open { "false" } else { "true" };

    // Create new block data with toggled open state
    let mut new_props = props.clone();
    new_props.insert("open".to_string(), new_open.to_string());

    let new_block_data = BlockData {
        name: door_data.name.clone(),
        properties: Some(new_props.clone()),
    };
    let new_block_id = BlockStateId::from_block_data(&new_block_data);

    // Determine the other half position
    let half = props.get("half").map(|s| s.as_str());
    let (this_pos, other_pos) = match half {
        Some("lower") => (clicked_pos, *clicked_pos + (0, 1, 0)),
        Some("upper") => (clicked_pos, *clicked_pos + (0, -1, 0)),
        _ => return false,
    };

    // Create the other half's block data (same open state, but different half)
    let other_half = if half == Some("lower") {
        "upper"
    } else {
        "lower"
    };
    let mut other_props = new_props.clone();
    other_props.insert("half".to_string(), other_half.to_string());

    let other_block_data = BlockData {
        name: door_data.name.clone(),
        properties: Some(other_props),
    };
    let other_block_id = BlockStateId::from_block_data(&other_block_data);

    // Update both halves in the world
    {
        let mut chunk =
            ferrumc_utils::world::load_or_generate_mut(&state.0, this_pos.chunk(), "overworld")
                .expect("Failed to load chunk for door");
        chunk.set_block(this_pos.chunk_block_pos(), new_block_id);
    }
    {
        let mut other_chunk =
            ferrumc_utils::world::load_or_generate_mut(&state.0, other_pos.chunk(), "overworld")
                .expect("Failed to load chunk for other door half");
        other_chunk.set_block(other_pos.chunk_block_pos(), other_block_id);
    }

    debug!(
        "Door {} at ({}, {}, {})",
        if current_open { "closed" } else { "opened" },
        clicked_pos.pos.x,
        clicked_pos.pos.y,
        clicked_pos.pos.z
    );

    // Send ack packet
    let ack_packet = BlockChangeAck {
        sequence: event.sequence,
    };
    if let Err(err) = conn.send_packet_ref(&ack_packet) {
        error!("Failed to send door interaction ack: {:?}", err);
    }

    // Create update packets for both halves
    let this_packet = BlockUpdate {
        location: NetworkPosition {
            x: this_pos.pos.x,
            y: this_pos.pos.y as i16,
            z: this_pos.pos.z,
        },
        block_state_id: VarInt::from(new_block_id),
    };
    let other_packet = BlockUpdate {
        location: NetworkPosition {
            x: other_pos.pos.x,
            y: other_pos.pos.y as i16,
            z: other_pos.pos.z,
        },
        block_state_id: VarInt::from(other_block_id),
    };

    // Broadcast to all nearby players
    let render_distance = get_global_config().chunk_render_distance as i32;
    let door_chunk = clicked_pos.chunk();
    let (door_chunk_x, door_chunk_z) = (door_chunk.x(), door_chunk.z());

    for (_, player_conn, _, _, pos) in query.iter() {
        let player_chunk = pos.chunk();
        let (chunk_x, chunk_z) = (player_chunk.x, player_chunk.y);

        if (door_chunk_x - chunk_x).abs() <= render_distance
            && (door_chunk_z - chunk_z).abs() <= render_distance
        {
            if let Err(err) = player_conn.send_packet_ref(&this_packet) {
                error!("Failed to send door update packet: {:?}", err);
            }
            if let Err(err) = player_conn.send_packet_ref(&other_packet) {
                error!("Failed to send other door half update packet: {:?}", err);
            }
        }
    }

    true
}
