use std::sync::Arc;

use ferrumc_commands::{
    arg::{parser::{string::GreedyStringParser, ArgumentParser}, CommandArgument}, ctx::CommandContext, executor,
    infrastructure::register_command, Command, CommandResult,
};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::{arg, command};
use ferrumc_text::{NamedColor, TextComponentBuilder};

#[arg("message", GreedyStringParser::new())]
#[command("echo")]
async fn echo(ctx: Arc<CommandContext>) -> CommandResult {
    let message = ctx.arg::<String>("message");
    let identity = ctx.state.universe.get::<PlayerIdentity>(ctx.connection_id).expect("failed to get identity");

    ctx.reply(
        TextComponentBuilder::new(format!("{} said: {message}", identity.username))
            .color(NamedColor::Green)
            .build(),
    )
    .await
    .map_err(|err| {
        TextComponentBuilder::new(err.to_string())
            .color(NamedColor::Red)
            .build()
    })
}
