use bevy_ecs::prelude::{Entity, MessageReader, Query};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::mq;
use ferrumc_text::{Color, NamedColor, TextComponent};

use ferrumc_messages::player_join::PlayerJoined;

use tracing::trace;

/// Listens for `PlayerJoinEvent` and broadcasts the "join" message
/// to all other conneceted players via the Message Queue.
pub fn handle(
    mut events: MessageReader<PlayerJoined>,
    player_query: Query<(Entity, &PlayerIdentity)>,
) {
    // 1. Loop through each "player left" event
    for event in events.read() {
        let player_who_left = &event.0;

        // 2. Build the "Player <player> joined the game" message
        let mut message =
            TextComponent::from(format!("{} joined the game", player_who_left.username));
        message.color = Some(Color::Named(NamedColor::Yellow));

        // 3. Loop through all players on the server
        for (reciever_entity, receiver_identity) in player_query.iter() {
            mq::queue(message.clone(), false, reciever_entity);

            trace!(
                "Notified {} that {} left",
                receiver_identity.username,
                player_who_left.username
            );
        }
    }
}
