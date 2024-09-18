pub mod errors;

use serde_derive::{Deserialize, Serialize};
use toml::de::Error;
use crate::errors::ConfigError;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16, // 0-65535
    pub motd: Vec<String>,
    pub max_players: u32,
    pub database: DatabaseConfig,
    pub world: String,
    pub network_compression_threshold: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub cache_size: u32,
    pub compression: String,
}

impl ServerConfig {
    /// Load the configuration from a file.
    pub fn new() -> Result<Self, ConfigError> {
        // Check file ./config.toml (relative to the current working directory)
        // In the future, this could accept an argument to specify the file path.
        let config = std::fs::read_to_string("./config.toml")?;

        // Deserialize the configuration file into a ServerConfig struct.
        match toml::from_str(&config) {
            Ok(config) => Ok(config),
            Err(e) => {
                println!("Could not read configuration file: {}", e);
                println!("Would you like to create a new one? (y/n): ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if input.trim() == "y" {
                    let new_config = ServerConfig::default();
                    let toml = toml::to_string(&new_config)?;
                    std::fs::write("./config.toml", toml)?;
                    println!("Configuration file created.");
                    Ok(new_config)
                } else {
                    Err(ConfigError::TomlError(Error::from(e)))
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
            motd: vec!["A Minecraft Server".to_string()],
            max_players: 20,
            database: DatabaseConfig {
                cache_size: 1024,
                compression: "fast".to_string(),
            },
            world: "world".to_string(),
            network_compression_threshold: 256,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
