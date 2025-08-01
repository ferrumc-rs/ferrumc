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

pub fn add_system<M>(system: impl IntoScheduleConfigs<ScheduleSystem, M>) {
    SYSTEMS_TO_BE_REGISTERED.with(|systems| {
        systems.borrow_mut().push(system.into_configs());
    });
}

pub fn register_command_systems(schedule: &mut Schedule) {
    SYSTEMS_TO_BE_REGISTERED.with(|systems| {
        let mut systems = systems.borrow_mut();
        while let Some(sys) = systems.pop() {
            schedule.add_systems(sys);
        }
    });
}

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
