//! # Statics module.
//!
//! Contains the static global configuration and its related functions.

use crate::server_config::ServerConfig;
use ferrumc_general_purpose::paths::get_root_path;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use tracing::{error, info};

/// The default server configuration that is stored in memory.
pub(crate) const DEFAULT_CONFIG: &str = include_str!("../../../../../.etc/example-config.toml");

lazy_static! {
    /// The server configuration that is stored in memory.
    static ref CONFIG: ServerConfig = create_config();
}
fn create_config() -> ServerConfig {
    let config_location = get_root_path().join("config.toml");
    if config_location.exists() {
        let mut file = match File::open(config_location) {
            Ok(file) => file,
            Err(e) => {
                error!("Could not open configuration file: {}", e);
                exit(1);
            }
        };
        let mut config_str = String::new();
        if let Err(e) = file.read_to_string(&mut config_str) {
            error!("Could not read configuration file: {}", e);
            exit(1);
        } else {
            if config_str.is_empty() {
                error!("Configuration file is empty.");
                exit(1);
            }
            match toml::from_str(&config_str) {
                Ok(config) => config,
                Err(e) => {
                    error!("Could not parse configuration file: {}", e);
                    exit(1);
                }
            }
        }
    } else {
        info!(
            "Configuration file not found. Making a default configuration at {}",
            config_location.display()
        );
        let default_config = DEFAULT_CONFIG;
        // write to the config file
        let mut file = match File::create(config_location) {
            Ok(file) => file,
            Err(e) => {
                error!("Could not create configuration file: {}", e);
                exit(1);
            }
        };

        if let Err(e) = file.write_all(default_config.as_bytes()) {
            error!("Could not write default configuration to file: {}", e);
            exit(1);
        }

        match toml::from_str(DEFAULT_CONFIG) {
            Ok(config) => config,
            Err(e) => {
                error!("Could not parse default configuration: {}", e);
                exit(1);
            }
        }
    }
}

pub fn get_global_config() -> &'static ServerConfig {
    &CONFIG
}
