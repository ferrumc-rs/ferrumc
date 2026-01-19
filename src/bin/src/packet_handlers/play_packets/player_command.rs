use bevy_ecs::prelude::{Entity, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::incoming::player_command::PlayerCommandAction;
use ferrumc_net::packets::outgoing::entity_metadata::{EntityMetadata, EntityMetadataPacket};
use ferrumc_net::PlayerCommandPacketReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error};

/// Handles PlayerCommand packets (sprinting, leave bed, etc.)
/// Note: Sneaking is handled via PlayerInput packet, NOT here
pub fn handle(
    receiver: Res<PlayerCommandPacketReceiver>,
    conn_query: Query<(Entity, &StreamWriter)>,
    identity_query: Query<&PlayerIdentity>,
    state: Res<GlobalStateResource>,
) {
    for (event, eid) in receiver.0.try_iter() {
        // Skip disconnected players
        if !state.0.players.is_connected(eid) {
            debug!("Player {:?} is not connected, skipping PlayerCommand", eid);
            continue;
        }

        // Get the sender's identity to use the correct entity ID
        let Ok(identity) = identity_query.get(eid) else {
            error!("Failed to get PlayerIdentity for entity {:?}", eid);
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

                for (entity, conn) in conn_query.iter() {
                    if entity == eid {
                        continue;
                    }
                    if !state.0.players.is_connected(entity) {
                        continue;
                    }
                    if let Err(err) = conn.send_packet_ref(&packet) {
                        error!("Failed to send start sprinting packet: {:?}", err);
                    }
                }
            }
            PlayerCommandAction::StopSprinting => {
                let packet =
                    EntityMetadataPacket::new(entity_id, [EntityMetadata::entity_clear_state()]);

                for (entity, conn) in conn_query.iter() {
                    if entity == eid {
                        continue;
                    }
                    if !state.0.players.is_connected(entity) {
                        continue;
                    }
                    if let Err(err) = conn.send_packet_ref(&packet) {
                        error!("Failed to send stop sprinting packet: {:?}", err);
                    }
                }
            }
            _ => {}
        }
    }
}
