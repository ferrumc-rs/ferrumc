use std::sync::Arc;

use ferrumc_commands::{
    arg::{
        parser::{
            int::IntParser,
            string::{QuotedStringParser, SingleStringParser},
        },
        CommandArgument,
    },
    ctx::CommandContext,
    executor,
    infrastructure::register_command,
    Command, CommandResult,
};
use ferrumc_macros::{arg, command};
use ferrumc_text::TextComponentBuilder;

#[command("nested")]
async fn root(ctx: Arc<CommandContext>) -> CommandResult {
    ctx.reply(TextComponentBuilder::new("Executed /nested").build())
        .await
        .unwrap();
    Ok(())
}

#[arg("message", QuotedStringParser)]
#[arg("word", SingleStringParser)]
#[arg("number", IntParser)]
#[command("nested abc")]
async fn abc(ctx: Arc<CommandContext>) -> CommandResult {
    let message = ctx.arg::<String>("message");
    let word = ctx.arg::<String>("word");
    let number = ctx.arg::<i32>("number");

    ctx.reply(
        TextComponentBuilder::new(format!(
            "Message: {message:?}, Word: {word:?}, Number: {number}"
        ))
        .build(),
    )
    .await
    .unwrap();

    Ok(())
}
