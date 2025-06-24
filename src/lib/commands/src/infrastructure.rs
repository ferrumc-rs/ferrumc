use dashmap::DashMap;
use std::sync::{Arc, LazyLock, RwLock};

use crate::{graph::CommandGraph, Command};

static COMMANDS: LazyLock<DashMap<&'static str, Arc<Command>>> = LazyLock::new(DashMap::new);
static COMMAND_GRAPH: LazyLock<RwLock<CommandGraph>> =
    LazyLock::new(|| RwLock::new(CommandGraph::default()));

pub fn register_command(command: Arc<Command>) {
    COMMANDS.insert(command.name, command.clone());
    if let Ok(mut graph) = COMMAND_GRAPH.write() {
        graph.push(command);
    }
}

pub fn get_graph() -> CommandGraph {
    if let Ok(graph) = COMMAND_GRAPH.read() {
        graph.clone()
    } else {
        CommandGraph::default()
    }
}

pub fn get_command_by_name(name: &str) -> Option<Arc<Command>> {
    COMMANDS.get(name).map(|cmd_ref| Arc::clone(&cmd_ref))
}

pub fn find_command(input: &str) -> Option<Arc<Command>> {
    let graph = get_graph();
    let name = graph.find_command_by_input(input);
    if let Some(name) = name {
        get_command_by_name(&name)
    } else {
        None
    }
}
