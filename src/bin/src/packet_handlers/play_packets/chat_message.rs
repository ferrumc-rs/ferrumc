use bevy_ecs::prelude::*;
use ferrumc_core::{identity::player_identity::PlayerIdentity, mq};
use ferrumc_net::ChatMessagePacketReceiver;
use ferrumc_text::TextComponent;

/// Handles the normal chat for the player,
/// so that if someone sends a message,
/// everyone that should get it, gets it.
pub fn handle(events: Res<ChatMessagePacketReceiver>, query: Query<&PlayerIdentity>) {
    for (message, sender) in events.0.try_iter() {
        let Ok(identity) = query.get(sender) else {
            continue;
        };

        let message = format!("<{}> {}", identity.username, message.message);
        mq::broadcast(TextComponent::from(message), false);
    }
}
