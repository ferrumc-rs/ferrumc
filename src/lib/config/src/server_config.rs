//! # The server configuration module.
//!
//! Contains the server configuration struct and its related functions.

use ferrumc_general_purpose::paths::get_root_path;
use figment::providers::Format;
use once_cell::sync::OnceCell;
use serde_derive::{Deserialize, Serialize};

static STATIC_CONFIG: OnceCell<ServerConfig> = OnceCell::new();
pub(crate) const DEFAULT_CONFIG: &str =
    include_str!("../../../../assets/data/configs/main-config.toml");
pub fn get_global_config() -> &'static ServerConfig {
    STATIC_CONFIG.get_or_init(create_config)
}

/// Sets the global server configuration.
/// You really only want to use this for unit tests, otherwise just use `get_global_config()`
/// to set the config with the default values or the values from the config file.
pub fn set_global_config(config: ServerConfig) {
    if STATIC_CONFIG.set(config).is_err() {
        eprintln!("Failed to set global server configuration, it has already been initialized.");
    }
}

/// The server configuration struct.
///
/// Fields:
/// - `host`: The IP/host that the server will bind to.
/// - `port`: The port that the server will bind to. (0-65535)
/// - `motd`: The message of the day that is displayed to clients. It will randomly select one from the list.
/// - `max_players`: The maximum number of players that can be connected to the server.
/// - `tps`: The ticks per second that the server will run at.
/// - `database` - [DatabaseConfig]: The configuration for the database.
/// - `world`: The name of the world that the server will load.
/// - `network_compression_threshold`: The threshold at which the server will compress network packets.
/// - `whitelist`: Whether the server whitelist is enabled or not.
/// - `chunk_render_distance`: The render distance of the chunks. This is the number of chunks that will be
///   loaded around the player.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16, // 0-65535
    pub motd: Vec<String>,
    pub max_players: u32,
    pub tps: u32,
    pub database: DatabaseConfig,
    pub world: String,
    pub network_compression_threshold: i32, // Can be negative
    pub verify_decompressed_packets: bool,
    pub encryption_enabled: bool,
    pub online_mode: bool,
    pub whitelist: bool,
    pub chunk_render_distance: u32,
    pub default_gamemode: String,
    pub dashboard: DashboardConfig,
    pub performance: PerformanceConfig,
    #[serde(default)]
    pub fluids: FluidConfig,
}

/// The database configuration section from [ServerConfig].
///
/// Fields:
/// - `db_path`: The path to the database. This is relative to the server root path.
/// - `verify_chunk_data`: Whether to verify chunk data when loading it from the database.
/// - `map_size`: The max size of the database's memory map. Basically you need this to be big enough
///   to hold everything before it starts writing to disk. This isn't memory use though, it's just
///   how much we can map into memory if needed, so you can set this to an insane number if you want,
///   but it won't actually use that much memory, it'll just show up as virtual memory use.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DatabaseConfig {
    pub db_path: String,
    pub verify_chunk_data: bool,
    pub map_size: u64,
}

/// The dashboard configuration section from [ServerConfig].
///
/// Fields:
/// - `port`: The port that the dashboard will bind to. (0-65535)
/// - `secret`: The secret key for accessing the dashboard.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DashboardConfig {
    pub port: u16,
    pub secret: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PerformanceConfig {
    pub chunks_per_tick_min: u32,
    pub chunks_per_tick: i32,
}

/// Selects which fluid-spreading kernel the server uses.
///
/// `simplified` is a cheap approximation (uniform spread, no hole steering); `vanilla` is the
/// Minecraft-faithful bounded slope search that steers fluid toward the nearest hole at a higher
/// CPU cost. The actual kernels live in `ferrumc-world`; this enum is the config-level selector so
/// it can be validated at load time without the config crate depending on the world crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FluidAlgorithm {
    /// Cheap approximation: per-block feeder re-derivation, uniform horizontal spread.
    Simplified,
    /// Vanilla-faithful bounded slope search (`getSlopeDistance`): fluid steers toward holes.
    #[default]
    Vanilla,
}

/// The fluid simulation configuration section from [ServerConfig].
#[derive(Debug, Deserialize, Serialize)]
pub struct FluidConfig {
    /// Which spreading algorithm to use. Defaults to `vanilla`.
    #[serde(default)]
    pub algorithm: FluidAlgorithm,
    /// Whether generated "hanging" fluids are settled while the chunk is generated, on the chunk
    /// worker thread, so it arrives already flowed and needs no fluid simulation on the game-tick
    /// thread. This is the primary, off-tick settle path and resolves all flow contained within a
    /// chunk. Cross-chunk seams are left to [`settle_on_load`].
    #[serde(default = "default_true")]
    pub settle_on_generate: bool,
    /// Maximum block changes the generation-time settle ([`settle_on_generate`]) makes per chunk
    /// before stopping, bounding worker-thread cost. `0` means unbounded.
    #[serde(default = "default_max_settle_changes")]
    pub max_settle_changes: u32,
    /// Whether generated "hanging" fluids are also settled, on the game-tick thread, the first time a
    /// chunk loads near a player. With [`settle_on_generate`] on, this only mops up cross-chunk seams,
    /// so it is cheap; turn it off to keep all fluid work off the tick thread (chunk-interior fluids
    /// are still settled at generation time).
    #[serde(default = "default_true")]
    pub settle_on_load: bool,
    /// Maximum number of fluid ticks processed in a single game tick. A large cascade is spread
    /// across several ticks (settling slightly slower) rather than freezing one tick. `0` means
    /// unbounded.
    #[serde(default = "default_max_fluid_ticks_per_tick")]
    pub max_ticks_per_tick: u32,
}

fn default_true() -> bool {
    true
}

fn default_max_settle_changes() -> u32 {
    65_536
}

fn default_max_fluid_ticks_per_tick() -> u32 {
    2048
}

impl Default for FluidConfig {
    fn default() -> Self {
        Self {
            algorithm: FluidAlgorithm::default(),
            settle_on_generate: default_true(),
            max_settle_changes: default_max_settle_changes(),
            settle_on_load: default_true(),
            max_ticks_per_tick: default_max_fluid_ticks_per_tick(),
        }
    }
}

fn create_config() -> ServerConfig {
    let config_location = get_root_path().join("configs");
    let main_config_file = config_location.join("config.toml");
    match figment::Figment::new()
        // Load the default configuration
        .merge(figment::providers::Toml::string(DEFAULT_CONFIG))
        // Then override it with the main config file
        .merge(figment::providers::Toml::file(main_config_file))
        .extract()
    {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load server configuration: {e}");
            std::process::exit(1);
        }
    }
}
