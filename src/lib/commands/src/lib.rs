//! FerrumC's Command API.

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
pub use input::*;
pub use sender::*;

#[cfg(test)]
mod tests;

/// An instance of a command.
pub struct Command {
    /// The name of the command.
    pub name: &'static str,

    /// All possible arguments this command can take.
    pub args: Vec<CommandArgumentNode>,
}
