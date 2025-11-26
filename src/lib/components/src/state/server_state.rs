use bevy_ecs::prelude::Resource;

use crate::state::player_cache::PlayerCache;
use crate::state::player_list::PlayerList;
use ferrumc_threadpool::ThreadPool;
use ferrumc_world::World;
use ferrumc_world_gen::WorldGenerator;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;

/// The top-level container for all server systems.
/// This is passed to Tokio threads.
pub struct ServerState {
    pub world: World,
    pub players: PlayerList,
    pub terrain_generator: WorldGenerator,
    pub shut_down: AtomicBool,
    pub player_cache: PlayerCache,
    pub thread_pool: ThreadPool,
    pub start_time: Instant,
}

/// The thread-safe wrapper used everywhere.
pub type GlobalState = Arc<ServerState>;

/// The Bevy Resource wrapper for the server's global state Arc.
/// This allows systems to access the GlobalState (read-only reference).
#[derive(Resource, Clone)]
pub struct GlobalStateResource(pub GlobalState);
