use bevy_ecs::prelude::*;

use ferrumc_components::chat::message_queue as mq;
use ferrumc_components::player::identity::PlayerIdentity;
use ferrumc_net::ChatMessagePacketReceiver;
use ferrumc_text::TextComponent;

pub fn handle(events: Res<ChatMessagePacketReceiver>, query: Query<&PlayerIdentity>) {
    for (message, sender) in events.0.try_iter() {
        let Ok(identity) = query.get(sender) else {
            continue;
        };

        let message = format!("<{}> {}", identity.username, message.message);
        mq::broadcast(TextComponent::from(message), false);
    }
}
