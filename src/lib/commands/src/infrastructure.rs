use dashmap::DashMap;
use std::sync::{Arc, LazyLock};

use crate::Command;

static COMMANDS: LazyLock<DashMap<&'static str, Arc<Command>>> = LazyLock::new(DashMap::new);

pub fn register_command(command: Arc<Command>) {
    COMMANDS.insert(command.name, command);
}

pub fn get_command_by_name(name: &str) -> Option<Arc<Command>> {
    COMMANDS.get(name).map(|cmd_ref| Arc::clone(&cmd_ref))
}

pub fn find_command(input: &str) -> Option<Arc<Command>> {
    let mut command = None;
    let mut current = input;

    while !current.is_empty() {
        command = get_command_by_name(current);
        if command.is_some() {
            break;
        }

        if let Some(pos) = current.rfind(' ') {
            current = &current[..pos];
        } else {
            current = "";
        }
    }

    command
}
