use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_entities::components::{EntityId, EntityType, Health};
use ferrumc_entities::EntityNetworkIdIndex;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::remove_entities::RemoveEntitiesPacket;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use tracing::{error, info};

/// System that checks for dead entities and despawns them
/// Works for all entity types (pigs, zombies, etc.)
pub fn entity_death_system(
    mut commands: Commands,
    entity_query: Query<(Entity, &EntityId, &EntityType, &Health)>,
    player_query: Query<(Entity, &StreamWriter), With<PlayerIdentity>>,
    mut entity_index: ResMut<EntityNetworkIdIndex>,
    state: Res<GlobalStateResource>,
) {
    for (entity, entity_id, entity_type, health) in entity_query.iter() {
        if health.is_dead() {
            info!(
                "Entity {:?} (ID: {}) died (HP: {}/{}), despawning...",
                entity_type,
                entity_id.to_network_id(),
                health.current,
                health.max
            );

            // Send RemoveEntitiesPacket to all connected players
            let network_id = entity_id.to_network_id();
            let remove_packet = RemoveEntitiesPacket {
                entity_ids: LengthPrefixedVec::new(vec![VarInt::new(network_id)]),
            };

            for (player_entity, stream_writer) in player_query.iter() {
                // Check if player is still connected
                if !state.0.players.is_connected(player_entity) {
                    continue;
                }

                if let Err(e) = stream_writer.send_packet_ref(&remove_packet) {
                    error!("Failed to send remove entity packet: {}", e);
                }
            }

            // Remove from network ID index
            entity_index.remove(network_id);

            // Despawn the entity from ECS
            commands.entity(entity).despawn();

            info!("Entity {} successfully despawned", network_id);
        }
    }
}
