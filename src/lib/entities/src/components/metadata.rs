use bevy_ecs::prelude::Component;
use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

/// Static entity metadata from ferrumc-data.
///
/// This component stores a reference to immutable vanilla data
/// defined in ferrumc-data. These data come directly from
/// Minecraft data files and never change during runtime.
///
/// # Examples
///
/// ```ignore
/// use ferrumc_entities::components::EntityMetadata;
/// use ferrumc_data::generated::entities::EntityType as VanillaEntityType;
///
/// let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::PIG);
/// assert_eq!(metadata.protocol_id(), 95);
/// assert_eq!(metadata.resource_name(), "pig");
/// assert!(metadata.is_mob());
/// ```
#[derive(Component, Clone, Copy)]
pub struct EntityMetadata {
    /// Static reference to vanilla data for this entity.
    /// This reference points to a constant defined in ferrumc-data
    /// and is valid for the entire program duration.
    vanilla_data: &'static VanillaEntityType,
}

impl EntityMetadata {
    /// Creates a new EntityMetadata component from vanilla data.
    ///
    /// # Arguments
    ///
    /// * `data` - Static reference to entity vanilla data
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ferrumc_data::generated::entities::EntityType as VanillaEntityType;
    ///
    /// let pig_metadata = EntityMetadata::from_vanilla(&VanillaEntityType::PIG);
    /// ```
    pub const fn from_vanilla(data: &'static VanillaEntityType) -> Self {
        Self { vanilla_data: data }
    }

    /// Access complete vanilla data.
    ///
    /// Allows access to all vanilla data fields if needed.
    pub const fn vanilla_data(&self) -> &'static VanillaEntityType {
        self.vanilla_data
    }

    /// Returns the protocol ID of this entity.
    ///
    /// This ID is used in network packets to identify the entity type.
    /// For example, the pig ID is 95.
    pub const fn protocol_id(&self) -> u16 {
        self.vanilla_data.id
    }

    /// Returns the resource name of this entity.
    ///
    /// The resource name is the Minecraft identifier without the "minecraft:" namespace.
    /// For example: "pig", "zombie", "creeper".
    pub const fn resource_name(&self) -> &'static str {
        self.vanilla_data.resource_name
    }

    /// Returns true if this entity is a mob.
    ///
    /// Mobs are living entities (animals, monsters).
    /// Non-mobs include items, projectiles, boats, etc.
    pub const fn is_mob(&self) -> bool {
        self.vanilla_data.mob
    }

    /// Returns true if this entity can be saved.
    ///
    /// Saveable entities persist in the world when
    /// the chunk is unloaded.
    pub const fn is_saveable(&self) -> bool {
        self.vanilla_data.saveable
    }

    /// Returns true if this entity can be summoned with /summon.
    pub const fn is_summonable(&self) -> bool {
        self.vanilla_data.summonable
    }

    /// Returns true if this entity is immune to fire.
    pub const fn is_fire_immune(&self) -> bool {
        self.vanilla_data.fire_immune
    }

    /// Returns the limit of entities of this type per chunk.
    ///
    /// Used by the spawn system to limit the number of entities.
    /// A value of 0 means no limit.
    pub const fn limit_per_chunk(&self) -> i32 {
        self.vanilla_data.limit_per_chunk
    }

    /// Returns true if this entity can spawn far from the player.
    pub const fn can_spawn_far_from_player(&self) -> bool {
        self.vanilla_data.can_spawn_far_from_player
    }
}

impl std::fmt::Debug for EntityMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityMetadata")
            .field("resource_name", &self.resource_name())
            .field("protocol_id", &self.protocol_id())
            .field("is_mob", &self.is_mob())
            .finish()
    }
}
