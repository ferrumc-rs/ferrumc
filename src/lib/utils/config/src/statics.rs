use crate::server_config::ServerConfig;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_general_purpose::paths::get_root_path;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use std::sync::Mutex;
use tracing::{debug, error, info};

/// The default server configuration that is stored in memory.
pub(crate) const DEFAULT_CONFIG: &str = include_str!("../../../../../.etc/example-config.toml");

lazy_static! {
    /// The server configuration that is stored in memory.
    static ref CONFIG: ServerConfig = create_config();
    /// The whitelist of players, wrapped in a Mutex for thread-safe access
    /// assuming commands can come from any thread in future
    static ref WHITELIST: Mutex<Vec<PlayerIdentity>> = Mutex::new(create_whitelist()); //should this be an rwlock instead? does it even need to be in statics?
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

fn create_whitelist() -> Vec<PlayerIdentity> {
    let whitelist_location = get_root_path().join("whitelist.json");
    if !whitelist_location.exists() {
        if let Err(e) = File::create(&whitelist_location).and_then(|mut file| file.write_all(b"[]"))
        {
            error!("Could not create initial white-list file: {e}");
            return Vec::new();
        }
        return Vec::new();
    }

    let mut file = match File::open(&whitelist_location) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open white-list file: {e}");
            return Vec::new();
        }
    };

    let mut whitelist_str = String::new();
    if let Err(e) = file.read_to_string(&mut whitelist_str) {
        error!("Could not read white-list file: {e}");
        return Vec::new();
    }

    if whitelist_str.is_empty() {
        return Vec::new();
    }
    serde_json::from_str::<Vec<PlayerIdentity>>(&whitelist_str).unwrap_or_else(|e| {
        error!("Could not parse white-list JSON: {e}");
        Vec::new()
    })
}

pub fn get_global_config() -> &'static ServerConfig {
    &CONFIG
}

pub fn get_whitelist() -> Vec<PlayerIdentity> {
    WHITELIST.lock().unwrap().clone()
}
