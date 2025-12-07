use bevy_ecs::prelude::*;
use bevy_math::IVec3;
use ferrumc_components::player::swimming::SwimmingState;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_metadata::{EntityMetadata, EntityMetadataPacket};
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use tracing::error;

/// Height of player's eyes from feet (blocks)
const PLAYER_EYE_HEIGHT: f64 = 1.62;

/// Check if a player is in water by testing at eye level
fn is_player_in_water(state: &ferrumc_state::GlobalState, pos: &Position) -> bool {
    let eye_pos = IVec3::new(
        pos.x.floor() as i32,
        (pos.y + PLAYER_EYE_HEIGHT).floor() as i32,
        pos.z.floor() as i32,
    );

    state
        .world
        .get_block_and_fetch(eye_pos.x, eye_pos.y, eye_pos.z, "overworld")
        .map(|block_state| (86..=101).contains(&block_state.0))
        .unwrap_or(false)
}

/// System that detects when players enter/exit water and updates their swimming state
/// Also broadcasts the swimming pose to all connected clients
pub fn detect_player_swimming(
    mut swimmers: Query<(&PlayerIdentity, &Position, &mut SwimmingState)>,
    all_connections: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for (identity, pos, mut swimming_state) in swimmers.iter_mut() {
        let in_water = is_player_in_water(&state.0, pos);

        if in_water && !swimming_state.is_swimming {
            swimming_state.start_swimming();

            let entity_id = VarInt::new(identity.short_uuid);
            let packet = EntityMetadataPacket::new(
                entity_id,
                [
                    EntityMetadata::entity_swimming_state(),
                    EntityMetadata::entity_swimming_pose(),
                ],
            );

            broadcast_metadata(&packet, &all_connections, &state);
        } else if !in_water && swimming_state.is_swimming {
            swimming_state.stop_swimming();

            let entity_id = VarInt::new(identity.short_uuid);
            let packet = EntityMetadataPacket::new(
                entity_id,
                [
                    EntityMetadata::entity_clear_state(),
                    EntityMetadata::entity_standing(),
                ],
            );

            broadcast_metadata(&packet, &all_connections, &state);
        }
    }
}

/// Helper function to broadcast entity metadata to all connected players
fn broadcast_metadata(
    packet: &EntityMetadataPacket,
    connections: &Query<(Entity, &StreamWriter)>,
    state: &GlobalStateResource,
) {
    for (entity, conn) in connections {
        if !state.0.players.is_connected(entity) {
            continue;
        }
        if let Err(err) = conn.send_packet_ref(packet) {
            error!("Failed to send entity metadata packet: {:?}", err);
        }
    }
}
