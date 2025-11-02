use std::time::Duration;

use bevy_ecs::prelude::*;
use ferrumc_commands::{
    arg::{
        primitive::{string::QuotableString, PrimitiveArgument},
        CommandArgument, ParserResult,
    },
    CommandContext, Sender, Suggestion,
};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_text::{TextComponent, TextComponentBuilder};

/// Test Argument
struct TestArg(String);

impl CommandArgument for TestArg {
    fn parse(ctx: &mut CommandContext) -> ParserResult<Self> {
        Ok(Self(ctx.input.read_string()))
    }

    fn primitive() -> PrimitiveArgument {
        PrimitiveArgument::word()
    }

    fn suggest(ctx: &mut CommandContext) -> Vec<Suggestion> {
        ctx.input.read_string();

        vec![
            Suggestion::of("egg"),
            Suggestion::of("cheese"),
            Suggestion::of("fish"),
        ]
    }
}

/// Test Command to say a specific string.
#[command("love")]
fn love(#[sender] sender: Sender, #[arg] arg: TestArg, #[arg] duration: Duration) {
    sender.send_message(
        TextComponent::from(format!("i've loved {} for {:?}", arg.0, duration)),
        false,
    );
}

/// Test command to echo a message.
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
