use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::broadcast::broadcast_packet_except;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::incoming::player_command::PlayerCommandAction;
use ferrumc_net::packets::outgoing::entity_metadata::{EntityMetadata, EntityMetadataPacket};
use ferrumc_net::PlayerCommandPacketReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::debug;

/// Handles PlayerCommand packets (sprinting, leave bed, etc.)
/// Note: Sneaking is handled via PlayerInput packet, NOT here
pub fn handle(
    receiver: Res<PlayerCommandPacketReceiver>,
    conn_query: Query<(Entity, &StreamWriter)>,
    identity_query: Query<&PlayerIdentity>,
) {
    for (event, eid) in receiver.0.try_iter() {
        // Get the sender's identity to use the correct entity ID
        let Ok(identity) = identity_query.get(eid) else {
            continue;
        };

        let entity_id = VarInt::new(identity.short_uuid);

        debug!(
            "PlayerCommand: {:?} from {} (entity_id={})",
            event.action, identity.username, identity.short_uuid
        );

        match event.action {
            PlayerCommandAction::StartSprinting => {
                let packet =
                    EntityMetadataPacket::new(entity_id, [EntityMetadata::entity_sprinting()]);
                broadcast_packet_except(eid, &packet, conn_query.iter());
            }
            PlayerCommandAction::StopSprinting => {
                let packet =
                    EntityMetadataPacket::new(entity_id, [EntityMetadata::entity_clear_state()]);
                broadcast_packet_except(eid, &packet, conn_query.iter());
            }
            _ => {}
        }
    }
}
