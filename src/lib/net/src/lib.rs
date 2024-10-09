use std::sync::Arc;
use ferrumc_ecs::Universe;
use ferrumc_macros::bake_packet_registry;

pub mod errors;
pub mod packets;
pub mod connection;
pub mod server;
pub type NetResult<T> = Result<T, errors::NetError>;


pub struct ServerState {
    universe: Universe
}

pub type GlobalState = Arc<ServerState>;

impl ServerState {
    pub fn new(universe: Universe) -> Self {
        Self {
            universe
        }
    }
}


bake_packet_registry!("\\src\\packets\\incoming");