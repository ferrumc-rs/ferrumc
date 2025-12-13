//! Update Time packet for synchronizing world time with clients.
//!
//! This packet is sent periodically to all clients to synchronize the world time,
//! which controls the day/night cycle, sky rendering, and time-based game mechanics.
//!
//! # Time System
//!
//! Minecraft time is measured in ticks (20 ticks = 1 second, although the TPS is configurable in the config file):
//! - A full day/night cycle is 24,000 ticks (20 real-world minutes)
//! - Time of day values:
//!   - 0: Sunrise
//!   - 6,000: Noon
//!   - 12,000: Sunset
//!   - 18,000: Midnight
//!
//! # Protocol
//! - Packet ID: `set_time` (0x6A in protocol 773)
//! - State: Play
//! - Bound to: Client

use ferrumc_macros::{packet, NetEncode};
use typename::TypeName;

/// Update Time packet sent to clients to synchronize world time.
///
/// This packet contains both the total world age and the current time of day,
/// allowing clients to render the appropriate sky and lighting conditions.
#[derive(TypeName, NetEncode, Clone, Debug)]
#[packet(packet_id = "set_time", state = "play")]
pub struct UpdateTime {
    /// Total age of the world in ticks.
    ///
    /// This value always increases and is not affected by `/time set` commands.
    /// Used for things like random tick timing and scheduled block updates.
    pub world_age: i64,

    /// Current time of day in ticks (0-23,999).
    ///
    /// This determines the position of the sun/moon and sky color.
    /// Can be modified by `/time set` commands.
    pub time_of_day: i64,

    /// Whether the client should automatically advance time.
    ///
    /// When `true`, the client will increment `time_of_day` locally at its
    /// tick rate, reducing the need for frequent server updates. When `false`,
    /// the client will only use the server-provided time (useful for frozen time).
    pub time_of_day_increasing: bool,
}

impl UpdateTime {
    /// Creates a new `UpdateTime` packet.
    ///
    /// # Arguments
    ///
    /// * `world_age` - The total world age in ticks
    /// * `time_of_day` - The current time of day (0-23,999)
    /// * `time_of_day_increasing` - Whether the client should auto-advance time
    #[must_use]
    pub const fn new(world_age: i64, time_of_day: i64, time_of_day_increasing: bool) -> Self {
        Self {
            world_age,
            time_of_day,
            time_of_day_increasing,
        }
    }

    /// Creates an `UpdateTime` packet with auto-advancing time enabled.
    ///
    /// This is the typical configuration for normal gameplay where the
    /// day/night cycle progresses naturally.
    #[must_use]
    pub const fn normal(world_age: i64, time_of_day: i64) -> Self {
        Self::new(world_age, time_of_day, true)
    }

    /// Creates an `UpdateTime` packet with time frozen (not auto-advancing).
    ///
    /// Used when the game rule `doDaylightCycle` is disabled.
    #[must_use]
    pub const fn frozen(world_age: i64, time_of_day: i64) -> Self {
        Self::new(world_age, time_of_day, false)
    }
}

impl Default for UpdateTime {
    fn default() -> Self {
        // Default to sunrise (tick 0) with time advancing
        Self::normal(0, 0)
    }
}