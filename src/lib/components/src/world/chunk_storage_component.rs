use bevy_ecs::prelude::Component;
// Use your existing chunk logic here
use ferrumc_world::World;
use std::sync::Arc;

/// Component attached to a World Entity.
/// Holds the actual block data/LMDB instance for this specific world.
#[derive(Component, Clone)]
pub struct WorldChunkStorage(pub Arc<World>);
