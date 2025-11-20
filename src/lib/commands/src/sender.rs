//! Command senders.

use bevy_ecs::prelude::*;
use ferrumc_components::chat::message_queue as mq;
use ferrumc_text::TextComponent;
use tracing::info;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A possible command sender.
pub enum Sender {
    /// A player has sent a command.
    Player(Entity),

    /// The server console has sent a command.
    Server,
}

impl Sender {
    /// Sends the given `message` to this sender, and to the action bar
    /// if `actionbar` is true.
    pub fn send_message(&self, message: TextComponent, actionbar: bool) {
        match self {
            Sender::Player(entity) => mq::queue(message, actionbar, *entity),
            Sender::Server => {
                info!("{message}"); // TODO: serialize into ANSI?
            }
        }
    }
}
