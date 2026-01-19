pub mod player_list;

use crate::player_list::PlayerList;
use bevy_ecs::prelude::Resource;
use ferrumc_threadpool::ThreadPool;
use ferrumc_world::World;
use ferrumc_world_gen::WorldGenerator;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;
use tempfile::TempDir;

pub struct ServerState {
    pub world: World,
    pub terrain_generator: WorldGenerator,
    pub shut_down: AtomicBool,
    pub players: PlayerList, // (UUID, Username)
    pub thread_pool: ThreadPool,
    pub start_time: Instant,
}

pub type GlobalState = Arc<ServerState>;

#[derive(Resource, Clone)]
pub struct GlobalStateResource(pub GlobalState);

/// Creates a minimal GlobalStateResource for testing with a temporary database
pub fn create_test_state() -> (GlobalStateResource, TempDir) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let db_path = temp_dir.path().to_path_buf();

    let server_state = ServerState {
        world: World::new(&db_path),
        terrain_generator: WorldGenerator::new(0),
        shut_down: false.into(),
        players: PlayerList::default(),
        thread_pool: ThreadPool::new(),
        start_time: Instant::now(),
    };

    let global_state = Arc::new(server_state);
    (GlobalStateResource(global_state), temp_dir)
}
