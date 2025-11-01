use bevy_ecs::prelude::{Commands, Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::{DisconnectHandle, StreamWriter};
use ferrumc_state::GlobalStateResource;
use ferrumc_text::TextComponent;
use tracing::{info, trace, warn};

pub fn connection_killer(
    mut query: Query<(
        Entity,
        &StreamWriter,
        &PlayerIdentity,
        &mut DisconnectHandle,
    )>, // Mutable query
    mut cmd: Commands,
    state: Res<GlobalStateResource>,
) {
    while let Some((disconnecting_entity, reason)) = state.0.players.disconnection_queue.pop() {
        for (entity, conn, player_identity, mut disconnect_handle) in query.iter_mut() {
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
                    if let Some(sender) = disconnect_handle.sender.take() {
                        if let Err(_) = sender.send(()) {
                            trace!("Failed to send disconnect signal (receiver already dropped)");
                        }
                    }
                } else {
                    trace!(
                        "Connection for player {} is not running, skipping disconnect packet",
                        player_identity.username
                    );
                }
                cmd.entity(entity).despawn();
            } else {
                // Broadcast the disconnection to other players
            }
        }
    }
}
