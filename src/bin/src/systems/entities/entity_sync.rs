use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_entities::components::*; // Includes SyncedToPlayers
use ferrumc_entities::types::passive::pig::EntityUuid;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_metadata::{EntityMetadata, EntityMetadataPacket};
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

            // TODO: INVESTIGATE PROTOCOL_ID OFFSET BUG
            // There is a systematic -1 offset between what we send and what the client displays:
            // - Registry says Pig=94, Phantom=93
            // - When sending 94, client displays Phantom (93)
            // - When sending 95, client displays Pig (94)
            // Possible causes:
            // 1. Registry version mismatch (registry for 1.21.7 but client is 1.21.8?)
            // 2. VarInt encoding issue
            // 3. Protocol expects 1-based indexing instead of 0-based
            // For now, adding +1 as a workaround until root cause is found
            let adjusted_protocol_id = protocol_id + 1;
            debug!(
                "Spawning {:?} (registry_id={}, sending={}) at ({:.2}, {:.2}, {:.2}) for player {:?}",
                entity_type, protocol_id, adjusted_protocol_id, pos.x, pos.y, pos.z, player_entity
            );
            debug!(
                "DEBUG: entity_id={}, uuid={}, type_id={}, data=0",
                entity_id.to_network_id(),
                entity_uuid.0.as_u128(),
                adjusted_protocol_id
            );

            let spawn_packet = SpawnEntityPacket::entity(
                entity_id.to_network_id(),
                entity_uuid.0.as_u128(),
                adjusted_protocol_id,
                pos,
                rot,
            );

            if let Err(e) = stream_writer.send_packet(spawn_packet) {
                error!("Failed to send spawn packet: {:?}", e);
                continue;
            }

            // Send EntityMetadataPacket to properly display the entity
            // We need to send both entity flags (index 0) and pose (index 6)
            let metadata_packet = EntityMetadataPacket::new(
                entity_id.as_varint(),
                [
                    EntityMetadata::entity_normal_state(), // Index 0: normal entity state (no special flags)
                    EntityMetadata::entity_standing(),     // Index 6: standing pose
                ],
            );

            if let Err(e) = stream_writer.send_packet(metadata_packet) {
                error!("Failed to send entity metadata packet: {:?}", e);
                continue;
            }

            synced.player_entities.push(player_entity);
            debug!(
                "Successfully sent entity {:?} (ID: {}) to player {:?}",
                entity,
                entity_id.to_network_id(),
                player_entity
            );
        }
    }
}
