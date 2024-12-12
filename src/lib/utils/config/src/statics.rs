use crate::server_config::ServerConfig;
use dashmap::DashMap;
use ferrumc_general_purpose::paths::get_root_path;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use tracing::{error, info};
use uuid::Uuid;

/// The default server configuration that is stored in memory.
pub(crate) const DEFAULT_CONFIG: &str = include_str!("../../../../../.etc/example-config.toml");

lazy_static! {
    /// The server configuration that is stored in memory.
    static ref CONFIG: ServerConfig = create_config();
    /// The whitelist of player uuids.
    static ref WHITELIST: DashMap<u128, String> = create_whitelist();
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

fn create_whitelist() -> DashMap<u128, String> {
    let whitelist_location = get_root_path().join("whitelist.txt");
    if !whitelist_location.exists() {
        write_whitelist_to_file();
    }

    let mut file = match File::open(&whitelist_location) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open white-list file: {e}");
            return DashMap::new();
        }
    };

    let mut whitelist_str = String::new();
    if let Err(e) = file.read_to_string(&mut whitelist_str) {
        error!("Could not read white-list file: {e}");
        return DashMap::new();
    }

    if whitelist_str.is_empty() {
        return DashMap::new();
    }

    //read and split the file, its username:uuid format
    let whitelist: DashMap<u128, String> = DashMap::new();
    for (i, line) in whitelist_str.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut split = line.split(':');
        let name = match split.next() {
            Some(name) => name,
            None => {
                error!("Invalid line {} in whitelist (missing name): {line}", i + 1);
                continue;
            }
        };

        let uuid_str = match split.next() {
            Some(uuid_str) => uuid_str,
            None => {
                error!("Invalid line {} in whitelist (missing UUID): {line}", i + 1);
                continue;
            }
        };

        let u128_uuid = match Uuid::try_parse(uuid_str) {
            Ok(uuid) => uuid.as_u128(),
            Err(e) => {
                error!("Invalid uuid in whitelist on line {}: {line}: {e}", i + 1);
                continue;
            }
        };

        whitelist.insert(u128_uuid, name.to_string());
    }
    whitelist
}

pub fn write_whitelist_to_file() {
    let whitelist: &DashMap<u128, String> = get_whitelist();
    let whitelist_location = get_root_path().join("whitelist.txt");

    if let Err(e) = File::create(&whitelist_location).and_then(|mut file| {
        file.write_all(
            b"# This is the whitelist file.\n\
        # Each seperate line is a name and uuid seperated by :\n\
        # Eg. DefinitelyARealUser:00000000-0000-0000-0000-000000000000\n",
        )
        .and_then(|_| {
            whitelist.iter().try_for_each(|entry| {
                let uuid_str = Uuid::from_u128(*entry.key()).hyphenated().to_string();
                let line = format!("{}:{}\n", entry.value(), uuid_str);
                file.write_all(line.as_bytes())
            })
        })
    }) {
        error!("Failed to save whitelist: {e}");
    }
}

pub fn get_global_config() -> &'static ServerConfig {
    &CONFIG
}

pub fn get_whitelist() -> &'static DashMap<u128, String> {
    &WHITELIST
}
