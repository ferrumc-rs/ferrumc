use std::sync::Arc;

use bevy_ecs::entity::Entity;
use tracing::error;

use crate::{arg::{utils::parser_error, CommandArgument}, input::CommandInput, Command, ParserResult};

pub struct CommandContext {
    pub input: CommandInput,
    pub command: Arc<Command>,
    pub sender: Entity,
}

impl CommandContext {
    pub fn arg<T: CommandArgument + Sized>(&mut self, name: &str) -> ParserResult<T> {
        if self.command.args.iter().any(|a| a.name == name) {
            T::parse(self)
        } else {
            error!("attempted to fetch non-existant command argument");
            Err(parser_error(&format!("arg {name} does not exist")))
        }
    }
}
