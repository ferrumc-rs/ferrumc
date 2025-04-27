use crate::errors::BinaryError;
use crate::systems::keep_alive_system::KeepAliveSystem;
use crate::systems::send_chunks::ChunkSender;
use ferrumc_state::GlobalState;
use std::sync::Arc;

pub trait System: Send + Sync {
    fn run(self: Arc<Self>, state: GlobalState, tick: u128) -> Result<(), BinaryError>;

    fn name(&self) -> &'static str;
}

pub fn create_systems() -> Vec<Arc<dyn System>> {
    vec![Arc::new(KeepAliveSystem::new()), Arc::new(ChunkSender)]
}
