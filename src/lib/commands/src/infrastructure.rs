use dashmap::DashMap;
use std::sync::{Arc, LazyLock};

use crate::Command;

static COMMANDS: LazyLock<DashMap<&'static str, Arc<Command>>> = LazyLock::new(DashMap::new);

pub fn register_command(command: Arc<Command>) {
    COMMANDS.insert(command.name, command);
}

pub fn get_command_by_name(name: &'static str) -> Option<Arc<Command>> {
    COMMANDS.get(name).map(|cmd_ref| Arc::clone(&cmd_ref))
}

pub fn find_command(input: &'static str) -> Option<Arc<Command>> {
    let mut command = None;
    let mut input = input;

    while !input.is_empty() {
        command = get_command_by_name(input);
        if command.is_some() {
            break;
        }

        if let Some(pos) = input.rfind(' ') {
            input = &input[..pos];
        } else {
            input = "";
        }
    }

    command
}
