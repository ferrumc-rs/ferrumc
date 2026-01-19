use bevy_ecs::prelude::{Commands, Component, Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_metadata::{EntityMetadata, EntityMetadataPacket};
use ferrumc_net::PlayerInputReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error};

/// PlayerInput flags (1.21.x protocol)
const FLAG_SNEAK: u8 = 0x20;

/// Component to track player's previous sneak state
#[derive(Component, Default)]
pub struct SneakState {
    pub is_sneaking: bool,
}

/// Handles PlayerInput packets - specifically for sneaking state changes
/// PlayerInput contains movement flags including sneak (0x20)
pub fn handle(
    mut commands: Commands,
    receiver: Res<PlayerInputReceiver>,
    conn_query: Query<(Entity, &StreamWriter)>,
    identity_query: Query<&PlayerIdentity>,
    mut sneak_query: Query<&mut SneakState>,
    state: Res<GlobalStateResource>,
) {
    for (event, eid) in receiver.0.try_iter() {
        if !state.0.players.is_connected(eid) {
            continue;
        }

        let Ok(identity) = identity_query.get(eid) else {
            continue;
        };

        // Get or create SneakState
        let mut sneak_state = match sneak_query.get_mut(eid) {
            Ok(state) => state,
            Err(_) => {
                // Add SneakState component if missing
                commands.entity(eid).insert(SneakState::default());
                continue; // Skip this tick, will process next time
            }
        };

        let is_sneaking = (event.flags & FLAG_SNEAK) != 0;

        // Only broadcast if state changed
        if is_sneaking == sneak_state.is_sneaking {
            continue;
        }

        sneak_state.is_sneaking = is_sneaking;
        let entity_id = VarInt::new(identity.short_uuid);

        debug!(
            "PlayerInput: sneak={} from {} (entity_id={})",
            is_sneaking, identity.username, identity.short_uuid
        );

        let packet = if is_sneaking {
            EntityMetadataPacket::new(
                entity_id,
                [
                    EntityMetadata::entity_sneaking_visual(),
                    EntityMetadata::entity_sneaking_pressed(),
                ],
            )
        } else {
            EntityMetadataPacket::new(
                entity_id,
                [
                    EntityMetadata::entity_clear_state(),
                    EntityMetadata::entity_standing(),
                ],
            )
        };

        for (entity, conn) in conn_query.iter() {
            if entity == eid {
                continue;
            }
            if !state.0.players.is_connected(entity) {
                continue;
            }
            if let Err(err) = conn.send_packet_ref(&packet) {
                error!("Failed to send sneak packet: {:?}", err);
            }
        }
    }
}
