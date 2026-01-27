use crate::errors::BinaryError;
use bevy_ecs::prelude::{Entity, MessageWriter, Query, Res};
use ferrumc_components::player::abilities::PlayerAbilities;
use ferrumc_messages::player_digging::*;
use ferrumc_messages::BlockBrokenEvent;

use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net::PlayerActionReceiver;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::{block_state_id::BlockStateId, pos::BlockPos};
use tracing::{debug, error, warn};

pub fn handle(
    receiver: Res<PlayerActionReceiver>,
    state: Res<GlobalStateResource>,
    broadcast_query: Query<(Entity, &StreamWriter)>,
    player_query: Query<&PlayerAbilities>,
    (mut start_dig_events, mut cancel_dig_events, mut finish_dig_events, mut block_break_events): (
        MessageWriter<PlayerStartedDigging>,
        MessageWriter<PlayerCancelledDigging>,
        MessageWriter<PlayerFinishedDigging>,
        MessageWriter<BlockBrokenEvent>,
    ),
) {
    // https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol?oldid=2773393#Player_Action
    for (event, trigger_eid) in receiver.0.try_iter() {
        // Get the player's abilities to check their gamemode
        let Ok(abilities) = player_query.get(trigger_eid) else {
            warn!(
                "PlayerAction: Player {:?} has no PlayerAbilities component",
                trigger_eid
            );
            continue;
        };

        let pos: BlockPos = event.location.clone().into();
        if abilities.creative_mode {
            // --- CREATIVE MODE LOGIC ---
            // Only instabreak (status 0) is relevant in creative.
            if event.status.0 == 0 {
                let res: Result<(), BinaryError> = try {
                    // Get the block data before breaking to check if it's a door
                    let block_state_id = state
                        .0
                        .world
                        .get_block_and_fetch(pos.clone(), "overworld")
                        .unwrap_or_default();
                    let block_data = block_state_id.to_block_data();

                    // Check if this is a door block and get other half position
                    let other_half_pos = if let Some(ref data) = block_data {
                        if data.name.ends_with("_door") {
                            debug!("Creative mode: breaking door block {}", data.name);
                            if let Some(ref props) = data.properties {
                                let half = props.get("half").map(|s| s.as_str());
                                match half {
                                    Some("lower") => Some(pos.clone() + (0, 1, 0)),
                                    Some("upper") => Some(pos.clone() + (0, -1, 0)),
                                    _ => None,
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

                    // Break the main block
                    {
                        let mut chunk = ferrumc_utils::world::load_or_generate_mut(
                            &state.0,
                            pos.chunk(),
                            "overworld",
                        )
                        .expect("Failed to load or generate chunk");
                        chunk.set_block(pos.chunk_block_pos(), BlockStateId::default());
                    }

                    // Send block broken event for un-grounding system
                    block_break_events.write(BlockBrokenEvent {
                        position: pos.clone(),
                    });

                    // If it's a door, also break the other half
                    let other_half_update = if let Some(ref other_pos) = other_half_pos {
                        {
                            let mut other_chunk = ferrumc_utils::world::load_or_generate_mut(
                                &state.0,
                                other_pos.chunk(),
                                "overworld",
                            )
                            .expect("Failed to load or generate chunk for other door half");
                            other_chunk
                                .set_block(other_pos.chunk_block_pos(), BlockStateId::default());
                        }

                        debug!(
                            "Creative mode: also breaking other door half at ({}, {}, {})",
                            other_pos.pos.x, other_pos.pos.y, other_pos.pos.z
                        );

                        block_break_events.write(BlockBrokenEvent {
                            position: other_pos.clone(),
                        });

                        Some(BlockUpdate {
                            location: NetworkPosition {
                                x: other_pos.pos.x,
                                y: other_pos.pos.y as i16,
                                z: other_pos.pos.z,
                            },
                            block_state_id: VarInt::from(BlockStateId::default()),
                        })
                    } else {
                        None
                    };

                    // Broadcast the change
                    for (eid, conn) in &broadcast_query {
                        if !state.0.players.is_connected(eid) {
                            continue;
                        }

                        let block_update_packet = BlockUpdate {
                            location: event.location.clone(),
                            block_state_id: VarInt::from(BlockStateId::default()),
                        };
                        conn.send_packet_ref(&block_update_packet)
                            .map_err(BinaryError::Net)?;

                        // Also send other half update if it's a door
                        if let Some(ref other_update) = other_half_update {
                            conn.send_packet_ref(other_update)
                                .map_err(BinaryError::Net)?;
                        }

                        if eid == trigger_eid {
                            // Send ACK to the creative player
                            let ack_packet = BlockChangeAck {
                                sequence: event.sequence,
                            };
                            conn.send_packet_ref(&ack_packet)
                                .map_err(BinaryError::Net)?;
                        }
                    }
                };
                if res.is_err() {
                    error!("Error handling creative player action: {:?}", res);
                }
            }
        } else {
            // --- SURVIVAL MODE LOGIC ---
            // This handler's only job is to fire messages.
            match event.status.0 {
                0 => {
                    // Started digging
                    start_dig_events.write(PlayerStartedDigging {
                        player: trigger_eid,
                        position: event.location,
                        sequence: event.sequence,
                    });
                }
                1 => {
                    // Cancelled digging
                    cancel_dig_events.write(PlayerCancelledDigging {
                        player: trigger_eid,
                        sequence: event.sequence,
                    });
                }
                2 => {
                    // Finished digging
                    finish_dig_events.write(PlayerFinishedDigging {
                        player: trigger_eid,
                        position: event.location,
                        sequence: event.sequence,
                    });
                }
                _ => {} // Other statuses (drop item, etc.) are handled by different packets
            }
        }
    }
}
