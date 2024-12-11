//! # The server configuration module.
//!
//! Contains the server configuration struct and its related functions.

use serde_derive::{Deserialize, Serialize};

/// The server configuration struct.
///
/// Fields:
/// - `host`: The IP/host that the server will bind to.
/// - `port`: The port that the server will bind to. (0-65535)
/// - `motd`: The message of the day that is displayed to clients. It will randomly select one from the list.
/// - `max_players`: The maximum number of players that can be connected to the server.
/// - `network_tick_rate`: How many network updates to process per second per user.
/// - `database` - [DatabaseConfig]: The configuration for the database.
/// - `world`: The name of the world that the server will load.
/// - `network_compression_threshold`: The threshold at which the server will compress network packets.
/// - `whitelist`: Whether the server whitelist is enabled or not.
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16, // 0-65535
    pub motd: Vec<String>,
    pub max_players: u32,
    pub network_tick_rate: u32,
    pub database: DatabaseConfig,
    pub world: String,
    pub network_compression_threshold: i32, // Can be negative
    pub whitelist: bool
}

/// The database configuration section from [ServerConfig].
///
/// Fields:
/// - `cache_size`: The cache size in KB.
/// - `compression` - Which compression algorithm to use. Options are `brotli`, `deflate`, `gzip`, `zlib`
///     and `zstd`
/// - `world_path`: The path to the world database.
/// - `compression_level`: The compression level to use. This is a number from 0-22. Not all compressors
///     support levels, so this will be a no-op for some compressors.
/// - `map_size`: The max size of the database's memory map. Basically you need this to be big enough
///    to hold everything before it starts writing to disk. This isn't memory use though, it's just
///    how much we can map into memory if needed, so you can set this to an insane number if you want,
///    but it won't actually use that much memory, it'll just show up as virtual memory use.
/// - `cache_ttl`: The time to live for cache entries in seconds.
/// - `cache_capacity`: How big the cache can be in kb.
#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub compression: String,
    pub db_path: String,
    pub compression_level: i32,
    pub map_size: u64,
    pub cache_ttl: u64,
    pub cache_capacity: u64,
}

/// The database compression enum for [DatabaseConfig].
///
/// Variants:
/// - `none`: No compression.
/// - `fast`: Fast compression.
/// - `best`: Best compression.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum DatabaseCompression {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "fast")]
    Fast,
    #[serde(rename = "best")]
    Best,
}
