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

    #[derive(Component)]
    pub struct Allay;
    #[derive(Component)]
    pub struct Armadillo;
    #[derive(Component)]
    pub struct Axolotl;
    #[derive(Component)]
    pub struct Bat;
    #[derive(Component)]
    pub struct Bee;
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
    pub struct Dolphin;
    #[derive(Component)]
    pub struct Donkey;
    #[derive(Component)]
    pub struct Fox;
    #[derive(Component)]
    pub struct Frog;
    #[derive(Component)]
    pub struct Goat;
    #[derive(Component)]
    pub struct Horse;
    #[derive(Component)]
    pub struct Llama;
    #[derive(Component)]
    pub struct Mooshroom;
    #[derive(Component)]
    pub struct Ocelot;
    #[derive(Component)]
    pub struct Panda;
    #[derive(Component)]
    pub struct Parrot;
    #[derive(Component)]
    pub struct Pig;
    #[derive(Component)]
    pub struct PolarBear;
    #[derive(Component)]
    pub struct Pufferfish;
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
    pub struct Wolf;
    #[derive(Component)]
    pub struct ZombieHorse;
    #[derive(Component)]
    pub struct GlowSquid;
    #[derive(Component)]
    pub struct Mule;
}
