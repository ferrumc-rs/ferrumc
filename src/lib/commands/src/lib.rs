//! FerrumC's Command API.
#![feature(duration_constructors)]

use std::sync::{Arc, LazyLock};

use arg::CommandArgumentNode;

pub mod arg;
mod ctx;
pub mod errors;
pub mod graph;
pub mod infrastructure;
mod input;
pub mod messages;
mod sender;

// Re-export under main module to avoid clutter.
pub use ctx::*;
use ferrumc_macros::NetEncode;
use ferrumc_text::TextComponent;
pub use input::*;
pub use sender::*;

/// An instance of a command.
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    /// The name of the command.
    pub name: &'static str,

    /// All possible arguments this command can take.
    pub args: Vec<CommandArgumentNode>,
}

/// A command suggestion.
#[derive(NetEncode, Clone, Debug, PartialEq)]
pub struct Suggestion {
    /// The content of the suggestion.
    pub content: String,

    /// An optional tooltip that gets displayed when hovering over the suggestion.
    pub tooltip: Option<TextComponent>,
}

impl Suggestion {
    pub fn of(content: impl AsRef<str>) -> Suggestion {
        Suggestion {
            content: content.as_ref().to_string(),
            tooltip: None,
        }
    }
}

/// The root command. This is only for internal use and you should never ever have to rely on using this.
/// Only used in command suggestion cases when we don't know the command a player is entering yet.
pub static ROOT_COMMAND: LazyLock<Arc<Command>> = LazyLock::new(|| {
    Arc::new(Command {
        name: "",
        args: Vec::new(),
    })
});
