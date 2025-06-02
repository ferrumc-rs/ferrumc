use bevy_ecs::prelude::Resource;
use dashmap::DashMap;
use ferrumc_threadpool::ThreadPool;
use ferrumc_world::World;
use ferrumc_world_gen::WorldGenerator;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct ServerState {
    pub world: World,
    pub terrain_generator: WorldGenerator,
    pub shut_down: AtomicBool,
    pub players: DashMap<u128, String>,
    pub thread_pool: ThreadPool,
}

pub type GlobalState = Arc<ServerState>;

#[derive(Resource, Clone)]
pub struct GlobalStateResource(pub GlobalState);
