use bevy_ecs::prelude::{Entity, MessageReader, Query};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::mq;
use ferrumc_text::{Color, NamedColor, TextComponent};

use ferrumc_messages::player_join::PlayerJoined;

use tracing::trace;

/// Listens for `PlayerJoinEvent` and broadcasts the "join" message
/// to all other connected players via the Message Queue.
pub fn handle(
    mut events: MessageReader<PlayerJoined>,
    player_query: Query<(Entity, &PlayerIdentity)>,
) {
    for event in events.read() {
        let player_who_joined = &event.identity;

        // Build the "Player <player> joined the game" message
        let mut message =
            TextComponent::from(format!("{} joined the game", player_who_joined.username));
        message.color = Some(Color::Named(NamedColor::Yellow));

        // Broadcast to all players on the server
        for (receiver_entity, receiver_identity) in player_query.iter() {
            mq::queue(message.clone(), false, receiver_entity);

            trace!(
                "Notified {} that {} joined",
                receiver_identity.username,
                player_who_joined.username
            );
        }
    }
}
