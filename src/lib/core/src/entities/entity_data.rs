use super::category::EntityCategory;
use serde::{Deserialize, Serialize};

/// Represents the static "blueprint" data for an entity type.
/// This matches the structure of `assets/extracted/entities.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityBaseData {
    pub id: u32,
    pub attackable: bool,
    pub mob: bool,
    pub summonable: bool,
    pub fire_immune: bool,
    pub category: EntityCategory,
    /// [width, height]
    pub dimension: [f32; 2],
    pub eye_height: f32,
}

impl EntityBaseData {
    pub fn width(&self) -> f32 {
        self.dimension[0]
    }

    pub fn height(&self) -> f32 {
        self.dimension[1]
    }
}
