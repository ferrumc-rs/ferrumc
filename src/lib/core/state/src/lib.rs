use ferrumc_ecs::Universe;
use ferrumc_world::World;
use ferrumc_world_gen::WorldGenerator;
use std::net::TcpListener;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct ServerState {
    pub universe: Universe,
    pub tcp_listener: TcpListener,
    pub world: World,
    pub terrain_generator: WorldGenerator,
    pub shut_down: AtomicBool,
}

pub type GlobalState = Arc<ServerState>;
