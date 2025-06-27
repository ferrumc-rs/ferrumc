use bevy_ecs::prelude::{Commands, Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::StreamWriter;
use ferrumc_pdc::{container::PersistentDataContainer, db::PdcDatabaseResource};
use ferrumc_state::GlobalStateResource;
use ferrumc_text::TextComponent;
use tracing::{debug, info, trace, warn};

pub fn connection_killer(
    query: Query<(
        Entity,
        &StreamWriter,
        &PlayerIdentity,
        &PersistentDataContainer,
    )>,
    mut cmd: Commands,
    state: Res<GlobalStateResource>,
    pdc_database: Res<PdcDatabaseResource>,
) {
    while let Some((disconnecting_entity, reason)) = state.0.players.disconnection_queue.pop() {
        for (entity, conn, player_identity, persistent_container) in query.iter() {
            if disconnecting_entity == entity {
                info!(
                    "Player {} ({}) disconnected: {}",
                    player_identity.username,
                    player_identity.uuid,
                    reason.as_deref().unwrap_or("No reason")
                );

                pdc_database
                    .database
                    .save(
                        format!("player:{}", player_identity.uuid.as_u128()).as_str(),
                        persistent_container,
                    )
                    .unwrap();
                debug!("Saved {}'s PDC to database", player_identity.username);

                if conn.running.load(std::sync::atomic::Ordering::Relaxed) {
                    trace!(
                        "Sending disconnect packet to player {}",
                        player_identity.username
                    );
                    if let Err(e) = conn.send_packet(
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
                // Broadcast the disconnection to other players
            }
            cmd.entity(entity).despawn();
        }
    }
}
