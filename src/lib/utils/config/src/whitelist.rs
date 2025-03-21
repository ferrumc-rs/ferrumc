use crate::errors::ConfigError;
use crate::statics::WHITELIST;
use ferrumc_general_purpose::paths::get_root_path;
use rayon::prelude::*;
use reqwest::Client;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use tracing::error;
use uuid::Uuid;

pub fn create_whitelist() {
    let whitelist_location = get_root_path().join("whitelist.txt");
    if !whitelist_location.exists() {
        create_blank_whitelist_file();
    }

    let mut file = match File::open(&whitelist_location) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open whitelist file: {e}");
            return;
        }
    };

    let mut whitelist_str = String::new();
    if let Err(e) = file.read_to_string(&mut whitelist_str) {
        error!("Could not read whitelist file: {e}");
        return;
    }

    if whitelist_str.is_empty() {
        return;
    }

    let uuids: Vec<Uuid> = match convert_whitelist_file() {
        Ok(uuids) => uuids,
        Err(_e) => return,
    };
    uuids.into_iter().for_each(|uuid| {
        WHITELIST.insert(uuid.as_u128());
    });
}

///converts usernames within the whitelist file to uuid, returns a list of all resulting uuids within the file
fn convert_whitelist_file() -> Result<Vec<Uuid>, ConfigError> {
    let whitelist_location = get_root_path().join("whitelist.txt");
    let mut return_uuids: Vec<Uuid> = Vec::new();

    if !whitelist_location.exists() {
        create_blank_whitelist_file();
        return Ok(return_uuids);
    }

    let mut file = File::open(&whitelist_location).map_err(|e| {
        error!("Could not open whitelist file: {e}");
        ConfigError::IOError(e)
    })?;

    let mut whitelist_str = String::new();
    file.read_to_string(&mut whitelist_str).map_err(|e| {
        error!("Could not read whitelist file: {e}");
        ConfigError::IOError(e)
    })?;

    let mut lines = whitelist_str
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>();
    if lines.is_empty() {
        create_blank_whitelist_file();
        return Ok(return_uuids);
    }
    //here we iterate over the lines removing duplicates while keeping the current order
    let mut seen = std::collections::HashSet::new();

    lines.retain(|line| {
        if line.is_empty() || seen.contains(line) {
            false
        } else {
            seen.insert(line.clone());
            true
        }
    });

    let mut uuids_to_convert: HashMap<Uuid, usize> = HashMap::new(); //pure uuid entries
    let mut valid_lines = Vec::new(); //lines that dont need changes
    let mut invalid_lines = Vec::new(); //entry isn't a valid uuid

    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            valid_lines.push(index);
            continue;
        }

        let (uuid_or_name, commented_name) = line
            .split_once('#')
            .map_or((line.as_str(), ""), |(pre_hash, post_hash)| {
                (pre_hash.trim(), post_hash.trim())
            });

        if let Ok(uuid) = Uuid::try_parse(uuid_or_name) {
            if !commented_name.is_empty() {
                return_uuids.push(uuid);
                valid_lines.push(index);
            } else {
                uuids_to_convert.insert(uuid, index);
            }
        } else {
            invalid_lines.push((index, line.clone()));
        }
    }

    let uuid_query_input: Vec<&Uuid> = uuids_to_convert.keys().collect();
    let found_usernames = query_mojang_for_usernames(uuid_query_input);

    for profile in found_usernames {
        if let Some(index) = uuids_to_convert.remove(&Uuid::try_parse(&profile.id).unwrap()) {
            let uuid = Uuid::try_parse(&profile.id).unwrap();
            lines[index] = format!("{} # {}", uuid.hyphenated(), profile.name);
            valid_lines.push(index);
            return_uuids.push(uuid);
        }
    }

    for (uuid, index) in uuids_to_convert {
        //line matched uuid regex but mojang returned no name
        lines[index] = format!("# UUID Doesnt Match a Real User: {uuid}");
    }

    for (index, line) in invalid_lines {
        //line didn't match a valid uuid format
        lines[index] = format!("# Invalid UUID: {line}");
    }

    let mut updated_whitelist = File::create(&whitelist_location).map_err(|e| {
        error!("Could not write updated whitelist file: {e}");
        ConfigError::IOError(e)
    })?;

    //remove duplicate lines again
    let mut seen = std::collections::HashSet::new();
    lines.retain(|line| {
        if seen.contains(line) {
            false
        } else {
            seen.insert(line.clone());
            true
        }
    });

    for line in lines {
        writeln!(updated_whitelist, "{}", line).map_err(|e| {
            error!("Failed to write line: {e}");
            ConfigError::IOError(e)
        })?;
    }
    Ok(return_uuids)
}

#[derive(Deserialize, Debug)]
struct MojangProfile {
    id: String,
    name: String,
}

fn query_mojang_for_usernames(uuids: Vec<&Uuid>) -> Vec<MojangProfile> {
    if uuids.is_empty() {
        return Vec::new();
    }

    uuids
        .into_iter()
        .par_bridge()
        .map(|uuid| {
            let uuid = uuid.as_simple();
            let response = ureq::get(format!(
                "https://sessionserver.mojang.com/session/minecraft/profile/{uuid}"
            ))
            .call();

            match response {
                Ok(mut response) => Some(response.body_mut().read_json()),
                _ => None,
            }
        })
        .flatten()
        .filter(|profile| profile.is_ok())
        .map(|profile| profile.unwrap())
        .collect()
}

pub fn add_to_whitelist(uuid: Uuid) -> bool {
    WHITELIST.insert(uuid.as_u128())
}

pub fn remove_from_whitelist(uuid: Uuid) -> bool {
    WHITELIST.remove(&uuid.as_u128()).is_some()
}

pub fn create_blank_whitelist_file() {
    let whitelist_location = get_root_path().join("whitelist.txt");

    if let Err(e) = File::create(&whitelist_location).and_then(|mut file| {
        file.write_all(
            b"# This is the whitelist file.\n\
        # Each separate line contains a UUID Eg.\n\
        # 00000000-0000-0000-0000-000000000000\n\
        # 11111111-1111-1111-1111-111111111111\n",
        )
    }) {
        error!("Failed to save whitelist: {e}");
    }
}
