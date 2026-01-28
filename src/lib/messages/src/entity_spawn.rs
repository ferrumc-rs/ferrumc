use bevy_ecs::prelude::{Entity, Message};
use ferrumc_core::transform::position::Position;

/// Type of entity to spawn
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EntityType {
    Allay,
    Armadillo,
    Axolotl,
    Bat,
    Bee,
    Camel,
    Cat,
    CaveSpider,
    Chicken,
    Cod,
    Cow,
    Dolphin,
    Donkey,
    Drowned,
    Enderman,
    Fox,
    Frog,
    Goat,
    Horse,
    IronGolem,
    Llama,
    Mooshroom,
    Ocelot,
    Panda,
    Parrot,
    Pig,
    Piglin,
    PolarBear,
    Pufferfish,
    Rabbit,
    Salmon,
    Sheep,
    SkeletonHorse,
    Sniffer,
    SnowGolem,
    Spider,
    Squid,
    Strider,
    Tadpole,
    TraderLlama,
    TropicalFish,
    Turtle,
    Villager,
    WanderingTrader,
    Wolf,
    ZombieHorse,
    ZombifiedPiglin,
    GlowSquid,
    Mule,
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
