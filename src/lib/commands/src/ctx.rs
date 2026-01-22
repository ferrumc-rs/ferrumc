//! Command contexts.

use std::sync::Arc;

use tracing::error;

use crate::{
    Command,
    arg::{CommandArgument, ParserResult, utils::parser_error},
    input::CommandInput,
    sender::Sender,
};

/// Context of the execution of a command.
pub struct CommandContext {
    /// The command input.
    pub input: CommandInput,

    /// The command.
    pub command: Arc<Command>,

    /// The sender of the command.
    pub sender: Sender,
}

impl CommandContext {
    /// Attempts to retrieve and parse an argument of the given `name` and parses it with the given parser.
    pub fn arg<T: CommandArgument + Sized>(&mut self, name: &str) -> ParserResult<T> {
        if self.command.args.iter().any(|a| a.name == name) {
            T::parse(self)
        } else {
            error!("attempted to fetch non-existant command argument");
            Err(parser_error(&format!("arg {name} does not exist")))
        }
    }
}
