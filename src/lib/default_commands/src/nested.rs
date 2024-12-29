use std::sync::Arc;

use ferrumc_commands::{ctx::CommandContext, CommandResult, executor, infrastructure::register_command, Command, arg::{parser::string::GreedyStringParser, CommandArgument}};
use ferrumc_macros::{arg, command};

#[command("nested")]
async fn root(ctx: Arc<CommandContext>) -> CommandResult {
    println!("Executed root");
    Ok(())
}

#[arg("message", GreedyStringParser)]
#[command("nested abc")]
async fn abc(ctx: Arc<CommandContext>) -> CommandResult {
    println!("Executed abc with message {}", ctx.arg::<String>("message"));
    Ok(())
}
