use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_entities::components::*;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::update_entity_position_and_rotation::UpdateEntityPositionAndRotationPacket;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::error;

type EntitySyncComponents<'a> = (
    &'a EntityId,
    &'a Position,
    &'a Rotation,
    &'a OnGround,
    &'a SyncedToPlayers,
    &'a mut LastSyncedPosition,
);

type EntitySyncFilter = (With<EntityType>, Without<PlayerIdentity>);

/// System that syncs entity movement to players
pub fn entity_movement_sync_system(
    mut entities: Query<EntitySyncComponents, EntitySyncFilter>,
    players: Query<&StreamWriter, With<PlayerIdentity>>,
) {
    for (entity_id, pos, rot, on_ground, synced, mut last_pos) in entities.iter_mut() {
        // Only sync if entity has moved
        if !last_pos.has_moved(pos) {
            continue;
        }

        let delta = last_pos.delta_to(pos);

        // Send update to all players who have this entity spawned
        for player_entity in &synced.player_entities {
            if let Ok(stream_writer) = players.get(*player_entity) {
                let packet = UpdateEntityPositionAndRotationPacket {
                    entity_id: VarInt::new(entity_id.to_network_id()),
                    delta_x: delta.0,
                    delta_y: delta.1,
                    delta_z: delta.2,
                    yaw: ferrumc_net_codec::net_types::angle::NetAngle::from_degrees(
                        rot.yaw as f64,
                    ),
                    pitch: ferrumc_net_codec::net_types::angle::NetAngle::from_degrees(
                        rot.pitch as f64,
                    ),
                    on_ground: on_ground.0,
                };

                if let Err(e) = stream_writer.send_packet(packet) {
                    error!("Failed to send entity movement packet: {:?}", e);
                }
            }
        }

        // Update last synced position
        *last_pos = LastSyncedPosition::from_position(pos);
    }
}
