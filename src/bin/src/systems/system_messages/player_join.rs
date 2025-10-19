use bevy_ecs::entity::Entity;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::mq;
use ferrumc_text::{Color, NamedColor, TextComponent};
use tracing::debug;

pub fn handle(disconnecting_player: &PlayerIdentity, receiver_player: Entity) {
    let mut message =
        TextComponent::from(format!("{} joined the game", disconnecting_player.username));
    let color: Color = Color::Named(NamedColor::Yellow);
    message.color = Some(color);

    mq::queue(message, false, receiver_player);
}
