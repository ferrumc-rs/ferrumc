use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityData {
    pub protocol_id: u32,
    pub name: &'static str, // "minecraft:zombie"

    // --- Physical Properties ---
    pub width: f32,
    pub height: f32,
    pub eye_height: f32,

    // --- Behavior Flags ---
    pub summonable: bool,
    pub fire_immune: bool,
    pub can_spawn_far_from_player: bool,

    // --- Categories & Limits ---
    pub category: EntityCategory,
    pub limit_per_chunk: i32, // Dynamic limit if applicable, else default

    // --- Spawning Rules ---
    pub spawn_restriction_location: SpawnLocation,
    pub spawn_restriction_heightmap: HeightMap,

    // --- Optional Combat/Stats ---
    pub max_health: Option<f32>,
    pub attackable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum EntityCategory {
    Monster,
    Creature,
    Ambient,
    Axolotls,
    UndergroundWaterCreature,
    WaterCreature,
    WaterAmbient,
    Misc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpawnLocation {
    InLava,
    InWater,
    OnGround,
    Unrestricted,
    NoRestriction, // Fallback
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeightMap {
    WorldSurfaceWg,
    WorldSurface,
    OceanFloorWg,
    OceanFloor,
    MotionBlocking,
    MotionBlockingNoLeaves,
    None, // Fallback
}

// We don't implement logic like `despawn_distance()` here.
// That belongs in a `ferrumc-game` system that queries the category.
