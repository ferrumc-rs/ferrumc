pub mod player_cache;
pub mod player_list;

use crate::player_cache::PlayerCache;
use crate::player_list::PlayerList;
use bevy_ecs::prelude::Resource;
use ferrumc_threadpool::ThreadPool;
use ferrumc_world::World;
use ferrumc_world_gen::WorldGenerator;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;

pub struct ServerState {
    pub world: World,
    pub terrain_generator: WorldGenerator,
    pub shut_down: AtomicBool,
    pub players: PlayerList, // (UUID, Username)
    pub player_cache: PlayerCache,
    pub thread_pool: ThreadPool,
    pub start_time: Instant,
}

pub type GlobalState = Arc<ServerState>;

#[derive(Resource, Clone)]
pub struct GlobalStateResource(pub GlobalState);
