//! Messages related to commmands.

use std::sync::Arc;

use bevy_ecs::message::Message;

use crate::{ctx::CommandContext, sender::Sender, Command};

/// A command has been dispatched
#[derive(Message)]
pub struct CommandDispatched {
    /// The command string.
    pub command: String,

    /// The sender of the command.
    pub sender: Sender,
}

/// A command has been dispatched and resolved.
/// At this point in time, the command has not been executed
/// yet. This is up to the server or plugins to handle.
#[derive(Message)]
pub struct ResolvedCommandDispatched {
    /// The command.
    pub command: Arc<Command>,

    /// The created command context.
    pub ctx: CommandContext,

    /// The sender of the command.
    pub sender: Sender,
}
