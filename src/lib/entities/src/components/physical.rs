use bevy_ecs::prelude::Component;
use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

/// Entity bounding box (collision box).
///
/// Represents the volume occupied by an entity in the world.
/// Used for collision detection and physics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    /// Half-width of the bounding box in blocks.
    ///
    /// The total width is `half_width * 2`. Stored as half-width
    /// to facilitate collision calculations (distance from center to edges).
    pub half_width: f64,

    /// Height of the bounding box in blocks.
    pub height: f64,
}

impl BoundingBox {
    /// Creates a new bounding box from vanilla dimensions.
    ///
    /// # Arguments
    ///
    /// * `dimension` - Dimensions [width, height] from ferrumc-data
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ferrumc_data::generated::entities::EntityType as VanillaEntityType;
    ///
    /// let bbox = BoundingBox::from_vanilla_dimension(VanillaEntityType::PIG.dimension);
    /// assert_eq!(bbox.half_width, 0.45); // 0.9 / 2
    /// assert_eq!(bbox.height, 0.9);
    /// ```
    pub const fn from_vanilla_dimension(dimension: [f32; 2]) -> Self {
        Self {
            half_width: (dimension[0] / 2.0) as f64,
            height: dimension[1] as f64,
        }
    }

    /// Returns the total width of the bounding box.
    pub const fn width(&self) -> f64 {
        self.half_width * 2.0
    }

    /// Returns the volume of the bounding box in cubic blocks.
    pub const fn volume(&self) -> f64 {
        self.width() * self.width() * self.height
    }
}

/// Physical properties of an entity.
///
/// These properties are derived from vanilla data but can be modified
/// by gameplay effects (baby, crouching, potions, etc.).
///
/// # Examples
///
/// ```ignore
/// use ferrumc_entities::components::{EntityMetadata, PhysicalProperties};
/// use ferrumc_data::generated::entities::EntityType as VanillaEntityType;
///
/// let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::PIG);
/// let physical = PhysicalProperties::from_metadata(&metadata);
///
/// assert_eq!(physical.bounding_box.height, 0.9);
/// assert_eq!(physical.eye_height, 0.765);
/// assert!(!physical.fire_immune);
/// ```
#[derive(Component, Clone, Copy)]
pub struct PhysicalProperties {
    /// Bounding box of the entity for collisions.
    ///
    /// Can change if the entity is a baby, crouching, etc.
    pub bounding_box: BoundingBox,

    /// Eye height in blocks from the entity's feet.
    ///
    /// Used for vision calculation, raytracing, and camera position
    /// for players.
    pub eye_height: f32,

    /// True if the entity is immune to fire and lava.
    pub fire_immune: bool,
}

impl PhysicalProperties {
    /// Creates physical properties from vanilla metadata.
    ///
    /// Extracts dimensions, eye_height and fire_immune from vanilla data.
    ///
    /// # Arguments
    ///
    /// * `metadata` - Entity metadata
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::PIG);
    /// let physical = PhysicalProperties::from_metadata(&metadata);
    /// ```
    pub fn from_metadata(metadata: &EntityMetadata) -> Self {
        let data = metadata.vanilla_data();

        Self {
            bounding_box: BoundingBox::from_vanilla_dimension(data.dimension),
            eye_height: data.eye_height,
            fire_immune: data.fire_immune,
        }
    }

    /// Creates directly from vanilla data.
    ///
    /// Shortcut version that doesn't require creating EntityMetadata first.
    pub fn from_vanilla(data: &'static VanillaEntityType) -> Self {
        Self {
            bounding_box: BoundingBox::from_vanilla_dimension(data.dimension),
            eye_height: data.eye_height,
            fire_immune: data.fire_immune,
        }
    }

    /// Applies a scale factor to dimensions (for babies for example).
    ///
    /// # Arguments
    ///
    /// * `scale` - Multiplier factor (0.5 for baby, 1.0 for adult)
    pub fn apply_scale(&mut self, scale: f64) {
        self.bounding_box.half_width *= scale;
        self.bounding_box.height *= scale;
        self.eye_height = (self.eye_height as f64 * scale) as f32;
    }
}

impl std::fmt::Debug for PhysicalProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhysicalProperties")
            .field("bounding_box", &self.bounding_box)
            .field("eye_height", &self.eye_height)
            .field("fire_immune", &self.fire_immune)
            .finish()
    }
}

// Import EntityMetadata for from_metadata methods
use super::metadata::EntityMetadata;
