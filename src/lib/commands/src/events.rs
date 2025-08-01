use std::sync::Arc;

use bevy_ecs::{entity::Entity, prelude::Event};

use crate::{ctx::CommandContext, infrastructure, Command};

#[derive(Event)]
pub struct CommandDispatchEvent {
    pub command: String,
    pub sender: Entity,
}

#[derive(Event)]
pub struct ResolvedCommandDispatchEvent {
    pub command: Arc<Command>,
    pub ctx: Arc<CommandContext>,
    pub sender: Entity,
}

impl CommandDispatchEvent {
    /// Attempts to find the command that was dispatched.
    pub fn lookup(&self) -> Option<Arc<Command>> {
        infrastructure::find_command(&self.command)
    }
}

