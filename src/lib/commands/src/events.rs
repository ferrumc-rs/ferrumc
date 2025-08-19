//! Events related to commmands.

use std::sync::Arc;

use bevy_ecs::prelude::Event;

use crate::{ctx::CommandContext, infrastructure, sender::Sender, Command};

/// A command has been dispatched.
#[derive(Event)]
pub struct CommandDispatchEvent {
    /// The command string.
    pub command: String,

    /// The sender of the command.
    pub sender: Sender,
}

/// A command has been dispatched and resolved.
/// At this point in time, the command has not been executed
/// yet. This is up to the server or plugins to handle.
#[derive(Event)]
pub struct ResolvedCommandDispatchEvent {
    /// The command.
    pub command: Arc<Command>,

    /// The created command context.
    pub ctx: CommandContext,

    /// The sender of the command.
    pub sender: Sender,
}

impl CommandDispatchEvent {
    /// Attempts to find the command that was dispatched.
    pub fn lookup(&self) -> Option<Arc<Command>> {
        infrastructure::find_command(&self.command)
    }
}
