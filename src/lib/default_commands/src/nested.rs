use std::sync::Arc;

use ferrumc_commands::{
    arg::{parser::string::GreedyStringParser, CommandArgument},
    ctx::CommandContext,
    executor,
    infrastructure::register_command,
    Command, CommandResult,
};
use ferrumc_macros::{arg, command};

#[command("nested")]
async fn root(_ctx: Arc<CommandContext>) -> CommandResult {
    println!("Executed root");
    Ok(())
}

#[arg("message", GreedyStringParser)]
#[command("nested abc")]
async fn abc(ctx: Arc<CommandContext>) -> CommandResult {
    println!("Executed abc with message {}", ctx.arg::<String>("message"));
    Ok(())
}
