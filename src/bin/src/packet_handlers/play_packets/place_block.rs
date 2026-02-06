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
use tracing::{debug, error, trace};

use crate::systems::interaction::block_interactions::{try_interact, InteractionResult};
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

        // Convert network position to block position (the block that was clicked)
        let clicked_pos: BlockPos = event.position.clone().into();

        // Load the chunk containing the clicked block
        let chunk_result = ferrumc_utils::world::load_or_generate_mut(
            &state.0,
            clicked_pos.chunk(),
            "overworld",
        );

        let mut chunk = match chunk_result {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to load chunk for interaction: {:?}", e);
                continue 'ev_loop;
            }
        };

        // Get the block that was clicked
        let clicked_block_state = chunk.get_block(clicked_pos.chunk_block_pos());

        debug!(
            "PlaceBlock event: pos=({}, {}, {}), clicked_block_state={} (raw: {})",
            clicked_pos.pos.x, clicked_pos.pos.y, clicked_pos.pos.z,
            clicked_block_state, clicked_block_state.raw()
        );

        // Try to interact with the block directly in the world
        // TODO: Check if player is sneaking - if sneaking, skip interaction and place block
        match try_interact(clicked_block_state) {
            InteractionResult::Toggled(new_state) => {
                // Block was toggled! Update the chunk and broadcast to players
                debug!(
                    "Player {:?} toggled block at ({}, {}, {}) -> new state: {}",
                    entity, clicked_pos.pos.x, clicked_pos.pos.y, clicked_pos.pos.z, new_state
                );

                // Update the block in the chunk
                chunk.set_block(clicked_pos.chunk_block_pos(), new_state);

                // Send ack to the player who clicked
                let ack_packet = BlockChangeAck {
                    sequence: event.sequence,
                };
                if let Err(err) = conn.send_packet_ref(&ack_packet) {
                    error!("Failed to send block change ack packet: {:?}", err);
                }

                // Broadcast block update to all nearby players
                let block_update = BlockUpdate {
                    location: NetworkPosition {
                        x: clicked_pos.pos.x,
                        y: clicked_pos.pos.y as i16,
                        z: clicked_pos.pos.z,
                    },
                    block_state_id: VarInt::from(new_state),
                };

                let clicked_chunk = clicked_pos.chunk();
                let (clicked_chunk_x, clicked_chunk_z) = (clicked_chunk.x(), clicked_chunk.z());
                let render_distance = get_global_config().chunk_render_distance as i32;

                for (_, player_conn, _, _, player_pos) in query.iter() {
                    let player_chunk = player_pos.chunk();
                    let (player_chunk_x, player_chunk_z) = (player_chunk.x, player_chunk.y);

                    // Send update if player is within render distance
                    if (clicked_chunk_x - player_chunk_x).abs() <= render_distance
                        && (clicked_chunk_z - player_chunk_z).abs() <= render_distance
                    {
                        if let Err(err) = player_conn.send_packet_ref(&block_update) {
                            error!("Failed to send block update packet: {:?}", err);
                        }
                    }
                }

                continue 'ev_loop;
            }
            InteractionResult::NotInteractive => {
                // Block is not interactive, proceed with normal placement logic
                trace!("Block at ({}, {}, {}) is not interactive",
                    clicked_pos.pos.x, clicked_pos.pos.y, clicked_pos.pos.z);
            }
            InteractionResult::InvalidBlock => {
                error!("Invalid block state at ({}, {}, {})",
                    clicked_pos.pos.x, clicked_pos.pos.y, clicked_pos.pos.z);
                continue 'ev_loop;
            }
        }

        // Drop the chunk lock before proceeding with placement
        // (placement needs to load chunk again with offset position)
        drop(chunk);

        // Normal block placement logic
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
                        error!("No block mapping found for item ID: {}", item_id.0);
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

                    let mut chunk = ferrumc_utils::world::load_or_generate_mut(
                        &state.0,
                        offset_pos.chunk(),
                        "overworld",
                    )
                    .expect("Failed to load or generate chunk");
                    let block_clicked = chunk.get_block(offset_pos.chunk_block_pos());
                    trace!("Block clicked: {:?}", block_clicked);

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

                    chunk.set_block(offset_pos.chunk_block_pos(), *mapped_block_state_id);
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
