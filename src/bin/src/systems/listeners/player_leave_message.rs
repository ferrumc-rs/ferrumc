use bevy_ecs::prelude::{Entity, MessageReader, Query};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::mq;
use ferrumc_messages::player_leave::PlayerLeft;
use ferrumc_text::{Color, NamedColor, TextComponent};

use tracing::trace; // We only need trace, mq will handle errors

/// Listens for `PlayerLeaveEvent` and broadcasts the "left" message
/// to all other connected players via the Message Queue.
pub fn handle(
    mut events: MessageReader<PlayerLeft>,
    player_query: Query<(Entity, &PlayerIdentity)>,
) {
    // 1. Loop through each "player left" event
    for event in events.read() {
        let player_who_left = &event.0;

        // 2. Build the "Player <player> left the game" message
        let mut message =
            TextComponent::from(format!("{} left the game", player_who_left.username));
        message.color = Some(Color::Named(NamedColor::Yellow));

        // 3. Loop through all players on the server
        for (receiver_entity, receiver_identity) in player_query.iter() {
            // Don't send the "you left" message to the player who just left
            if receiver_identity.uuid == player_who_left.uuid {
                continue;
            }

            // We clone the message because `mq::queue` takes ownership.
            mq::queue(message.clone(), false, receiver_entity);

            trace!(
                "Notified {} that {} left",
                receiver_identity.username,
                player_who_left.username
            );
        }
    }
}
