//! Handles despawning players when they leave the server.
//!
//! When a player disconnects:
//! 1. Broadcast RemoveEntitiesPacket to remove the entity from world
//! 2. Broadcast PlayerInfoRemovePacket to remove from tab list

use bevy_ecs::prelude::{Entity, MessageReader, Query, Res};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_messages::player_leave::PlayerLeft;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::player_info_remove::PlayerInfoRemovePacket;
use ferrumc_net::packets::outgoing::remove_entities::RemoveEntitiesPacket;
use ferrumc_state::GlobalStateResource;
use tracing::{error, trace};

/// Listens for `PlayerLeft` events and broadcasts despawn packets to remaining players.
pub fn handle(
    mut events: MessageReader<PlayerLeft>,
    player_query: Query<(Entity, &PlayerIdentity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        let left_player = &event.0;

        // Create packets once
        let remove_info_packet = PlayerInfoRemovePacket::single(left_player.uuid.as_u128());
        let remove_entity_packet =
            RemoveEntitiesPacket::from_entities(std::iter::once(left_player.clone()));

        let mut notified_count = 0;

        // Broadcast to all remaining players
        for (entity, identity, conn) in player_query.iter() {
            // Skip the player who left (their entity may already be despawning)
            if identity.uuid == left_player.uuid {
                continue;
            }

            // Skip disconnected players
            if !state.0.players.is_connected(entity) {
                continue;
            }

            // Remove entity from world
            if let Err(e) = conn.send_packet_ref(&remove_entity_packet) {
                error!("Failed to send remove entities packet: {:?}", e);
                continue;
            }

            // Remove from tab list
            if let Err(e) = conn.send_packet_ref(&remove_info_packet) {
                error!("Failed to send player info remove packet: {:?}", e);
                continue;
            }

            notified_count += 1;
        }

        trace!(
            "Player {} left: notified {} remaining players",
            left_player.username,
            notified_count
        );
    }
}
