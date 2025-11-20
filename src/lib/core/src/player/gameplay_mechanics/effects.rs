use serde::{Deserialize, Serialize};

/// Represents a specific instance of an active effect on a player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectData {
    /// The strength of the effect (0 = Level I, 1 = Level II, etc.)
    pub amplifier: u8,
    /// Time remaining in ticks (20 ticks = 1 second).
    pub duration: u32,
    /// Whether particles should be shown.
    pub show_particles: bool,
    /// Whether the icon should be shown in the top-right.
    pub show_icon: bool,
}

/// All vanilla Minecraft status effects.
/// (We can use u8 or a proper Enum here. An Enum is safer/cleaner).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum EffectType {
    Speed = 1,
    Slowness = 2,
    Haste = 3,
    MiningFatigue = 4,
    Strength = 5,
    InstantHealth = 6,
    InstantDamage = 7,
    JumpBoost = 8,
    Nausea = 9,
    Regeneration = 10,
    Resistance = 11,
    FireResistance = 12,
    WaterBreathing = 13,
    Invisibility = 14,
    Blindness = 15,
    NightVision = 16,
    Hunger = 17,
    Weakness = 18,
    Poison = 19,
    Wither = 20,
    HealthBoost = 21,
    Absorption = 22,
    Saturation = 23,
    Glowing = 24,
    Levitation = 25,
    Luck = 26,
    Unluck = 27,
    SlowFalling = 28,
    ConduitPower = 29,
    DolphinsGrace = 30,
    BadOmen = 31,
    HeroOfTheVillage = 32,
    Darkness = 33,
}
