pub mod errors;

use std::sync::OnceLock;
use serde_derive::{Deserialize, Serialize};
use crate::errors::ConfigError;

/// The server configuration that is stored in memory.
static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

/// Helper function to get the server configuration.
///
/// **WARNING:** Configuration [ServerConfig::new] must be called before calling this function.
/// Otherwise, it will return an error.
///
/// Example of proper usage:
/// ```rust
/// # #![allow(unused_variables)]
/// # fn main() {
/// #   use ferrumc_config::{get_global_config, ServerConfig};
/// // Get config from default path.
/// ServerConfig::new(None).expect("Failed to read configuration file.");
///
/// // Do other stuff...
///
/// // Get the global configuration.
/// let config = get_global_config().expect("Failed to get global configuration.");
/// println!("{:?}", config);
/// # }
/// ```
///
/// Example of improper usage:
/// ```rust
/// # #![allow(unused_variables)]
/// # fn main() {
/// #   use ferrumc_config::get_global_config;
/// // Get the global configuration without setting the configuration first.
/// let config = get_global_config().expect("Failed to get global configuration."); // Error.
/// println!("{:?}", config);
/// # }
/// ```
pub fn get_global_config() -> Result<&'static ServerConfig, ConfigError> {
    CONFIG.get().ok_or(ConfigError::ConfigLoadError)
}

/// Sets the global configuration.
///
/// This function should be called once before calling `get_global_config()`.
///
/// Arguments:
/// - `config`: The configuration to be set globally.
fn set_global_config(config: ServerConfig) -> Result<(), ConfigError> {
    CONFIG.set(config).map_err(|_| ConfigError::ConfigSetError)
}

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
    pub network_compression_threshold: u32,
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
    /// #   use ferrumc_config::ServerConfig;
    /// // Load the configuration from the default path.
    /// let config = ServerConfig::new(None).expect("Failed to read configuration file.");
    /// println!("{:?}", config);
    ///
    /// // Load the configuration from a custom path.
    /// let config = ServerConfig::new(Some("./custom_config.toml")).expect("Failed to read configuration file.");
    /// println!("{:?}", config);
    /// # }
    pub fn new(path: Option<&str>) -> Result<&'static Self, ConfigError> {
        // Default path to "./config.toml" if None.
        let path = path.unwrap_or("./config.toml");

        // Load the configuration from the file.
        let config = Self::set_config(path, true)?;

        set_global_config(config)?;

        get_global_config()
    }

    /// Load the configuration from a file without prompting the user to create a new one.
    ///
    /// Exact same as [ServerConfig::new], but does not prompt the user to create a new configuration file.
    ///
    /// Safe for use in automated tests.
    #[allow(dead_code)]
    fn new_no_prompt(path: Option<&str>) -> Result<&'static Self, ConfigError> {
        // Default path to "./config.toml" if None.
        let path = path.unwrap_or("./config.toml");

        // Load the configuration from the file.
        let config = Self::set_config(path, false)?;

        set_global_config(config)?;

        get_global_config()
    }

    /// Logic to read the configuration file.
    ///
    /// Not meant to be called directly. Use [ServerConfig::new] instead.
    /// This was separated to allow for test cases.
    ///
    /// Arguments:
    /// - `path`: The path to the configuration file.
    /// - `prompt_user`: Whether to prompt the user to create a new configuration file if the current one is invalid.
    fn set_config(path: &str, prompt_user: bool) -> Result<ServerConfig, ConfigError> {
        let config = std::fs::read_to_string(path)?;

        // Deserialize the configuration file into a ServerConfig struct.
        match toml::from_str(&config) {
            Ok(config) => Ok(config),
            Err(e) => {
                // Check if we can prompt the user to create a new configuration file.
                if !prompt_user {
                    return Err(ConfigError::TomlDeError(e));
                }

                // Config could not be serialized. Prompt the user to create
                // a new one from ServerConfig::Default.
                println!("Could not read configuration file: {}", e);
                println!("Would you like to create a new one? Your old configuration will be saved as config.toml.bak. (y/N) ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                // If the user enters "y", create a new configuration file.
                if input.trim().to_ascii_lowercase() == "y" {
                    // Backup the old configuration file.
                    std::fs::rename("./config.toml", "./config.toml.bak")?;

                    // Create new configuration file.
                    let new_config = ServerConfig::default();
                    let toml = toml::to_string(&new_config)?;
                    std::fs::write("./config.toml", toml)?;
                    println!("Configuration file created.");
                    Ok(new_config)
                } else {
                    // User did not enter "y". Return the error.
                    Err(ConfigError::TomlDeError(e))
                }
            }
        }
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;
    use std::io::Write;

    /// A struct to hold the test configuration file paths.
    /// When drop is called, it will remove the files.
    /// Prevents files from being left behind after tests.
    struct TestFile {
        config_file: File,
        path: &'static str,
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            remove_file(self.path).expect("Unable to remove test config file.");
        }
    }

    /// A helper function to generate a sample configuration string in TOML format.
    fn sample_config_toml() -> String {
        r#"
        host = "127.0.0.1"
        port = 25565
        motd = ["hi", "bye"]
        max_players = 100
        network_tick_rate = 20
        world = "default_world"
        network_compression_threshold = 512

        [database]
        cache_size = 4096
        compression = "fast"
        "#
            .to_string()
    }

    /// A helper function to generate an invalid configuration string in TOML format.
    fn invalid_config_toml() -> String {
        r#"
        host = "
        port = 25565
        motd = ["hi", "bye"]
        max_players = 100
        network_tick_rate = 20
        "#
            .to_string()
    }

    /// Test a sample configuration file in TOML format.
    #[test]
    fn test_sample_config_toml() {
        // Write the sample config to a temporary file
        let config_str = sample_config_toml();
        let config_file_path = "./test_server_config.toml";

        // Write the configuration to the file
        // TestFile implements Drop, so the file will be removed after the test.
        let mut file = TestFile {
            config_file: File::create(config_file_path).expect("Unable to create test config file."),
            path: config_file_path,
        };
        file.config_file.write_all(config_str.as_bytes()).expect("Unable to write test config data.");
        
        // Load the configuration from the file
        let server_config = ServerConfig::new_no_prompt(Some(config_file_path)).expect("Failed to read configuration file.");
        
        // Test the get_global_config function
        let global_config = get_global_config().expect("Failed to get global configuration.");
        assert_eq!(global_config.host, "127.0.0.1");
        assert_eq!(global_config.port, 25565);
        assert_eq!(global_config.motd, vec!["hi", "bye"]);
        assert_eq!(global_config.max_players, 100);
        assert_eq!(global_config.network_tick_rate, 20);
        assert_eq!(global_config.world, "default_world");
        assert_eq!(global_config.network_compression_threshold, 512);
        assert_eq!(global_config.database.cache_size, 4096);
        assert!(matches!(global_config.database.compression, DatabaseCompression::Fast));

        // Test the values in the ServerConfig struct
        assert_eq!(server_config.host, "127.0.0.1");
        assert_eq!(server_config.port, 25565);
        assert_eq!(server_config.motd, vec!["hi", "bye"]);
        assert_eq!(server_config.max_players, 100);
        assert_eq!(server_config.network_tick_rate, 20);
        assert_eq!(server_config.world, "default_world");
        assert_eq!(server_config.network_compression_threshold, 512);
        assert_eq!(server_config.database.cache_size, 4096);
        assert!(matches!(server_config.database.compression, DatabaseCompression::Fast));
    }

    /// Test an invalid configuration file in TOML format.
    #[test]
    fn test_invalid_config_toml() {
        // Write the invalid config to a temporary file
        let config_str = invalid_config_toml();
        let config_file_path = "./test_invalid_config.toml";

        // Write the configuration to the file
        // TestFile implements Drop, so the file will be removed after the test.
        let mut file = TestFile {
            config_file: File::create(config_file_path).expect("Unable to create test config file."),
            path: config_file_path,
        };
        file.config_file.write_all(config_str.as_bytes()).expect("Unable to write test config data.");

        // Load the configuration from the file
        let server_config = ServerConfig::new_no_prompt(Some(config_file_path));

        // Test that the configuration could not be loaded
        assert!(server_config.is_err());

        println!("{}", server_config.err().unwrap());
    }
}
