use std::sync::Arc;

use ferrumc_commands::{
    arg::parser::string::GreedyStringParser, arg::CommandArgument, ctx::CommandContext, executor,
    infrastructure::register_command, Command, CommandResult,
};
use ferrumc_macros::{arg, command};
use ferrumc_text::{NamedColor, TextComponentBuilder};

#[arg("message", GreedyStringParser)]
#[command("echo")]
async fn echo(ctx: Arc<CommandContext>) -> CommandResult {
    let message = ctx.arg::<String>("message");
    ctx.reply(
        TextComponentBuilder::new(message)
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
