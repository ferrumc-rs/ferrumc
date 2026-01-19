//! Command infrastructure

use bevy_ecs::{prelude::*, schedule::ScheduleConfigs, system::ScheduleSystem};
use dashmap::DashMap;
use std::{
    cell::RefCell,
    sync::{Arc, LazyLock, RwLock},
};

use crate::{graph::CommandGraph, Command};

static COMMANDS: LazyLock<DashMap<&'static str, Arc<Command>>> = LazyLock::new(DashMap::new);
static COMMAND_GRAPH: LazyLock<RwLock<CommandGraph>> =
    LazyLock::new(|| RwLock::new(CommandGraph::default()));

thread_local! {
    static SYSTEMS_TO_BE_REGISTERED: RefCell<Vec<ScheduleConfigs<ScheduleSystem>>> = RefCell::new(Vec::new());
}

/// Internal function. Adds a command system.
#[doc(hidden)]
pub fn add_system<M>(system: impl IntoScheduleConfigs<ScheduleSystem, M>) {
    SYSTEMS_TO_BE_REGISTERED.with(|systems| {
        systems.borrow_mut().push(system.into_configs());
    });
}

/// Internal function. Registers all command systems.
#[doc(hidden)]
pub fn register_command_systems(schedule: &mut Schedule) {
    SYSTEMS_TO_BE_REGISTERED.with(|systems| {
        let mut systems = systems.borrow_mut();
        while let Some(sys) = systems.pop() {
            schedule.add_systems(sys);
        }
    });
}

/// Registers a command.
pub fn register_command(command: Arc<Command>) {
    COMMANDS.insert(command.name, command.clone());
    if let Ok(mut graph) = COMMAND_GRAPH.write() {
        graph.push(command);
    }
}

/// Gets the server's command graph.
pub fn get_graph() -> CommandGraph {
    match COMMAND_GRAPH.read() { Ok(graph) => {
        graph.clone()
    } _ => {
        CommandGraph::default()
    }}
}

/// Attempts to find a command by its `name`.
pub fn get_command_by_name(name: &str) -> Option<Arc<Command>> {
    COMMANDS.get(name).map(|cmd_ref| Arc::clone(&cmd_ref))
}

/// Attempts to find a command by an `input` string.
pub fn find_command(input: &str) -> Option<Arc<Command>> {
    let graph = get_graph();
    let name = graph.find_command_by_input(input);
    if let Some(name) = name {
        get_command_by_name(&name)
    } else {
        None
    }
}
