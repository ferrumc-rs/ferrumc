//! FerrumC's Command API.

use std::sync::{Arc, LazyLock};

use arg::CommandArgumentNode;

pub mod arg;
mod ctx;
pub mod errors;
pub mod events;
pub mod graph;
pub mod infrastructure;
mod input;
mod sender;

// Re-export under main module to avoid clutter.
pub use ctx::*;
use ferrumc_text::TextComponent;
pub use input::*;
pub use sender::*;

#[cfg(test)]
mod tests;

/// An instance of a command.
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    /// The name of the command.
    pub name: &'static str,

    /// All possible arguments this command can take.
    pub args: Vec<CommandArgumentNode>,
}

/// A command suggestion.
#[derive(Clone, Debug, PartialEq)]
pub struct Suggestion {
    /// The content of the suggestion.
    pub content: String,

    /// An optional tooltip that gets displayed when hovering over the suggestion.
    pub tooltip: Option<TextComponent>,
}

/// The root command. This is only for internal use and you should never ever have to rely on using this.
/// Only used in command suggestion cases when we don't know the command a player is entering yet.
pub static ROOT_COMMAND: LazyLock<Arc<Command>> = LazyLock::new(|| Arc::new(Command { name: "", args: Vec::new() }));
