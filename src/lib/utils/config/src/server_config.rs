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
}

/// The database configuration section from [ServerConfig].
///
/// Fields:
/// - `cache_size`: The cache size in KB.
/// - `compression` - Which compression algorithm to use. Options are `brotli`, `deflate`, `gzip`, `zlib` 
///     and `zstd`
/// - `backend` - Which database backend to use. Options are `redb`, `rocksdb`, `sled`, `surrealkv`.
/// - `world_path`: The path to the world database.
/// - `import_path`: The path to the world to import. This should point to the folder that contains
///     directories such as `region`, `poi`, `playerdata`, etc. Usually found at %APPDATA%/.minecraft/saves.
/// - `compression_level`: The compression level to use. This is a number from 0-22. Not all compressors
///     support levels, so this will be a no-op for some compressors.
#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub cache_size: u32,
    pub compression: String,
    pub backend: String,
    pub db_path: String,
    pub import_path: String,
    pub compression_level: i32,
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