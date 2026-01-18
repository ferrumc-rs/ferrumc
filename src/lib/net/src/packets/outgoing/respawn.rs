//! Respawn packet.
//!
//! Sent by the server to respawn a player. Changes the player's dimension,
//! resets their position, and optionally keeps their metadata.

use ferrumc_components::player::gamemode::GameMode;
use ferrumc_macros::{packet, NetEncode};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Bit flags for what player data to keep on respawn/dimension change.
pub mod data_kept_flags {
    /// Keep entity attributes (e.g., max health, speed modifiers)
    pub const KEEP_ATTRIBUTES: u8 = 0x01;
    /// Keep entity metadata (e.g., custom name, invisibility)
    pub const KEEP_METADATA: u8 = 0x02;
    /// Keep both attributes and metadata
    pub const KEEP_ALL: u8 = KEEP_ATTRIBUTES | KEEP_METADATA;
    /// Don't keep any data (full reset)
    pub const KEEP_NONE: u8 = 0x00;
}

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
    /// The player's gamemode
    pub gamemode: GameMode,
    /// The previous gamemode (-1 if none)
    pub previous_gamemode: i8,
    /// Whether the world is a debug world
    pub is_debug: bool,
    /// Whether the world is a superflat world
    pub is_flat: bool,
    /// Bit flags for what player data to keep (see [`data_kept_flags`])
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
    pub fn same_dimension(gamemode: GameMode) -> Self {
        Self {
            dimension_type: VarInt::new(0),
            dimension_name: "minecraft:overworld",
            seed_hash: 0,
            gamemode,
            previous_gamemode: -1,
            is_debug: false,
            is_flat: false,
            data_kept: data_kept_flags::KEEP_NONE,
            has_death_location: false,
            death_dimension_name: None,
            death_location: None,
            portal_cooldown: VarInt::new(0),
            sea_level: VarInt::new(63),
        }
    }

    /// Create a respawn packet that keeps player data (for dimension change).
    pub fn change_dimension(dimension_name: &'a str, gamemode: GameMode) -> Self {
        Self {
            dimension_type: VarInt::new(0), // TODO: proper dimension type lookup
            dimension_name,
            seed_hash: 0,
            gamemode,
            previous_gamemode: gamemode as i8,
            is_debug: false,
            is_flat: false,
            data_kept: data_kept_flags::KEEP_ALL,
            has_death_location: false,
            death_dimension_name: None,
            death_location: None,
            portal_cooldown: VarInt::new(0),
            sea_level: VarInt::new(63),
        }
    }
}
