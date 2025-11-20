use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

use crate::components::{CombatProperties, EntityMetadata, PhysicalProperties, SpawnProperties};

/// Trait to create entities from their vanilla data.
///
/// This trait provides helper methods to create basic entity components
/// from ferrumc-data. Concrete implementations (like PigBundle) can use these
/// methods to initialize their components.
///
/// # Examples
///
/// ```ignore
/// use ferrumc_entities::traits::EntityFactory;
/// use ferrumc_data::generated::entities::EntityType as VanillaEntityType;
///
/// let (metadata, physical, combat, spawn) =
///     EntityFactory::create_base_components(&VanillaEntityType::PIG);
///
/// assert_eq!(metadata.protocol_id(), 95);
/// assert_eq!(physical.bounding_box.height, 0.9);
/// ```
pub trait EntityFactory {
    /// Creates base components for an entity from its vanilla data.
    ///
    /// This method creates the 4 main components derived from vanilla data:
    /// - EntityMetadata: immutable metadata
    /// - PhysicalProperties: physical properties
    /// - CombatProperties: combat properties
    /// - SpawnProperties: spawn properties
    ///
    /// # Arguments
    ///
    /// * `vanilla_data` - Static reference to entity vanilla data
    ///
    /// # Returns
    ///
    /// A tuple containing (EntityMetadata, PhysicalProperties, CombatProperties, SpawnProperties)
    fn create_base_components(
        vanilla_data: &'static VanillaEntityType,
    ) -> (
        EntityMetadata,
        PhysicalProperties,
        CombatProperties,
        SpawnProperties,
    ) {
        let metadata = EntityMetadata::from_vanilla(vanilla_data);

        (
            metadata,
            PhysicalProperties::from_metadata(&metadata),
            CombatProperties::from_metadata(&metadata),
            SpawnProperties::from_metadata(&metadata),
        )
    }

    /// Creates just the metadata for an entity.
    ///
    /// Useful if you only need vanilla metadata.
    fn create_metadata(vanilla_data: &'static VanillaEntityType) -> EntityMetadata {
        EntityMetadata::from_vanilla(vanilla_data)
    }
}

/// Default implementation for all types.
///
/// Allows any type to use EntityFactory trait methods
/// without needing to explicitly implement the trait.
impl<T> EntityFactory for T {}
