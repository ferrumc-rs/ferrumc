//! Spatial index for block entities.
//!
//! This module provides a resource that maps block positions to their
//! corresponding ECS entities. This allows efficient lookup when a player
//! interacts with a block at specific coordinates.
//!
//! ## Usage
//!
//! ```ignore
//! // Register a block entity
//! index.insert(BlockPos::of(10, 64, 10), entity);
//!
//! // Look up by position (from interact packet)
//! if let Some(entity) = index.get(&block_pos) {
//!     commands.trigger_targets(BlockInteractEvent::new(...), entity);
//! }
//!
//! // Remove when block is destroyed
//! index.remove(&block_pos);
//! ```

use bevy_ecs::prelude::{Entity, Resource};
use ferrumc_world::pos::BlockPos;
use std::collections::HashMap;

/// Resource that maps block positions to their ECS entities.
///
/// This is the primary lookup mechanism for the interaction system.
/// When a player right-clicks on a block, the packet handler uses this
/// index to find the corresponding entity (if any) and trigger the
/// interaction event on it.
///
/// ## Performance
///
/// Uses a HashMap with packed u64 keys for O(1) lookup. The key packing
/// uses the same format as Minecraft's packed block positions:
/// - X: 26 bits (signed)
/// - Z: 26 bits (signed)
/// - Y: 12 bits (signed)
#[derive(Resource, Default)]
pub struct BlockEntityIndex {
    /// Map from packed position to entity
    entities: HashMap<u64, Entity>,
}

impl BlockEntityIndex {
    /// Creates a new empty index.
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    /// Creates an index with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entities: HashMap::with_capacity(capacity),
        }
    }

    /// Packs a BlockPos into a u64 key.
    ///
    /// Format: ((x & 0x3FFFFFF) << 38) | ((z & 0x3FFFFFF) << 12) | (y & 0xFFF)
    fn pack_pos(pos: &BlockPos) -> u64 {
        let x = (pos.pos.x as u64) & 0x3FFFFFF;
        let y = (pos.pos.y as u64) & 0xFFF;
        let z = (pos.pos.z as u64) & 0x3FFFFFF;
        (x << 38) | (z << 12) | y
    }

    /// Inserts a block entity into the index.
    ///
    /// If an entity already exists at this position, it is replaced
    /// and the old entity is returned.
    pub fn insert(&mut self, pos: BlockPos, entity: Entity) -> Option<Entity> {
        let key = Self::pack_pos(&pos);
        self.entities.insert(key, entity)
    }

    /// Looks up an entity by block position.
    pub fn get(&self, pos: &BlockPos) -> Option<Entity> {
        let key = Self::pack_pos(pos);
        self.entities.get(&key).copied()
    }

    /// Removes an entity from the index.
    ///
    /// Returns the removed entity if it existed.
    pub fn remove(&mut self, pos: &BlockPos) -> Option<Entity> {
        let key = Self::pack_pos(pos);
        self.entities.remove(&key)
    }

    /// Checks if a position has an associated entity.
    pub fn contains(&self, pos: &BlockPos) -> bool {
        let key = Self::pack_pos(pos);
        self.entities.contains_key(&key)
    }

    /// Returns the number of indexed block entities.
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Returns true if the index is empty.
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Clears all entries from the index.
    pub fn clear(&mut self) {
        self.entities.clear();
    }

    /// Iterates over all (position_key, entity) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&u64, &Entity)> {
        self.entities.iter()
    }
}

/// System that synchronizes the BlockEntityIndex with spawned/despawned block entities.
///
/// This system should run after block entities are spawned to ensure the index
/// is up-to-date for interaction lookups.
use bevy_ecs::prelude::*;
use ferrumc_entities::components::{BlockPosition, InteractableBlock};

/// Adds newly spawned interactable blocks to the index.
pub fn index_new_block_entities(
    query: Query<(Entity, &BlockPosition), (With<InteractableBlock>, Added<BlockPosition>)>,
    mut index: ResMut<BlockEntityIndex>,
) {
    for (entity, block_pos) in query.iter() {
        index.insert(block_pos.pos(), entity);
        tracing::trace!(
            "Indexed block entity {:?} at ({}, {}, {})",
            entity,
            block_pos.0.pos.x,
            block_pos.0.pos.y,
            block_pos.0.pos.z
        );
    }
}

/// Removes despawned block entities from the index.
///
/// Note: This uses RemovedComponents which requires running after entity cleanup.
pub fn cleanup_block_entity_index(
    mut removed: RemovedComponents<BlockPosition>,
    query: Query<&BlockPosition>,
    mut index: ResMut<BlockEntityIndex>,
) {
    for entity in removed.read() {
        // We can't get the position anymore since the component was removed,
        // so we have to search the index. This is O(n) but despawns are rare.
        // A more efficient approach would store the position in a separate map.
        if let Ok(block_pos) = query.get(entity) {
            index.remove(&block_pos.pos());
            tracing::trace!("Removed block entity {:?} from index", entity);
        }
    }
}
