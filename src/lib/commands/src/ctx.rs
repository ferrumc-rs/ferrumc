use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use bevy_ecs::entity::Entity;

use crate::{input::CommandInput, Command};

pub struct CommandContext {
    pub input: Arc<Mutex<CommandInput>>,
    pub command: Arc<Command>,
    pub sender: Entity,
}

impl CommandContext {
    pub fn new(
        input: CommandInput,
        command: Arc<Command>,
        sender: Entity,
    ) -> Arc<Self> {
        Arc::new(Self {
            input: Arc::new(Mutex::new(input)),
            command,
            sender,
        })
    }

    pub fn arg<T: Any>(self: &Arc<Self>, name: &str) -> T {
        if let Some(arg) = self.command.args.iter().find(|a| a.name == name) {
            let input = self.input.clone();
            let result = arg.parser.parse(self.clone(), input);

            match result {
                Ok(b) => match b.downcast::<T>() {
                    Ok(value) => *value,
                    Err(_) => {
                        todo!("failed downcasting command argument, change design of this");
                    }
                },
                Err(err) => unreachable!("arg should have already been validated: {err}"),
            }
        } else {
            todo!();
        }
    }
}
