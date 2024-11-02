//! # Statics module.
//!
//! Contains the static global configuration and its related functions.

use std::fs::File;
use std::io::Read;
use std::process::exit;
use crate::server_config::ServerConfig;
use lazy_static::lazy_static;
use ferrumc_general_purpose::paths::get_root_path;

/// The default server configuration that is stored in memory.
const DEFAULT_CONFIG: &str = include_str!("../../../../../.etc/example-config.toml");

lazy_static! {
    /// The server configuration that is stored in memory.
    static ref CONFIG: ServerConfig = create_config();
}
fn create_config() -> ServerConfig {
    let config_location = get_root_path().expect("Could not get root").join("config.toml");
    if config_location.exists() {
        let mut file = match File::open(config_location) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Could not open configuration file: {}", e);
                exit(1);
            }
        };
        let mut config_str = String::new();
        if let Err(e) = file.read_to_string(&mut config_str) {
            eprintln!("Could not read configuration file: {}",e );
            exit(1);
        } else {
            if config_str.is_empty() {
                eprintln!("Configuration file is empty.");
                exit(1);
            }
            match toml::from_str(&config_str) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Could not parse configuration file: {}", e);
                    exit(1);
                }
            }
        }
    } else {
        println!("Configuration file not found. Using default configuration.");
        match toml::from_str(DEFAULT_CONFIG) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Could not parse default configuration: {}", e);
                exit(1);
            }
        }
    }
}

pub fn get_global_config() -> &'static ServerConfig {
    &CONFIG
}



