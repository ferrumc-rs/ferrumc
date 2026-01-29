use bevy_ecs::prelude::{Entity, Message};
use ferrumc_core::transform::position::Position;

/// Type of entity to spawn
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EntityType {
    // Passive entities
    Allay,
    Armadillo,
    Axolotl,
    Bat,
    Camel,
    Cat,
    Chicken,
    Cod,
    Cow,
    Donkey,
    Frog,
    GlowSquid,
    Horse,
    Mooshroom,
    Mule,
    Ocelot,
    Parrot,
    Pig,
    Rabbit,
    Salmon,
    Sheep,
    SkeletonHorse,
    Sniffer,
    SnowGolem,
    Squid,
    Strider,
    Tadpole,
    TropicalFish,
    Turtle,
    Villager,
    WanderingTrader,
    ZombieHorse,

    // Neutral entities
    Bee,
    CaveSpider,
    Dolphin,
    Drowned,
    Enderman,
    Fox,
    Goat,
    IronGolem,
    Llama,
    Panda,
    Piglin,
    PolarBear,
    Pufferfish,
    Spider,
    TraderLlama,
    Wolf,
    ZombifiedPiglin,

    // Hostile entities
    Blaze,
    Bogged,
    Breeze,
    Creaking,
    Creeper,
    ElderGuardian,
    Endermite,
    Evoker,
    Ghast,
    Guardian,
    Hoglin,
    Husk,
    MagmaCube,
    Phantom,
    PiglinBrute,
    Pillager,
    Ravager,
    Shulker,
    Silverfish,
    Skeleton,
    Slime,
    Stray,
    Vex,
    Vindicator,
    Warden,
    Witch,
    WitherSkeleton,
    Zoglin,
    Zombie,
    ZombieVillager,
}

/// Command to spawn an entity in front of a player.
///
/// This message is written by the /spawn command and processed by
/// the spawn_command_processor system which calculates the spawn position.
#[derive(Message)]
pub struct SpawnEntityCommand {
    pub entity_type: EntityType,
    pub player_entity: Entity,
}

/// Event fired when an entity should be spawned at a specific position.
///
/// This is triggered by spawn_command_processor after calculating
/// the spawn position from the player's position and rotation.
#[derive(Message)]
pub struct SpawnEntityEvent {
    pub entity_type: EntityType,
    pub position: Position,
}
