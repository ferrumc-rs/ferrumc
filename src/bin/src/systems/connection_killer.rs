use crate::systems::system_messages;
use bevy_ecs::prelude::{Commands, Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::player::abilities::PlayerAbilities;
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::player_cache::OfflinePlayerData;
use ferrumc_state::GlobalStateResource;
use ferrumc_text::TextComponent;
use tracing::{info, trace, warn};

pub fn connection_killer(
    query: Query<(Entity, &StreamWriter, &PlayerIdentity, &PlayerAbilities)>,
    mut cmd: Commands,
    state: Res<GlobalStateResource>,
) {
    while let Some((disconnecting_entity, reason)) = state.0.players.disconnection_queue.pop() {
        let disconnecting_player_identity = query
            .get(disconnecting_entity)
            .ok()
            .map(|(_, _, identity, _)| identity.clone());

        if disconnecting_player_identity.is_none() {
            warn!("Player's entity has already been removed");
            continue;
        }

        let disconnecting_player_identity = disconnecting_player_identity.unwrap();

        for (entity, conn, player_identity, abilities) in query.iter() {
            if disconnecting_entity == entity {
                info!(
                    "Player {} ({}) disconnected: {}. Caching data...",
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

                // --- Save player data to the cache --
                // Create the data bundle
                let data_to_cache = OfflinePlayerData::new(abilities.clone());
                // Access the cache via the GlobalStateResource and insert the data
                state
                    .0
                    .player_cache
                    .insert(player_identity.uuid, data_to_cache);

                cmd.entity(entity).despawn();
            } else {
                system_messages::player_leave::handle(&disconnecting_player_identity, entity);
            }
        }
    }
}
