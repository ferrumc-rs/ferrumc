use bevy_ecs::prelude::Component;
use ferrumc_data::generated::entities::{
    EntityType as VanillaEntityType, HeightMap, MobCategory, SpawnLocation, SpawnRestriction,
};

use super::metadata::EntityMetadata;

/// Property for entity spawn and persistence
///
/// Those properties determine how and where entities can spawn,
/// how many can exists in a chunk if they need to be saved etc ...
///
/// # Examples
///
/// ```ignore
/// use ferrumc_entities::components::{EntityMetadata, SpawnProperties};
/// use ferrumc_data::generated::entities::EntityType as VanillaEntityType;
///
/// let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::PIG);
/// let spawn = SpawnProperties::from_metadata(&metadata);
///
/// assert!(spawn.is_friendly());
/// assert!(spawn.is_persistent());
/// assert_eq!(spawn.limit_per_chunk, 4);
/// ```
#[derive(Component)]
pub struct SpawnProperties {
    /// mob category (CREATURE, MONSTER, AMBIENT, etc.).
    ///
    /// Determine spawn / despawn comportment
    pub category: MobCategory,

    /// True if the entity need to be saved with the chunk
    pub saveable: bool,

    /// Max number of this entity type on a chunk
    ///
    /// 0 = no limit :)
    pub limit_per_chunk: i32,

    /// True if the entity can spawn far from the player
    pub can_spawn_far_from_player: bool,

    /// Spawn restriction (eg. heightmap).
    pub spawn_restriction: SpawnRestriction,
}

// Manual Clone implementation because ferrumc-data types don't implement Clone
impl Clone for SpawnProperties {
    fn clone(&self) -> Self {
        Self {
            category: clone_mob_category(&self.category),
            saveable: self.saveable,
            limit_per_chunk: self.limit_per_chunk,
            can_spawn_far_from_player: self.can_spawn_far_from_player,
            spawn_restriction: clone_spawn_restriction(&self.spawn_restriction),
        }
    }
}

impl SpawnProperties {
    /// Create spawn properties from vanilla metadatas
    pub fn from_metadata(metadata: &EntityMetadata) -> Self {
        let data = metadata.vanilla_data();

        Self {
            category: clone_mob_category(data.category),
            saveable: data.saveable,
            limit_per_chunk: data.limit_per_chunk,
            can_spawn_far_from_player: data.can_spawn_far_from_player,
            spawn_restriction: clone_spawn_restriction(&data.spawn_restriction),
        }
    }

    /// Create directly from vanilla data
    pub fn from_vanilla(data: &'static VanillaEntityType) -> Self {
        Self {
            category: clone_mob_category(data.category),
            saveable: data.saveable,
            limit_per_chunk: data.limit_per_chunk,
            can_spawn_far_from_player: data.can_spawn_far_from_player,
            spawn_restriction: clone_spawn_restriction(&data.spawn_restriction),
        }
    }

    /// Return true if this mob is friendly (Passive)
    pub fn is_friendly(&self) -> bool {
        self.category.is_friendly()
    }

    /// Return true if this mob is persistent (do not despawn)
    pub fn is_persistent(&self) -> bool {
        self.category.is_persistent()
    }

    /// Return the distance of despawn for this mob.
    pub fn despawn_distance(&self) -> i32 {
        self.category.despawn_distance()
    }

    /// Return the max numbers of the entity type per chunk.
    pub fn max_per_chunk(&self) -> i32 {
        self.category.max_per_chunk()
    }

    /// Return true if this mob can spawn at this position (type of block).
    pub fn can_spawn_at_location(&self, location: &SpawnLocation) -> bool {
        matches!(
            (&self.spawn_restriction.location, location),
            (SpawnLocation::OnGround, SpawnLocation::OnGround)
                | (SpawnLocation::InWater, SpawnLocation::InWater)
                | (SpawnLocation::InLava, SpawnLocation::InLava)
                | (SpawnLocation::Unrestricted, _)
        )
    }
}

impl std::fmt::Debug for SpawnProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpawnProperties")
            .field("category", &format!("{:?}", self.category))
            .field("saveable", &self.saveable)
            .field("limit_per_chunk", &self.limit_per_chunk)
            .field("can_spawn_far", &self.can_spawn_far_from_player)
            .field("is_friendly", &self.is_friendly())
            .field("is_persistent", &self.is_persistent())
            .finish()
    }
}

// Helper functions to clone enums from ferrumc-data
// (Because they doesn't implement clone)

fn clone_mob_category(category: &MobCategory) -> MobCategory {
    match category {
        MobCategory::MONSTER => MobCategory::MONSTER,
        MobCategory::CREATURE => MobCategory::CREATURE,
        MobCategory::AMBIENT => MobCategory::AMBIENT,
        MobCategory::AXOLOTLS => MobCategory::AXOLOTLS,
        MobCategory::UNDERGROUND_WATER_CREATURE => MobCategory::UNDERGROUND_WATER_CREATURE,
        MobCategory::WATER_CREATURE => MobCategory::WATER_CREATURE,
        MobCategory::WATER_AMBIENT => MobCategory::WATER_AMBIENT,
        MobCategory::MISC => MobCategory::MISC,
    }
}

fn clone_spawn_restriction(restriction: &SpawnRestriction) -> SpawnRestriction {
    SpawnRestriction {
        location: clone_spawn_location(&restriction.location),
        heightmap: clone_heightmap(&restriction.heightmap),
    }
}

fn clone_spawn_location(location: &SpawnLocation) -> SpawnLocation {
    match location {
        SpawnLocation::InLava => SpawnLocation::InLava,
        SpawnLocation::InWater => SpawnLocation::InWater,
        SpawnLocation::OnGround => SpawnLocation::OnGround,
        SpawnLocation::Unrestricted => SpawnLocation::Unrestricted,
    }
}

fn clone_heightmap(heightmap: &HeightMap) -> HeightMap {
    match heightmap {
        HeightMap::WorldSurfaceWg => HeightMap::WorldSurfaceWg,
        HeightMap::WorldSurface => HeightMap::WorldSurface,
        HeightMap::OceanFloorWg => HeightMap::OceanFloorWg,
        HeightMap::OceanFloor => HeightMap::OceanFloor,
        HeightMap::MotionBlocking => HeightMap::MotionBlocking,
        HeightMap::MotionBlockingNoLeaves => HeightMap::MotionBlockingNoLeaves,
    }
}
