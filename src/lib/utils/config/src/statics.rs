use crate::server_config::ServerConfig;
use ferrumc_general_purpose::paths::get_root_path;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use std::sync::Mutex;
use tracing::{error, info};

/// The default server configuration that is stored in memory.
pub(crate) const DEFAULT_CONFIG: &str = include_str!("../../../../../.etc/example-config.toml");

lazy_static! {
    /// The server configuration that is stored in memory.
    static ref CONFIG: ServerConfig = create_config();
    /// The whitelist of players, wrapped in a Mutex for thread-safe access
    /// assuming commands can come from any thread in future
    static ref WHITELIST: Mutex<Vec<Player>> = Mutex::new(create_whitelist());
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub uuid: u128,
    pub name: String,
}

fn create_whitelist() -> Vec<Player> {
    let whitelist_location = get_root_path().join("whitelist.json");

    if !whitelist_location.exists() {
        if let Err(e) = File::create(&whitelist_location).and_then(|mut file| file.write_all(b"[]"))
        {
            error!("Could not create initial white-list file: {e}");
            error!("White-list disabled");
            return Vec::new();
        }
        return Vec::new();
    }

    let mut file = match File::open(&whitelist_location) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open white-list file: {e}");
            error!("White-list disabled");
            return Vec::new();
        }
    };

    let mut whitelist_str = String::new();
    if let Err(e) = file.read_to_string(&mut whitelist_str) {
        error!("Could not read white-list file: {e}");
        error!("White-list disabled");
        return Vec::new();
    }

    if whitelist_str.is_empty() {
        return Vec::new();
    }

    serde_json::from_str::<Vec<Player>>(&whitelist_str).unwrap_or_else(|e| {
        error!("Could not parse white-list JSON: {e}");
        error!("White-list disabled");
        Vec::new()
    })
}

pub fn get_global_config() -> &'static ServerConfig {
    &CONFIG
}

pub fn get_whitelist() -> Vec<Player> {
    WHITELIST.lock().unwrap().clone()
}

/// Adds a player to the white-list.
/// Returns `true` if the player was added successfully, `false` otherwise.
pub fn add_player_to_whitelist(player: Player) -> Result<bool, String> {
    let mut whitelist = WHITELIST
        .lock()
        .map_err(|e| format!("Failed to acquire lock: {e}"))?;

    if whitelist.iter().any(|p| p.uuid == player.uuid) {
        return Ok(false);
    }

    whitelist.push(player);

    if let Err(e) = save_whitelist(&whitelist) {
        error!("Failed to save whitelist: {e}");
        return Err(format!("Failed to save whitelist: {e}"));
    }

    Ok(true)
}

/// Removes a player from the whitelist.
/// Returns `true` if the player was removed successfully, `false` otherwise.
pub fn remove_player_from_whitelist(player: Player) -> Result<bool, String> {
    let mut whitelist = WHITELIST
        .lock()
        .map_err(|e| format!("Failed to acquire lock: {e}"))?;

    let initial_len = whitelist.len();
    whitelist.retain(|p| p.uuid != player.uuid);

    if whitelist.len() == initial_len {
        return Ok(false);
    }

    if let Err(e) = save_whitelist(&whitelist) {
        error!("Failed to save whitelist: {e}");
        return Err(format!("Failed to save whitelist: {e}"));
    }

    Ok(true)
}

fn save_whitelist(whitelist: &[Player]) -> Result<(), String> {
    let whitelist_location = get_root_path().join("whitelist.json");
    let mut file = File::create(&whitelist_location).map_err(|e| {
        error!("Could not create white-list file for saving: {e}");
        format!("Could not create white-list file for saving: {e}")
    })?;

    serde_json::to_writer(&mut file, whitelist).map_err(|e| {
        error!("Could not create white-list file for saving: {e}");
        format!("Could not write white-list to file: {e}")
    })
}
