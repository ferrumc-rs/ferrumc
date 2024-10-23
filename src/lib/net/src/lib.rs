use ferrumc_ecs::Universe;
use ferrumc_macros::bake_packet_registry;
use std::sync::{atomic::AtomicBool, Arc};

pub mod connection;
pub mod errors;
pub mod packets;
pub mod server;
pub mod utils;
pub type NetResult<T> = Result<T, errors::NetError>;

pub struct ServerState {
    pub universe: Universe,
    pub log_packets: AtomicBool,
}

pub type GlobalState = Arc<ServerState>;

impl ServerState {
    pub fn new(universe: Universe, log_packets: AtomicBool) -> Self {
        Self {
            universe,
            log_packets,
        }
    }
}

bake_packet_registry!("\\src\\packets\\incoming");
