use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_entities::components::*; // Includes SyncedToPlayers
use ferrumc_entities::types::passive::pig::EntityUuid;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::spawn_entity::SpawnEntityPacket;
use tracing::{debug, error};

// Type alias to simplify the complex Query type
type EntitySyncQuery<'a> = (
    Entity,
    &'a EntityType,
    &'a EntityId,
    &'a EntityUuid,
    &'a Position,
    &'a Rotation,
    &'a mut SyncedToPlayers,
);

/// System that send new entities to players
pub fn entity_sync_system(
    // All non-player entities they needed to be sync
    mut entity_query: Query<EntitySyncQuery<'_>, Without<PlayerIdentity>>,

    // All connected players
    player_query: Query<(Entity, &StreamWriter, &Position), With<PlayerIdentity>>,
) {
    for (entity, entity_type, entity_id, entity_uuid, pos, rot, mut synced) in
        entity_query.iter_mut()
    {
        for (player_entity, stream_writer, player_pos) in player_query.iter() {
            // Skip if already send to the player
            if synced.player_entities.contains(&player_entity) {
                continue;
            }

            // TODO: Check distance (render distance)
            let distance = ((pos.x - player_pos.x).powi(2) + (pos.z - player_pos.z).powi(2)).sqrt();

            if distance > 128.0 {
                // 8 chunks de distance
                continue;
            }

            // Create and send spawn packet
            let protocol_id = entity_type.protocol_id();
            debug!(
                "Spawning {:?} (protocol_id={}) at ({:.2}, {:.2}, {:.2}) for player {:?}",
                entity_type, protocol_id, pos.x, pos.y, pos.z, player_entity
            );

            let spawn_packet = SpawnEntityPacket::entity(
                entity_id.0,
                entity_uuid.0.as_u128(),
                protocol_id,
                pos,
                rot,
            );

            if let Err(e) = stream_writer.send_packet(spawn_packet) {
                error!("Failed to send spawn packet: {:?}", e);
                continue;
            }

            // TODO: Send EntityMetadataPacket here to properly display the entity
            // The EntityMetadata constructors are not publicly exported yet
            // This might be why the pig appears as a phantom!

            synced.player_entities.push(player_entity);
            debug!(
                "Successfully sent entity {:?} (ID: {}) to player {:?}",
                entity, entity_id.0, player_entity
            );
        }
    }
}
