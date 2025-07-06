use crate::errors::ConfigError;
use crate::statics::WHITELIST;
use ferrumc_general_purpose::paths::get_root_path;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use tracing::error;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WhitelistEntry {
    pub uuid: String,
    pub name: String,
}

pub fn create_whitelist() {
    let whitelist_location = get_root_path().join("whitelist.json");
    if !whitelist_location.exists() {
        create_blank_whitelist_file();
    }

    let file = match File::open(&whitelist_location) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open whitelist file: {e}");
            return;
        }
    };

    let reader = BufReader::new(file);
    let whitelist: Vec<WhitelistEntry> = match serde_json::from_reader(reader) {
        Ok(whitelist) => whitelist,
        Err(e) => {
            error!("Could not parse whitelist file: {e}");
            return;
        }
    };

    for entry in whitelist {
        if let Ok(uuid) = Uuid::parse_str(&entry.uuid) {
            WHITELIST.insert(uuid.as_u128());
        }
    }
}

pub fn add_to_whitelist(uuid: Uuid, name: String) -> Result<(), ConfigError> {
    let mut whitelist = read_whitelist()?;
    let uuid_str = uuid.to_string();

    if !whitelist.iter().any(|entry| entry.uuid == uuid_str) {
        whitelist.push(WhitelistEntry {
            uuid: uuid_str,
            name,
        });
        write_whitelist(&whitelist)?;
        WHITELIST.insert(uuid.as_u128());
    }

    Ok(())
}

pub fn remove_from_whitelist(uuid: Uuid) -> Result<(), ConfigError> {
    let mut whitelist = read_whitelist()?;
    let uuid_str = uuid.to_string();

    if let Some(pos) = whitelist.iter().position(|entry| entry.uuid == uuid_str) {
        whitelist.remove(pos);
        write_whitelist(&whitelist)?;
        WHITELIST.remove(&uuid.as_u128());
    }

        Ok(())
}

pub fn reload_whitelist() -> Result<(), ConfigError> {
    let whitelist = read_whitelist()?;
    WHITELIST.clear();
    for entry in whitelist {
        if let Ok(uuid) = Uuid::parse_str(&entry.uuid) {
            WHITELIST.insert(uuid.as_u128());
        }
    }
    Ok(())
}

pub fn list_whitelist() -> Result<Vec<WhitelistEntry>, ConfigError> {
    read_whitelist()
}

fn read_whitelist() -> Result<Vec<WhitelistEntry>, ConfigError> {
    let whitelist_location = get_root_path().join("whitelist.json");
    let file = File::open(&whitelist_location).map_err(ConfigError::IOError)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(ConfigError::JsonError)
}

fn write_whitelist(whitelist: &[WhitelistEntry]) -> Result<(), ConfigError> {
    let whitelist_location = get_root_path().join("whitelist.json");
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&whitelist_location)
        .map_err(ConfigError::IOError)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, whitelist).map_err(ConfigError::JsonError)
}

pub fn create_blank_whitelist_file() {
    let whitelist_location = get_root_path().join("whitelist.json");
    if let Err(e) = File::create(&whitelist_location).and_then(|mut file| file.write_all(b"[]")) {
        error!("Failed to save whitelist: {e}");
    }
}