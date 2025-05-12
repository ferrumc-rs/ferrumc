use bevy_ecs::prelude::*;
use ferrumc_world::World;
use ferrumc_world_gen::WorldGenerator;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct ServerState {
    pub universe: World, // Changed from Universe to bevy's World
    pub world: World,
    pub terrain_generator: WorldGenerator,
    pub shut_down: AtomicBool,
}

pub type GlobalState = Arc<ServerState>;
