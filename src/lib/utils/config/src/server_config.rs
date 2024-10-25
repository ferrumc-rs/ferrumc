//! # The server configuration module.
//!
//! Contains the server configuration struct and its related functions.

use crate::errors::ConfigError;
use crate::statics::{get_global_config, set_global_config};
use ferrumc_general_purpose::paths::get_root_path;
use serde_derive::{Deserialize, Serialize};
use tracing::{error, info, warn};

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
    pub log_packets:bool
}

/// The database configuration section from [ServerConfig].
///
/// Fields:
/// - `cache_size`: The cache size in KB.
/// - `compression` - [DatabaseCompression]: The compression algorithm to use.
#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub cache_size: u32,
    pub compression: DatabaseCompression,
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

impl ServerConfig {
    /// Load the configuration from a file.
    ///
    /// This returns a [ConfigError] if the file could not be read or the configuration could not be deserialized.
    /// If the configuration file does not exist, it will prompt the user to create a new one.
    ///
    /// Also sets the global configuration path to the path parameter (or default).
    ///
    /// Arguments:
    /// - `path`: Optional path to the configuration file. If not provided, it will default to "./config.toml".
    ///
    /// Example:
    /// ```rust
    /// # #![allow(unused_variables)]
    /// # fn main() {
    /// #   use ferrumc_config::server_config::ServerConfig;
    /// // Load the configuration from the default path.
    /// let config = ServerConfig::new(None).expect("Failed to read configuration file.");
    /// println!("{:?}", config);
    ///
    /// // Load the configuration from a custom path.
    /// let config = ServerConfig::new(Some("./custom_config.toml")).expect("Failed to read configuration file.");
    /// println!("{:?}", config);
    /// # }
    pub fn new(path: Option<&str>) -> Result<Self, ConfigError> {
        // Default path to "./config.toml" if None.
        let path = path.unwrap_or("./config.toml");

        // Load the configuration from the file.
        let config = Self::set_config(path, true)?;

        Ok(config)
        /*set_global_config(config)?;

        Ok(get_global_config())*/
    }

    /// Load the configuration from a file without prompting the user to create a new one.
    ///
    /// Exact same as [ServerConfig::new], but does not prompt the user to create a new configuration file.
    ///
    /// Safe for use in automated tests.
    // Allow dead code since this is only used in tests.
    #[allow(dead_code)]
    pub(crate) fn new_no_prompt(path: Option<&str>) -> Result<&'static Self, ConfigError> {
        // Default path to "./config.toml" if None.
        let path = path.unwrap_or("./config.toml");

        // Load the configuration from the file.
        let config = Self::set_config(path, false)?;

        set_global_config(config)?;

        Ok(get_global_config())
    }

    /// Logic to read the configuration file.
    ///
    /// Not meant to be called directly. Use [ServerConfig::new] instead.
    /// This was separated to allow for test cases.
    ///
    /// Arguments:
    /// - `path`: The path to the configuration file.
    /// - `prompt_user`: Whether to prompt the user to create a new configuration file if the current one is invalid.
    pub(crate) fn set_config(path: &str, prompt_user: bool) -> Result<ServerConfig, ConfigError> {
        let path = get_root_path()?.join(path);
        let config = std::fs::read_to_string(&path);
        let config: &str = match &config {
            Ok(config) => config,
            Err(e) => {
                // Check if we can prompt the user to create a new configuration file.
                if !prompt_user {
                    return Err(ConfigError::ConfigLoadError(
                        path.to_string_lossy().to_string(),
                    ));
                }
                // Config could not be read. Prompt the user to create a new one from ServerConfig::Default.
                warn!(
                    "Could not read configuration file \"{}\" : {}",
                    path.to_string_lossy().to_string(),
                    e
                );
                info!("Creating new config file!");

                // Create a new config file
                std::fs::write(&path, DEFAULT_CONFIG)?;

                DEFAULT_CONFIG
            }
        };

        let config: ServerConfig = match toml::from_str(config) {
            Ok(config) => config,
            Err(e) => {
                // Config could not be serialized. Prompt the user to create
                // a new one from ServerConfig::Default.
                error!(
                    "Could not read configuration file \"{}\" : {}",
                    path.to_string_lossy().to_string(),
                    e
                );
                error!("Would you like to create a new config file? Your old configuration will be saved as \"config.toml.bak\". (y/N): ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                // If the user enters "y", create a new configuration file.
                if input.trim().eq_ignore_ascii_case("y") {
                    // Backup the old configuration file.
                    std::fs::rename(&path, "config.toml.bak")?;

                    // Create new configuration file.
                    std::fs::write(&path, DEFAULT_CONFIG)?;
                    info!("Configuration file created.");
                } else {
                    // User did not enter "y". Return the error.
                    return Err(ConfigError::ConfigLoadError(
                        path.to_string_lossy().to_string(),
                    ));
                }
                // Deserialize the configuration file into a ServerConfig struct.
                toml::from_str(DEFAULT_CONFIG)?
            }
        };

        Ok(config)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 25565,
            motd: vec!["A supersonic FerrumC server.".to_string()],
            max_players: 20,
            network_tick_rate: 0,
            database: DatabaseConfig {
                cache_size: 1024,
                compression: DatabaseCompression::Fast,
            },
            world: "world".to_string(),
            network_compression_threshold: 256,
            log_packets:false
        }
    }
}

const DEFAULT_CONFIG: &str = r#"
# This is the host/ip that the server will bind to. (127.0.0.1 for local, and 0.0.0.0 for public)
host = "0.0.0.0"
# This is the port that the server will bind to. (0-65535), 25565 is the default Minecraft port.
port = 25565
motd = ["A supersonic FerrumC server."]
max_players = 20
network_tick_rate = 0

world = "world"
network_compression_threshold = 256
log_packets = false

[database]
cache_size = 1024
compression = "fast"
"#;
