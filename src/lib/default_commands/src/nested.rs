use bevy_ecs::prelude::*;
use ferrumc_commands::Sender;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_text::TextComponent;

/// Command to show the sender.
#[command("nested")]
fn nested_command(#[sender] sender: Sender, query: Query<&PlayerIdentity>) {
    let username = match sender {
        Sender::Server => "Server".to_string(),
        Sender::Player(entity) => query
            .get(entity)
            .expect("sender does not exist")
            .username
            .clone(),
    };

    sender.send_message(
        TextComponent::from(format!("{} executed /nested", username)),
        false,
    );
}

/// Command to show sender with multiple words in the command
#[command("nested nested")]
fn nested_nested_command(#[sender] sender: Sender, query: Query<&PlayerIdentity>) {
    let username = match sender {
        Sender::Server => "Server".to_string(),
        Sender::Player(entity) => query
            .get(entity)
            .expect("sender does not exist")
            .username
            .clone(),
    };

    sender.send_message(
        TextComponent::from(format!("{} executed /nested nested", username)),
        false,
    );
}
