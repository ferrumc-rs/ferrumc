//! Respawn packet.
//!
//! Sent by the server to respawn a player. Changes the player's dimension,
//! resets their position, and optionally keeps their metadata.

use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Respawn packet sent to clients when they respawn or change dimensions.
///
/// This is similar to the Login packet but used for respawning.
#[derive(NetEncode)]
#[packet(packet_id = "respawn", state = "play")]
pub struct RespawnPacket<'a> {
    /// The dimension type ID
    pub dimension_type: VarInt,
    /// The dimension identifier (e.g., "minecraft:overworld")
    pub dimension_name: &'a str,
    /// Hashed seed for client-side biome noise
    pub seed_hash: i64,
    /// The player's gamemode (0=survival, 1=creative, 2=adventure, 3=spectator)
    pub gamemode: u8,
    /// The previous gamemode (-1 if none)
    pub previous_gamemode: i8,
    /// Whether the world is a debug world
    pub is_debug: bool,
    /// Whether the world is a superflat world
    pub is_flat: bool,
    /// Copy metadata (bit flags for what to keep):
    /// - 0x01: Keep attributes
    /// - 0x02: Keep metadata
    pub data_kept: u8,
    /// Whether there is a death location to send
    pub has_death_location: bool,
    /// Name of the dimension where the player died (if has_death_location)
    pub death_dimension_name: Option<&'a str>,
    /// Position where the player died (if has_death_location)
    pub death_location: Option<u64>, // Packed block position
    /// Portal cooldown in ticks
    pub portal_cooldown: VarInt,
    /// Sea level of the world (affects fog)
    pub sea_level: VarInt,
}

impl<'a> RespawnPacket<'a> {
    /// Create a respawn packet for a simple respawn in the overworld.
    ///
    /// This is the most common case: player died and respawns at spawn point.
    pub fn same_dimension(gamemode: u8) -> Self {
        Self {
            dimension_type: VarInt::new(0),
            dimension_name: "minecraft:overworld",
            seed_hash: 0,
            gamemode,
            previous_gamemode: -1,
            is_debug: false,
            is_flat: false,
            data_kept: 0, // Don't keep any data on death respawn
            has_death_location: false,
            death_dimension_name: None,
            death_location: None,
            portal_cooldown: VarInt::new(0),
            sea_level: VarInt::new(63),
        }
    }

    /// Create a respawn packet that keeps player data (for dimension change).
    pub fn change_dimension(dimension_name: &'a str, gamemode: u8) -> Self {
        Self {
            dimension_type: VarInt::new(0), // TODO: proper dimension type lookup
            dimension_name,
            seed_hash: 0,
            gamemode,
            previous_gamemode: gamemode as i8,
            is_debug: false,
            is_flat: false,
            data_kept: 0x03, // Keep attributes and metadata
            has_death_location: false,
            death_dimension_name: None,
            death_location: None,
            portal_cooldown: VarInt::new(0),
            sea_level: VarInt::new(63),
        }
    }
}
