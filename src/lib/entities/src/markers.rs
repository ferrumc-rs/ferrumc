use bevy_ecs::prelude::Component;

/// Marker components for entities
#[derive(Component)]
pub struct HasGravity;

#[derive(Component)]
pub struct HasWaterDrag;

#[derive(Component)]
pub struct HasCollisions;

// Entity types
pub mod entity_types {
    use super::Component;

    // Passive entities
    #[derive(Component)]
    pub struct Allay;
    #[derive(Component)]
    pub struct Armadillo;
    #[derive(Component)]
    pub struct Axolotl;
    #[derive(Component)]
    pub struct Bat;
    #[derive(Component)]
    pub struct Camel;
    #[derive(Component)]
    pub struct Cat;
    #[derive(Component)]
    pub struct Chicken;
    #[derive(Component)]
    pub struct Cod;
    #[derive(Component)]
    pub struct Cow;
    #[derive(Component)]
    pub struct Donkey;
    #[derive(Component)]
    pub struct Frog;
    #[derive(Component)]
    pub struct GlowSquid;
    #[derive(Component)]
    pub struct Horse;
    #[derive(Component)]
    pub struct Mooshroom;
    #[derive(Component)]
    pub struct Mule;
    #[derive(Component)]
    pub struct Ocelot;
    #[derive(Component)]
    pub struct Parrot;
    #[derive(Component)]
    pub struct Pig;
    #[derive(Component)]
    pub struct Rabbit;
    #[derive(Component)]
    pub struct Salmon;
    #[derive(Component)]
    pub struct Sheep;
    #[derive(Component)]
    pub struct SkeletonHorse;
    #[derive(Component)]
    pub struct Sniffer;
    #[derive(Component)]
    pub struct SnowGolem;
    #[derive(Component)]
    pub struct Squid;
    #[derive(Component)]
    pub struct Strider;
    #[derive(Component)]
    pub struct Tadpole;
    #[derive(Component)]
    pub struct TropicalFish;
    #[derive(Component)]
    pub struct Turtle;
    #[derive(Component)]
    pub struct Villager;
    #[derive(Component)]
    pub struct WanderingTrader;
    #[derive(Component)]
    pub struct ZombieHorse;

    // Neutral entities
    #[derive(Component)]
    pub struct Bee;
    #[derive(Component)]
    pub struct CaveSpider;
    #[derive(Component)]
    pub struct Dolphin;
    #[derive(Component)]
    pub struct Drowned;
    #[derive(Component)]
    pub struct Enderman;
    #[derive(Component)]
    pub struct Fox;
    #[derive(Component)]
    pub struct Goat;
    #[derive(Component)]
    pub struct IronGolem;
    #[derive(Component)]
    pub struct Llama;
    #[derive(Component)]
    pub struct Panda;
    #[derive(Component)]
    pub struct Piglin;
    #[derive(Component)]
    pub struct PolarBear;
    #[derive(Component)]
    pub struct Pufferfish;
    #[derive(Component)]
    pub struct Spider;
    #[derive(Component)]
    pub struct TraderLlama;
    #[derive(Component)]
    pub struct Wolf;
    #[derive(Component)]
    pub struct ZombifiedPiglin;

    // Hostile entities
    #[derive(Component)]
    pub struct Blaze;
    #[derive(Component)]
    pub struct Bogged;
    #[derive(Component)]
    pub struct Breeze;
    #[derive(Component)]
    pub struct Creaking;
    #[derive(Component)]
    pub struct Creeper;
    #[derive(Component)]
    pub struct ElderGuardian;
    #[derive(Component)]
    pub struct Endermite;
    #[derive(Component)]
    pub struct Evoker;
    #[derive(Component)]
    pub struct Ghast;
    #[derive(Component)]
    pub struct Guardian;
    #[derive(Component)]
    pub struct Hoglin;
    #[derive(Component)]
    pub struct Husk;
    #[derive(Component)]
    pub struct MagmaCube;
    #[derive(Component)]
    pub struct Phantom;
    #[derive(Component)]
    pub struct PiglinBrute;
    #[derive(Component)]
    pub struct Pillager;
    #[derive(Component)]
    pub struct Ravager;
    #[derive(Component)]
    pub struct Shulker;
    #[derive(Component)]
    pub struct Silverfish;
    #[derive(Component)]
    pub struct Skeleton;
    #[derive(Component)]
    pub struct Slime;
    #[derive(Component)]
    pub struct Stray;
    #[derive(Component)]
    pub struct Vex;
    #[derive(Component)]
    pub struct Vindicator;
    #[derive(Component)]
    pub struct Warden;
    #[derive(Component)]
    pub struct Witch;
    #[derive(Component)]
    pub struct WitherSkeleton;
    #[derive(Component)]
    pub struct Zoglin;
    #[derive(Component)]
    pub struct Zombie;
    #[derive(Component)]
    pub struct ZombieVillager;
}
