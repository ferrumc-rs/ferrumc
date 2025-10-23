use crate::systems::system_messages;
use bevy_ecs::{
    event::EventWriter,
    prelude::{Entity, Query, Res},
};
use ferrumc_core::{
    conn::player_disconnect_event::PlayerDisconnectEvent,
    data::player::PlayerData,
    identity::player_identity::PlayerIdentity,
    transform::{grounded::OnGround, position::Position, rotation::Rotation},
};
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
use ferrumc_text::TextComponent;
use tracing::{info, trace, warn};

pub fn connection_killer(
    query: Query<(
        Entity,
        &StreamWriter,
        &PlayerIdentity,
        &Position,
        &OnGround,
        &Rotation,
    )>,
    mut dispatch_events: EventWriter<PlayerDisconnectEvent>,
    state: Res<GlobalStateResource>,
) {
    while let Some((disconnecting_entity, reason)) = state.0.players.disconnection_queue.pop() {
        let entity_result = query.get(disconnecting_entity);
        match entity_result {
            Ok(disconnecting_player) => {
                for (entity, conn, player_identity, position, on_ground, rotation) in query.iter() {
                    if disconnecting_entity == entity {
                        info!(
                            "Player {} ({}) disconnected: {}",
                            player_identity.username,
                            player_identity.uuid,
                            reason.as_deref().unwrap_or("No reason")
                        );
                        if conn.running.load(std::sync::atomic::Ordering::Relaxed) {
                            trace!(
                                "Sending disconnect packet to player {}",
                                player_identity.username
                            );
                            if let Err(e) = conn.send_packet_ref(
                                &ferrumc_net::packets::outgoing::disconnect::DisconnectPacket {
                                    reason: TextComponent::from(
                                        reason.as_deref().unwrap_or("Disconnected"),
                                    ),
                                },
                            ) {
                                warn!(
                                    "Failed to send disconnect packet to player {}: {:?}",
                                    player_identity.username, e
                                );
                            }
                        } else {
                            trace!(
                        "Connection for player {} is not running, skipping disconnect packet",
                        player_identity.username
                    );
                        }
                    } else {
                        system_messages::player_leave::handle(disconnecting_player.2, entity);
                    }
                    let player_disconnect = PlayerDisconnectEvent {
                        data: PlayerData::new(position, on_ground.0, "overworld", rotation),
                        identity: player_identity.to_owned(),
                        entity,
                    };
                    dispatch_events.write(player_disconnect);
                }
            }
            Err(e) => {
                warn!("Player's entity has already been removed: {}", e);
            }
        }
    }
}
