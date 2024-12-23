use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use ferrumc_state::GlobalState;

use crate::{input::CommandInput, Command};

pub struct CommandContext {
    pub input: Arc<Mutex<CommandInput>>,
    pub command: Arc<Command>,
    pub state: GlobalState,
}

impl CommandContext {
    pub fn new(input: CommandInput, command: Arc<Command>, state: GlobalState) -> Arc<Self> {
        Arc::new(Self {
            input: Arc::new(Mutex::new(input)),
            command,
            state,
        })
    }

    pub fn arg<T: Any>(&self, name: &str) -> T {
        if let Some(arg) = self.command.args.iter().find(|a| a.name == name) {
            let input = self.input.clone();
            let result = arg.parser.parse(Arc::new(self), input);

            return match result {
                Ok(b) => match b.downcast::<T>() {
                    Ok(value) => *value,
                    Err(_) => {
                        todo!("failed downcasting command argument, change design of this fn");
                    }
                },
                Err(err) => unreachable!("arg should have already been validated: {err}"),
            };
        } else {
            todo!();
        }
    }
}
