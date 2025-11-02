//! Command infrastructure for registering, managing, and retrieving server commands.

use bevy_ecs::{prelude::*, schedule::ScheduleConfigs, system::ScheduleSystem};
use dashmap::DashMap;
use std::{
    cell::RefCell,
    sync::{Arc, LazyLock, RwLock},
};

use crate::{graph::CommandGraph, Command};

/// Global map of registered commands keyed by their name.
static COMMANDS: LazyLock<DashMap<&'static str, Arc<Command>>> = LazyLock::new(DashMap::new);

/// Global command graph for efficient command lookup and parsing.
static COMMAND_GRAPH: LazyLock<RwLock<CommandGraph>> =
    LazyLock::new(|| RwLock::new(CommandGraph::default()));

thread_local! {
    static SYSTEMS_TO_BE_REGISTERED: RefCell<Vec<ScheduleConfigs<ScheduleSystem>>> = RefCell::new(Vec::new());
}

/// Adds a command system to be registered later.
///
/// # Note
/// This is an internal function and is normally used by command infrastructure modules.
///
/// # Type Parameters
/// - `M`: Marker type for the system (usually inferred).
#[doc(hidden)]
pub fn add_system<M>(system: impl IntoScheduleConfigs<ScheduleSystem, M>) {
    SYSTEMS_TO_BE_REGISTERED.with(|systems| {
        systems.borrow_mut().push(system.into_configs());
    });
}

/// Registers all command systems into the given ECS `Schedule`.
///
/// # Note
/// This should be called once during the server setup phase to ensure all command systems are scheduled.
///
/// # Arguments
/// - `schedule`: The Bevy ECS schedule to which command systems will be added.
#[doc(hidden)]
pub fn register_command_systems(schedule: &mut Schedule) {
    SYSTEMS_TO_BE_REGISTERED.with(|systems| {
        let mut systems = systems.borrow_mut();
        while let Some(sys) = systems.pop() {
            schedule.add_systems(sys);
        }
    });
}

/// Registers a command in the global command map and graph.
///
/// # Arguments
/// - `command`: An `Arc<Command>` representing the command to register.
pub fn register_command(command: Arc<Command>) {
    COMMANDS.insert(command.name, command.clone());
    if let Ok(mut graph) = COMMAND_GRAPH.write() {
        graph.push(command);
    }
}

/// Returns a clone of the current command graph.
///
/// # Returns
/// A `CommandGraph` containing all registered commands.
pub fn get_graph() -> CommandGraph {
    if let Ok(graph) = COMMAND_GRAPH.read() {
        graph.clone()
    } else {
        CommandGraph::default()
    }
}

/// Retrieves a command by its exact `name`.
///
/// # Arguments
/// - `name`: The name of the command to find.
///
/// # Returns
/// `Some(Arc<Command>)` if the command exists, otherwise `None`.
pub fn get_command_by_name(name: &str) -> Option<Arc<Command>> {
    COMMANDS.get(name).map(|cmd_ref| Arc::clone(&cmd_ref))
}

/// Finds a command by parsing an input string.
///
/// # Arguments
/// - `input`: The input string from which to resolve the command.
///
/// # Returns
/// `Some(Arc<Command>)` if a command matches the input, otherwise `None`.
pub fn find_command(input: &str) -> Option<Arc<Command>> {
    let graph = get_graph();
    let name = graph.find_command_by_input(input);
    if let Some(name) = name {
        get_command_by_name(&name)
    } else {
        None
    }
}
