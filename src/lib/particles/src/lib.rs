mod net;

use enum_discriminant::discriminant;
use ferrumc_core::color::Color;
use ferrumc_core::transform::position::Position;
use ferrumc_inventories::slot::InventorySlot;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_world::block_state_id::BlockStateId;

#[discriminant(i32)]
#[derive(Clone)]
pub enum ParticleType {
    AngryVillager = 0,
    Block {
        blockstate: BlockStateId,
    } = 1,
    BlockMarker {
        blockstate: BlockStateId,
    } = 2,
    Bubble = 3,
    Cloud = 4,
    CopperFireFlame = 5,
    Crit = 6,
    DamageIndicator = 7,
    DragonBreath {
        power: f32,
    } = 8,
    DrippingLava = 9,
    FallingLava = 10,
    LandingLava = 11,
    DrippingWater = 12,
    FallingWater = 13,
    Dust {
        color: Color,
        scale: f32,
    } = 14,
    DustColorTransition {
        from: Color,
        to: Color,
        scale: f32,
    } = 15,
    Effect {
        color: Color,
        power: f32,
    } = 16,
    ElderGuardian = 17,
    EnchantedHit = 18,
    Enchant = 19,
    EndRod = 20,
    EntityEffect {
        color: Color,
    } = 21,
    ExplosionEmitter = 22,
    Explosion = 23,
    Gust = 24,
    SmallGust = 25,
    GustEmitterLarge = 26,
    GustEmitterSmall = 27,
    SonicBoom = 28,
    FallingDust {
        blockstate: BlockStateId,
    } = 29,
    Firework = 30,
    Fishing = 31,
    Flame = 32,
    Infested = 33,
    CherryLeaves = 34,
    PaleOakLeaves = 35,
    TintedLeaves {
        color: Color,
    } = 36,
    SculkSoul = 37,
    SculkCharge {
        roll: f32,
    } = 38,
    SculkChargePop = 39,
    SoulFireFlame = 40,
    Soul = 41,
    Flash {
        color: Color,
    } = 42,
    HappyVillager = 43,
    Composter = 44,
    Heart = 45,
    InstantEffect {
        color: Color,
        power: f32,
    } = 46,
    Item {
        item: InventorySlot,
    } = 47,

    /// Vibration from a block or an entity.
    Vibration {
        source: VibrationSource,
        /// Ticks it takes for the vibration to travel.
        ticks: VarInt,
    } = 48,

    Trail {
        x: f64,
        y: f64,
        z: f64,
        color: Color,
        duration: VarInt,
    } = 49,

    ItemSlime = 50,
    ItemCobweb = 51,
    ItemSnowball = 52,
    LargeSmoke = 53,
    Lava = 54,
    Mycelium = 55,
    Note = 56,
    Poof = 57,
    Portal = 58,
    Rain = 59,
    Smoke = 60,
    WhiteSmoke = 61,
    Sneeze = 62,
    Spit = 63,
    SquidInk = 64,
    SweepAttack = 65,
    TotemOfUndying = 66,
    Underwater = 67,
    Splash = 68,
    Witch = 69,
    BubblePop = 70,
    CurrentDown = 71,
    BubbleColumnUp = 72,
    Nautilus = 73,
    Dolphin = 74,
    CampfireCosySmoke = 75,
    CampfireSignalSmoke = 76,
    DrippingHoney = 77,
    FallingHoney = 78,
    LandingHoney = 79,
    FallingNectar = 80,
    FallingSporeBlossom = 81,
    Ash = 82,
    CrimsonSpore = 83,
    WarpedSpore = 84,
    SporeBlossomAir = 85,
    DrippingObsidianTear = 86,
    FallingObsidianTear = 87,
    LandingObsidianTear = 88,
    ReversePortal = 89,
    WhiteAsh = 90,
    SmallFlame = 91,
    Snowflake = 92,
    DrippingDripstoneLava = 93,
    FallingDripstoneLava = 94,
    DrippingDripstoneWater = 95,
    FallingDripstoneWater = 96,
    GlowSquidInk = 97,
    Glow = 98,
    WaxOn = 99,
    WaxOff = 100,
    ElectricSpark = 101,
    Scrape = 102,
    Shriek {
        delay: VarInt,
    } = 103,
    EggCrack = 104,
    DustPlume = 105,
    TrialSpawnerDetection = 106,
    TrialSpawnerDetectionOminous = 107,
    VaultConnection = 108,
    DustPillar {
        blockstate: BlockStateId,
    } = 109,
    OminousSpawning = 110,
    RaidOmen = 111,
    TrialOmen = 112,
    BlockCrumble {
        blockstate: BlockStateId,
    } = 113,
    Firefly = 114,
}

/// Source type for the `vibration` particle.
#[derive(Copy, Clone)]
pub enum VibrationSource {
    Block { position: Position },
    Entity { entity_id: VarInt, eye_height: f32 },
}
