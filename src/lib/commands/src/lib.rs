use std::{
    any::Any,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use arg::CommandArgument;
use ctx::CommandContext;
use ferrumc_text::TextComponent;
use input::CommandInput;

pub mod arg;
pub mod ctx;
pub mod errors;
pub mod graph;
pub mod infrastructure;
pub mod input;

#[cfg(test)]
mod tests;

pub type ParserResult = Result<Box<dyn Any + 'static>, TextComponent>;
pub type CommandResult = Result<(), TextComponent>;
pub type CommandOutput = Pin<Box<dyn Future<Output = CommandResult> + Send + 'static>>;
pub type CommandExecutor =
    Arc<dyn for<'a> Fn(Arc<CommandContext>) -> CommandOutput + Send + Sync + 'static>;

pub struct Command {
    pub name: &'static str,
    pub args: Vec<CommandArgument>,
    pub executor: CommandExecutor,
}

impl Command {
    pub fn execute(&self, ctx: Arc<CommandContext>) -> CommandOutput {
        (self.executor)(ctx)
    }

    pub fn validate(
        &self,
        ctx: &Arc<CommandContext>,
        input: &Arc<Mutex<CommandInput>>,
    ) -> Result<(), TextComponent> {
        for arg in &self.args {
            arg.parser.parse(ctx.clone(), input.clone())?;
        }

        Ok(())
    }
}

pub fn executor<F, Fut>(func: F) -> Arc<dyn Fn(Arc<CommandContext>) -> CommandOutput + Send + Sync>
where
    F: Fn(Arc<CommandContext>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = CommandResult> + Send + 'static,
{
    Arc::new(move |ctx: Arc<CommandContext>| Box::pin(func(ctx)) as CommandOutput)
}
