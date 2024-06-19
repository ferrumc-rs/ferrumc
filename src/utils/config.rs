use std::collections::HashMap;
use std::io::ErrorKind::NotFound;
use std::io::Write;

use config::{Config, ConfigError};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::constants::{DEFAULT_CONFIG_FILE, DEFAULT_SERVER_HOST, DEFAULT_SERVER_PORT};
use crate::error::Error;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    endpoint: String,
}

impl ServerConfig {
    pub fn new() -> Result<Self> {
        let settings = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()
            .or_else(|err| {
                if is_not_found(&err) {
                    info!("Config file wasn't found, creating a new one.");
                    create_config_file()?;
                    return Config::builder()
                        .add_source(config::File::with_name("config"))
                        .build()
                        .map_err(Error::from);
                }
                Err(Error::from(err))
            })?;

        let de_settings: ServerConfig = settings.try_deserialize().or_else(|e| {
            error!("There was an error deserializing the config.");
            if let Some(field) = missing_field(&e) {
                info!("Missing field {} in config file", field);
                info!("Would you like to create a new config file? (y/n)");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                return if input.trim() == "y" {
                    info!("Creating new config file...");
                    create_config_file()?;
                    Config::builder()
                        .add_source(config::File::with_name("config"))
                        .build()
                        .map_err(Error::from)
                        .and_then(|settings| settings.try_deserialize().map_err(Error::from))
                } else {
                    error!("Aborting...");
                    Err(Error::from(e))
                };
            }
            Err(Error::from(e))
        })?;

        Ok(de_settings)
    }
}

fn is_not_found(err: &ConfigError) -> bool {
    let ConfigError::Foreign(foreign_error) = err else {
        error!("Error wasn't foreign: {:?}", err);
        return false;
    };

    let Some(io_error) = foreign_error.downcast_ref::<std::io::Error>() else{
        error!("Foreign error wasn't an IO error: {:?}", foreign_error);
        return false;
    };

    io_error.kind() == NotFound
}

fn missing_field(err: &ConfigError) -> Option<String> {
    if let ConfigError::Message(message) = err {
        if message.contains("missing field") {
            return message.split('"').nth(1).map(String::from);
        }
    }
    None
}

fn create_config_file() -> Result<()> {
    let path = std::path::Path::new(DEFAULT_CONFIG_FILE);
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    let mut file = std::fs::File::create(path)?;
    let contents = toml::to_string(&ServerConfig::default())?;
    file.write_all(contents.as_bytes())?;

    info!("Path: {}", path.display());

    Ok(())
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: DEFAULT_SERVER_HOST.to_string(),
            port: DEFAULT_SERVER_PORT, // Minecraft default port
        }
    }
}
