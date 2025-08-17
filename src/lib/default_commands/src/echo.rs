use bevy_ecs::prelude::*;
use ferrumc_commands::{arg::primitive::string::QuotableString, Sender};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_text::{TextComponent, TextComponentBuilder};

#[command("echo")]
fn test_command(
    #[arg] message: QuotableString,
    #[sender] sender: Sender,
    query: Query<&PlayerIdentity>,
) {
    let username = match sender {
        Sender::Server => "Server".to_string(),
        Sender::Player(entity) => query
            .get(entity)
            .expect("sender does not exist")
            .username
            .clone(),
    };

    sender.send_message(
        TextComponentBuilder::new(format!("{} said: ", username))
            .extra(TextComponent::from(message.clone()))
            .build(),
        false,
    );
}
