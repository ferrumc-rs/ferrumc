use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use arg::CommandArgument;
use ctx::CommandContext;
use ferrumc_text::TextComponent;
use input::CommandInput;

pub mod arg;
pub mod ctx;
pub mod errors;
pub mod events;
pub mod graph;
pub mod infrastructure;
pub mod input;

#[cfg(test)]
mod tests;

pub type ParserResult = Result<Box<dyn Any + 'static>, TextComponent>;

pub struct Command {
    pub name: &'static str,
    pub args: Vec<CommandArgument>,
}

impl Command {
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
