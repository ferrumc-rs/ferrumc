use crate::errors::ConfigError;
use crate::server_config::ServerConfig;
use dashmap::DashSet;
use ferrumc_general_purpose::paths::get_root_path;
use lazy_static::lazy_static;
use regex::Regex;
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
    static ref WHITELIST: DashSet<u128> = create_whitelist();
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

fn create_whitelist() -> DashSet<u128> {
    let whitelist_location = get_root_path().join("whitelist.txt");
    if !whitelist_location.exists() {
        create_blank_whitelist_file();
    }

    let mut file = match File::open(&whitelist_location) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open whitelist file: {e}");
            return DashSet::new();
        }
    };

    let mut whitelist_str = String::new();
    if let Err(e) = file.read_to_string(&mut whitelist_str) {
        error!("Could not read whitelist file: {e}");
        return DashSet::new();
    }

    if whitelist_str.is_empty() {
        return DashSet::new();
    }

    let uuids: Vec<Uuid> = match convert_whitelist_file() {
        Ok(uuids) => uuids,
        Err(e) => return DashSet::new(),
    };
    let whitelist: DashSet<u128> = uuids.into_iter().map(|uuid| uuid.as_u128()).collect();

    whitelist
}

///returns a list of all the uuids found or generated in the whitelist file
fn convert_whitelist_file() -> Result<Vec<Uuid>, ConfigError> {
    let whitelist_location = get_root_path().join("whitelist.txt");
    if !whitelist_location.exists() {
        create_blank_whitelist_file();
        return Ok(Vec::new());
    }

    let mut file = match File::open(&whitelist_location) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open whitelist file for conversion: {e}");
            return Err(ConfigError::IOError(e));
        }
    };

    let mut whitelist_str = String::new();
    if let Err(e) = file.read_to_string(&mut whitelist_str) {
        error!("Could not read whitelist file for conversion: {e}");
        return Err(ConfigError::IOError(e));
    }

    if whitelist_str.is_empty() {
        create_blank_whitelist_file();
        return Ok(Vec::new());
    }

    let uuid_regex = Regex::new(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
    )
    .unwrap();

    let mut lines: Vec<String> = Vec::new();
    let mut names_to_convert: Vec<(usize, String)> = Vec::new();
    let mut uuids_to_convert: Vec<(usize, String)> = Vec::new();

    for (i, line) in whitelist_str.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            lines.push(line.to_string());
            continue;
        }

        let (pre_comment, comment) = match trimmed.split_once('#') {
            Some((p, c)) => (p.trim(), c.trim()),
            None => (trimmed, ""),
        };

        // If it doesn't match UUID regex we need to convert it
        if !uuid_regex.is_match(pre_comment) {
            names_to_convert.push((i, pre_comment.to_string()));
        } else if comment.len() < 3 {
            //an else as pure usernames won't have a comment
            uuids_to_convert.push((i, pre_comment.to_string()))
        }
        //query local db for uuid to last known name, or mojang api if not yet possible
        //query mojang api for name to uuid
        //overwrite the lines required with their new values
    }
    Ok(Vec::new())
}

pub fn create_blank_whitelist_file() {
    let whitelist_location = get_root_path().join("whitelist.txt");

    if let Err(e) = File::create(&whitelist_location).and_then(|mut file| {
        file.write_all(
            b"# This is the whitelist file.\n\
        # Each separate line contains a UUID or username.\n\
        # Eg. 00000000-0000-0000-0000-000000000000\n\
        # Eg. DefinitelyARealUser\n\
        # The server will attempt to convert usernames to a uuid\n",
        )
    }) {
        error!("Failed to save whitelist: {e}");
    }
}

pub fn get_global_config() -> &'static ServerConfig {
    &CONFIG
}

pub fn get_whitelist() -> &'static DashSet<u128> {
    &WHITELIST
}
