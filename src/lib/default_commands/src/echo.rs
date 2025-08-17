use bevy_ecs::prelude::*;
use ferrumc_commands::{arg::{primitive::{string::QuotableString, PrimitiveArgument}, CommandArgument, ParserResult}, CommandContext, Sender, Suggestion};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_text::{TextComponent, TextComponentBuilder};

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
        
        vec![Suggestion::of("egg"), Suggestion::of("cheese"), Suggestion::of("fish")]
    }
}

#[command("fih")]
fn fih(#[arg] arg: TestArg, #[sender] sender: Sender) {
    sender.send_message(TextComponent::from(format!("i like {}", arg.0)), false);
}

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
