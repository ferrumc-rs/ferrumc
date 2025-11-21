use std::sync::Arc;

use crate::errors::BinaryError;
use bevy_ecs::prelude::{Entity, EventWriter, Query, Res};
use ferrumc_components::player::abilities::PlayerAbilities;
use ferrumc_events::player_digging::*;

use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net::PlayerActionReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use tracing::{error, trace, warn};

pub fn handle(
    events: Res<PlayerActionReceiver>,
    state: Res<GlobalStateResource>,
    broadcast_query: Query<(Entity, &StreamWriter)>,
    player_query: Query<&PlayerAbilities>,
    mut start_dig_events: EventWriter<PlayerStartDiggingEvent>,
    mut cancel_dig_events: EventWriter<PlayerCancelDiggingEvent>,
    mut finish_dig_events: EventWriter<PlayerFinishDiggingEvent>,
) {
    for (event, trigger_eid) in events.0.try_iter() {
        // Get the player's abilities to check their gamemode
        let Ok(abilities) = player_query.get(trigger_eid) else {
            warn!(
                "PlayerAction: Player {:?} has no PlayerAbilities component",
                trigger_eid
            );
            continue;
        };

        if abilities.creative_mode {
            // --- CREATIVE MODE LOGIC ---
            // Only instabreak (status 0) is relevant in creative.
            if event.status.0 == 0 {
                let res: Result<(), BinaryError> = try {
                    let mut chunk = match state.0.clone().world.load_chunk_owned(
                        event.location.x >> 4,
                        event.location.z >> 4,
                        "overworld",
                    ) {
                        Ok(chunk) => chunk,
                        Err(e) => {
                            trace!("Chunk not found, generating new chunk: {:?}", e);
                            state
                                .0
                                .clone()
                                .terrain_generator
                                .generate_chunk(event.location.x >> 4, event.location.z >> 4)
                                .map_err(BinaryError::WorldGen)?
                        }
                    };
                    let (relative_x, relative_y, relative_z) = (
                        event.location.x.abs() % 16,
                        event.location.y as i32,
                        event.location.z.abs() % 16,
                    );
                    chunk
                        .set_block(relative_x, relative_y, relative_z, BlockStateId::default())
                        .map_err(BinaryError::World)?;

                    state
                        .0
                        .world
                        .save_chunk(Arc::new(chunk))
                        .map_err(BinaryError::World)?;

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
            // This handler's only job is to fire events.
            match event.status.0 {
                0 => {
                    // Started digging
                    start_dig_events.write(PlayerStartDiggingEvent {
                        player: trigger_eid,
                        position: event.location,
                        sequence: event.sequence,
                    });
                }
                1 => {
                    // Cancelled digging
                    cancel_dig_events.write(PlayerCancelDiggingEvent {
                        player: trigger_eid,
                        sequence: event.sequence,
                    });
                }
                2 => {
                    // Finished digging
                    finish_dig_events.write(PlayerFinishDiggingEvent {
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
