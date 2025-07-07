use crate::errors::ConfigError;
use crate::statics::WHITELIST;
use ferrumc_general_purpose::paths::get_root_path;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WhitelistEntry {
    pub uuid: Option<String>,
    pub name: Option<String>,
}

const WHITELIST_FILE_HEADER_COMMENTS: &str = r#"# This file contains the whitelist for the server.
# Only players on this list will be able to join.
#
# Each entry must have a UUID and/or a name.
# If you provide only one, the server will fetch the other from Mojang's API.
# You can find a player's UUID from their username using a tool like https://namemc.com/
#
# Example with both:
# - uuid: "069a79f4-44e9-4726-a5be-fca90e38aaf5"
#   name: "Notch"
#
# Example with only username (UUID will be fetched):
# - name: "jeb_"
#
# Example with only UUID (username will be fetched):
# - uuid: "853c80ef-3c37-49fd-aa49-938b674adae6"
"#;

pub fn create_whitelist() {
    let whitelist_location = get_root_path().join("whitelist.yml");
    if !whitelist_location.exists() {
        create_blank_whitelist_file();
    }

    let whitelist = match read_whitelist() {
        Ok(whitelist) => whitelist,
        Err(e) => {
            error!("Could not read whitelist file: {}", e);
            return;
        }
    };
    process_whitelist_entries(whitelist);
}

fn process_whitelist_entries(entries: Vec<WhitelistEntry>) {
    for entry in entries {
        let (uuid, name) = match (entry.uuid, entry.name) {
            (Some(uuid), Some(name)) => (uuid, name),
            (Some(uuid), None) => {
                info!("Fetching name for {}", &uuid);
                match fetch_name_from_uuid(&uuid) {
                    Ok(name) => (uuid, name),
                    Err(e) => {
                        error!("Could not fetch name for {}: {}", uuid, e);
                        continue;
                    }
                }
            }
            (None, Some(name)) => {
                info!("Fetching uuid for {}", &name);
                match fetch_uuid_from_name(&name) {
                    Ok(uuid) => (uuid, name),
                    Err(e) => {
                        error!("Could not fetch uuid for {}: {}", name, e);
                        continue;
                    }
                }
            }
            (None, None) => {
                warn!("Skipping empty whitelist entry");
                continue;
            }
        };

        if let Ok(uuid) = Uuid::parse_str(&uuid) {
            WHITELIST.insert(uuid.as_u128(), name);
        } else {
            error!("Invalid UUID format in whitelist: {}", uuid);
        }
    }
}

fn fetch_uuid_from_name(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut response = ureq::get(&format!(
        "https://api.mojang.com/users/profiles/minecraft/{}",
        name
    ))
    .call()?;

    let resp: serde_json::Value = response.body_mut().read_json()?;

    let uuid_without_dashes = resp["id"]
        .as_str()
        .ok_or("Invalid response from Mojang API")?;
    let uuid_with_dashes = format!(
        "{}-{}-{}-{}-{}",
        &uuid_without_dashes[0..8],
        &uuid_without_dashes[8..12],
        &uuid_without_dashes[12..16],
        &uuid_without_dashes[16..20],
        &uuid_without_dashes[20..32]
    );
    Ok(uuid_with_dashes)
}

fn fetch_name_from_uuid(uuid: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut response = ureq::get(&format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid
    ))
    .call()?;

    let resp: serde_json::Value = response.body_mut().read_json()?;

    let name = resp["name"]
        .as_str()
        .ok_or("Invalid response from Mojang API")?;
    Ok(name.to_string())
}

pub fn add_to_whitelist(uuid: Uuid, name: String) {
    WHITELIST.insert(uuid.as_u128(), name);
}

pub fn remove_from_whitelist(uuid: Uuid) {
    WHITELIST.remove(&uuid.as_u128());
}

pub fn reload_whitelist(force: bool) -> Result<(), ConfigError> {
    if !force {
        let in_memory_whitelist = list_whitelist()?;
        let on_disk_whitelist = read_whitelist()?;

        let in_memory_yaml = serde_yaml_ng::to_string(&in_memory_whitelist).unwrap_or_default();
        let on_disk_yaml = serde_yaml_ng::to_string(&on_disk_whitelist).unwrap_or_default();

        if in_memory_yaml != on_disk_yaml {
            return Err(ConfigError::Custom(
                "Unsaved changes exist in memory. Use --force to discard these changes and reload from disk.".to_string(),
            ));
        }
    }

    let whitelist = read_whitelist()?;
    WHITELIST.clear();
    process_whitelist_entries(whitelist);
    Ok(())
}

pub fn list_whitelist() -> Result<Vec<WhitelistEntry>, ConfigError> {
    let mut whitelist = Vec::new();
    for item in WHITELIST.iter() {
        whitelist.push(WhitelistEntry {
            uuid: Some(Uuid::from_u128(*item.key()).to_string()),
            name: Some(item.value().clone()),
        });
    }
    Ok(whitelist)
}

fn read_whitelist() -> Result<Vec<WhitelistEntry>, ConfigError> {
    let whitelist_location = get_root_path().join("whitelist.yml");
    let file = File::open(&whitelist_location).map_err(ConfigError::IOError)?;
    let reader = BufReader::new(file);
    match serde_yaml_ng::from_reader(reader) {
        Ok(list) => Ok(list),
        Err(e) => Err(ConfigError::YamlError(e)),
    }
}

pub fn flush_whitelist_to_disk() -> Result<(), ConfigError> {
    let whitelist = list_whitelist()?;

    let whitelist_location = get_root_path().join("whitelist.yml");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // This will clear existing content, including old comments
        .create(true)
        .open(&whitelist_location)
        .map_err(ConfigError::IOError)?;
    let mut writer = BufWriter::new(&mut file);

    // 1. Write the constant header comments FIRST
    writer
        .write_all(WHITELIST_FILE_HEADER_COMMENTS.as_bytes())
        .map_err(ConfigError::IOError)?;
    writer.write_all(b"\n").map_err(ConfigError::IOError)?; // Add an extra newline for separation

    // 2. Then, serialize the actual whitelist data and write it
    let serialized_data = serde_yaml_ng::to_string(&whitelist).map_err(ConfigError::YamlError)?;
    writer
        .write_all(serialized_data.as_bytes())
        .map_err(ConfigError::IOError)?;

    writer.flush().map_err(ConfigError::IOError)?;
    Ok(())
}

pub fn create_blank_whitelist_file() {
    let whitelist_location = get_root_path().join("whitelist.yml");
    let initial_content = format!("{}{}\n", WHITELIST_FILE_HEADER_COMMENTS, "[]");

    if let Err(e) = File::create(&whitelist_location)
        .and_then(|mut file| file.write_all(initial_content.as_bytes()))
    {
        error!(
            "Failed to save blank whitelist file to {}: {}",
            whitelist_location.display(),
            e
        );
    }
}
