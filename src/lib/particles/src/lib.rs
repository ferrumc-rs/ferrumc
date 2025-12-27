mod net;

use enum_discriminant::discriminant;
use ferrumc_core::color::Color;
use ferrumc_core::transform::position::Position;
use ferrumc_inventories::slot::InventorySlot;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_world::block_state_id::BlockStateId;

#[discriminant(i32)]
#[derive(Clone)]
/// Enum representing different types of particles in the game.
///
/// To send to clients, use the `SendParticle` message from `ferrumc_messages::particle`.
pub enum ParticleType {
    AngryVillager,
    Block {
        blockstate: BlockStateId,
    },
    BlockMarker {
        blockstate: BlockStateId,
    },
    Bubble,
    Cloud,
    Crit,
    DamageIndicator,
    DragonBreath,
    DrippingLava,
    FallingLava,
    LandingLava,
    DrippingWater,
    FallingWater,
    Dust {
        color: Color,
        scale: f32,
    },
    DustColorTransition {
        from: Color,
        to: Color,
        scale: f32,
    },
    Effect,
    ElderGuardian,
    EnchantedHit,
    Enchant,
    EndRod,
    EntityEffect {
        color: Color,
    },
    ExplosionEmitter,
    Explosion,
    Gust,
    SmallGust,
    GustEmitterLarge,
    GustEmitterSmall,
    SonicBoom,
    FallingDust {
        blockstate: BlockStateId,
    },
    Firework,
    Fishing,
    Flame,
    Infested,
    CherryLeaves,
    PaleOakLeaves,
    TintedLeaves {
        color: Color,
    },
    SculkSoul,
    SculkCharge {
        roll: f32,
    },
    SculkChargePop,
    SoulFireFlame,
    Soul,
    Flash,
    HappyVillager,
    Composter,
    Heart,
    InstantEffect,
    Item {
        item: InventorySlot,
    },

    /// Vibration from a block or an entity.
    Vibration {
        source: VibrationSource,
        /// Ticks it takes for the vibration to travel.
        ticks: VarInt,
    },

    Trail {
        x: f64,
        y: f64,
        z: f64,
        color: Color,
        duration: VarInt,
    },

    ItemSlime,
    ItemCobweb,
    ItemSnowball,
    LargeSmoke,
    Lava,
    Mycelium,
    Note,
    Poof,
    Portal,
    Rain,
    Smoke,
    WhiteSmoke,
    Sneeze,
    Spit,
    SquidInk,
    SweepAttack,
    TotemOfUndying,
    Underwater,
    Splash,
    Witch,
    BubblePop,
    CurrentDown,
    BubbleColumnUp,
    Nautilus,
    Dolphin,
    CampfireCosySmoke,
    CampfireSignalSmoke,
    DrippingHoney,
    FallingHoney,
    LandingHoney,
    FallingNectar,
    FallingSporeBlossom,
    Ash,
    CrimsonSpore,
    WarpedSpore,
    SporeBlossomAir,
    DrippingObsidianTear,
    FallingObsidianTear,
    LandingObsidianTear,
    ReversePortal,
    WhiteAsh,
    SmallFlame,
    Snowflake,
    DrippingDripstoneLava,
    FallingDripstoneLava,
    DrippingDripstoneWater,
    FallingDripstoneWater,
    GlowSquidInk,
    Glow,
    WaxOn,
    WaxOff,
    ElectricSpark,
    Scrape,
    Shriek {
        delay: VarInt,
    },
    EggCrack,
    DustPlume,
    TrialSpawnerDetection,
    TrialSpawnerDetectionOminous,
    VaultConnection,
    DustPillar {
        blockstate: BlockStateId,
    },
    OminousSpawning,
    RaidOmen,
    TrialOmen,
    BlockCrumble {
        blockstate: BlockStateId,
    },
    Firefly,
}

/// Source type for the `vibration` particle.
#[derive(Copy, Clone)]
pub enum VibrationSource {
    Block { position: Position },
    Entity { entity_id: VarInt, eye_height: f32 },
}
