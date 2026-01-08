use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Default and maximum values for player health, food, and saturation.
pub mod health_defaults {
    /// Maximum/default health value (20 = 10 hearts)
    pub const MAX_HEALTH: f32 = 20.0;
    /// Maximum/default food level
    pub const MAX_FOOD: i32 = 20;
    /// Default saturation value
    pub const DEFAULT_SATURATION: f32 = 5.0;
    /// Health value representing death
    pub const DEAD_HEALTH: f32 = 0.0;
}

/// Sent by the server to update the player's health, food, and saturation.
#[derive(NetEncode)]
#[packet(packet_id = "set_health", state = "play")]
pub struct SetHealth {
    /// Current health (0.0 to max_health, clamped by client)
    pub health: f32,
    /// Food level (0 to 20)
    pub food: VarInt,
    /// Food saturation (0.0 to food level)
    pub saturation: f32,
}

impl SetHealth {
    pub fn new(health: f32, food: i32, saturation: f32) -> Self {
        Self {
            health,
            food: VarInt::new(food),
            saturation,
        }
    }

    /// Create a packet for full health
    pub fn full() -> Self {
        Self::new(
            health_defaults::MAX_HEALTH,
            health_defaults::MAX_FOOD,
            health_defaults::DEFAULT_SATURATION,
        )
    }

    /// Create a packet for death (0 health)
    pub fn dead() -> Self {
        Self::new(
            health_defaults::DEAD_HEALTH,
            health_defaults::MAX_FOOD,
            health_defaults::DEFAULT_SATURATION,
        )
    }
}
