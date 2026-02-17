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
use tracing::{error, warn};

use crate::systems::interaction::block_interactions::break_block_with_door_half;

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
                    let mut chunk = ferrumc_utils::world::load_or_generate_mut(
                        &state.0,
                        pos.chunk(),
                        "overworld",
                    )
                    .expect("Failed to load or generate chunk");

                    let broken_positions =
                        break_block_with_door_half(&mut chunk, pos, &mut block_break_events);

                    // Broadcast the change
                    for (eid, conn) in &broadcast_query {
                        if !state.0.players.is_connected(eid) {
                            continue;
                        }

                        for broken_pos in &broken_positions {
                            let update = BlockUpdate {
                                location: NetworkPosition {
                                    x: broken_pos.pos.x,
                                    y: broken_pos.pos.y as i16,
                                    z: broken_pos.pos.z,
                                },
                                block_state_id: VarInt::from(BlockStateId::default()),
                            };
                            conn.send_packet_ref(&update).map_err(BinaryError::Net)?;
                        }

                        if eid == trigger_eid {
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
