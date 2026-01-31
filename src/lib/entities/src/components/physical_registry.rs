use std::collections::HashMap;

use bevy_ecs::prelude::Resource;

use super::physical::PhysicalProperties;
use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

/// Key for the physical properties registry: (protocol_id, is_baby)
pub type PhysicalPropertiesKey = (u16, bool);

/// Baby scale factor (vanilla Minecraft uses 0.5 for most baby mobs)
const BABY_SCALE: f64 = 0.5;

/// Registry of physical properties, indexed by (protocol_id, is_baby).
///
/// This avoids storing 48 bytes of PhysicalProperties per entity.
/// Instead, entities just look up their properties from this registry
/// using `EntityMetadata::protocol_id()`.
///
/// With ~80 entity types × 2 (adult/baby) = ~160 entries × 48 bytes = ~7.6 KB total
/// versus 48 bytes × number of entities with the old approach.
#[derive(Resource)]
pub struct PhysicalRegistry {
    properties: HashMap<PhysicalPropertiesKey, PhysicalProperties>,
}

impl Default for PhysicalRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicalRegistry {
    pub fn new() -> Self {
        let mut properties = HashMap::new();

        let entity_types: &[&VanillaEntityType] = &[
            // Passive
            &VanillaEntityType::ALLAY,
            &VanillaEntityType::ARMADILLO,
            &VanillaEntityType::AXOLOTL,
            &VanillaEntityType::BAT,
            &VanillaEntityType::CAMEL,
            &VanillaEntityType::CAT,
            &VanillaEntityType::CHICKEN,
            &VanillaEntityType::COD,
            &VanillaEntityType::COW,
            &VanillaEntityType::DONKEY,
            &VanillaEntityType::FROG,
            &VanillaEntityType::GLOW_SQUID,
            &VanillaEntityType::HORSE,
            &VanillaEntityType::MOOSHROOM,
            &VanillaEntityType::MULE,
            &VanillaEntityType::OCELOT,
            &VanillaEntityType::PARROT,
            &VanillaEntityType::PIG,
            &VanillaEntityType::RABBIT,
            &VanillaEntityType::SALMON,
            &VanillaEntityType::SHEEP,
            &VanillaEntityType::SKELETON_HORSE,
            &VanillaEntityType::SNIFFER,
            &VanillaEntityType::SNOW_GOLEM,
            &VanillaEntityType::SQUID,
            &VanillaEntityType::STRIDER,
            &VanillaEntityType::TADPOLE,
            &VanillaEntityType::TROPICAL_FISH,
            &VanillaEntityType::TURTLE,
            &VanillaEntityType::VILLAGER,
            &VanillaEntityType::WANDERING_TRADER,
            &VanillaEntityType::ZOMBIE_HORSE,
            // Neutral
            &VanillaEntityType::BEE,
            &VanillaEntityType::CAVE_SPIDER,
            &VanillaEntityType::DOLPHIN,
            &VanillaEntityType::DROWNED,
            &VanillaEntityType::ENDERMAN,
            &VanillaEntityType::FOX,
            &VanillaEntityType::GOAT,
            &VanillaEntityType::IRON_GOLEM,
            &VanillaEntityType::LLAMA,
            &VanillaEntityType::PANDA,
            &VanillaEntityType::PIGLIN,
            &VanillaEntityType::POLAR_BEAR,
            &VanillaEntityType::PUFFERFISH,
            &VanillaEntityType::SPIDER,
            &VanillaEntityType::TRADER_LLAMA,
            &VanillaEntityType::WOLF,
            &VanillaEntityType::ZOMBIFIED_PIGLIN,
            // Hostile
            &VanillaEntityType::BLAZE,
            &VanillaEntityType::BOGGED,
            &VanillaEntityType::BREEZE,
            &VanillaEntityType::CREAKING,
            &VanillaEntityType::CREEPER,
            &VanillaEntityType::ELDER_GUARDIAN,
            &VanillaEntityType::ENDERMITE,
            &VanillaEntityType::EVOKER,
            &VanillaEntityType::GHAST,
            &VanillaEntityType::GUARDIAN,
            &VanillaEntityType::HOGLIN,
            &VanillaEntityType::HUSK,
            &VanillaEntityType::MAGMA_CUBE,
            &VanillaEntityType::PHANTOM,
            &VanillaEntityType::PIGLIN_BRUTE,
            &VanillaEntityType::PILLAGER,
            &VanillaEntityType::RAVAGER,
            &VanillaEntityType::SHULKER,
            &VanillaEntityType::SILVERFISH,
            &VanillaEntityType::SKELETON,
            &VanillaEntityType::SLIME,
            &VanillaEntityType::STRAY,
            &VanillaEntityType::VEX,
            &VanillaEntityType::VINDICATOR,
            &VanillaEntityType::WARDEN,
            &VanillaEntityType::WITCH,
            &VanillaEntityType::WITHER_SKELETON,
            &VanillaEntityType::ZOGLIN,
            &VanillaEntityType::ZOMBIE,
            &VanillaEntityType::ZOMBIE_VILLAGER,
        ];

        for vanilla in entity_types {
            let id = vanilla.id;

            // Adult version
            let adult_props = PhysicalProperties::from_vanilla(vanilla);
            properties.insert((id, false), adult_props);

            // Baby version (scaled)
            let mut baby_props = PhysicalProperties::from_vanilla(vanilla);
            baby_props.apply_scale(BABY_SCALE);
            properties.insert((id, true), baby_props);
        }

        Self { properties }
    }

    /// Get physical properties for an entity type.
    ///
    /// Use `EntityMetadata::protocol_id()` to get the ID.
    /// /!\ Using protocol ID because EntityType from ferrumc_data do not implement Copy/Hash
    /// And create dependency cyle.
    pub fn get(&self, protocol_id: u16, is_baby: bool) -> Option<&PhysicalProperties> {
        self.properties.get(&(protocol_id, is_baby))
    }

    /// Get physical properties with adult fallback.
    pub fn get_or_adult(&self, protocol_id: u16, is_baby: bool) -> Option<&PhysicalProperties> {
        self.properties
            .get(&(protocol_id, is_baby))
            .or_else(|| self.properties.get(&(protocol_id, false)))
    }
}
