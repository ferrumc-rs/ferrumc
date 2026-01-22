//! Handles PlayerInput packets for sneaking state changes.
//!
//! In 1.21.x protocol, sneaking is sent via PlayerInput packet (flag 0x20),
//! NOT via PlayerCommand (which was used in older protocol versions).

use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_components::player::sneak::SneakState;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::broadcast::broadcast_packet_except;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_metadata::{EntityMetadata, EntityMetadataPacket};
use ferrumc_net::PlayerInputReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::{debug, warn};

/// PlayerInput flags (1.21.x protocol)
const FLAG_SNEAK: u8 = 0x20;

/// Handles PlayerInput packets - specifically for sneaking state changes.
/// PlayerInput contains movement flags including sneak (0x20).
pub fn handle(
    receiver: Res<PlayerInputReceiver>,
    conn_query: Query<(Entity, &StreamWriter)>,
    identity_query: Query<&PlayerIdentity>,
    mut sneak_query: Query<&mut SneakState>,
) {
    for (event, eid) in receiver.0.try_iter() {
        let Ok(identity) = identity_query.get(eid) else {
            continue;
        };

        // SneakState should always exist - it's part of PlayerBundle
        let Ok(mut sneak_state) = sneak_query.get_mut(eid) else {
            warn!(
                "SneakState component missing for player {} - this shouldn't happen",
                identity.username
            );
            continue;
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
                    EntityMetadata::entity_sneaking_flag(),
                    EntityMetadata::entity_sneaking_visual(),
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

        broadcast_packet_except(eid, &packet, conn_query.iter());
    }
}
