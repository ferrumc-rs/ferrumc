use crate::errors::ConfigError;
use crate::statics::WHITELIST;
use ferrumc_general_purpose::paths::get_root_path;
use regex::Regex;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use tracing::{debug, error};
use uuid::Uuid;

pub async fn create_whitelist() {
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

    let uuids: Vec<Uuid> = match convert_whitelist_file().await {
        Ok(uuids) => uuids,
        Err(_e) => return,
    };
    uuids.into_iter().for_each(|uuid| {
        WHITELIST.insert(uuid.as_u128());
    });
}

///converts usernames within the whitelist file to uuid, returns a list of all resulting uuids within the file
async fn convert_whitelist_file() -> Result<Vec<Uuid>, ConfigError> {
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
    let mut lines = whitelist_str
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    if lines.is_empty() {
        create_blank_whitelist_file();
        return Ok(Vec::new());
    }

    let uuid_regex = Regex::new(
        r"^(?:[0-9a-fA-F]{8}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{12}|[0-9a-fA-F]{32})$",
    ) //validates both hyphenated and non hyphenated uuids
    .unwrap();
    let valid_name_regex = Regex::new(r"^[a-zA-Z0-9_]{3,16}$").unwrap();

    let mut names_to_convert: HashMap<String, usize> = HashMap::new();
    let mut uuids_to_convert: HashMap<String, usize> = HashMap::new();
    let mut valid_lines: Vec<usize> = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        debug!("Processing line {}: {}", index, trimmed);
        if trimmed.is_empty() || trimmed.starts_with('#') {
            valid_lines.push(index);
            continue;
        }

        let (pre_comment, comment) = match trimmed.split_once('#') {
            Some((p, c)) => (p.trim(), c.trim()),
            None => (trimmed, ""),
        };

        if !uuid_regex.is_match(pre_comment) {
            // If it's not a UUID, check if it's a valid username
            if valid_name_regex.is_match(pre_comment) {
                names_to_convert.insert(pre_comment.to_string().to_ascii_lowercase(), index);
                debug!("Found username to convert: {}", pre_comment);
            }
        } else {
            // It's a UUID, check its length to ensure it is valid
            if !valid_name_regex.is_match(comment) {
                valid_lines.push(index); // temporary until I can be bothered to do the call for name and validate it below
                debug!("Found UUID to generate name for: {}", pre_comment);
            } else {
                valid_lines.push(index);
            }
        }
    }

    let name_query_input = names_to_convert
        .keys()
        .map(|name| name.as_str())
        .collect::<Vec<&str>>();

    // let uuid_query_input = uuids_to_convert
    //     .keys()
    //     .map(|name| name.as_str())
    //     .collect::<Vec<&str>>();
    //todo, generate names for valid uuids that dont have one attached

    let found_uuids = query_mojang_for_uuid(&name_query_input).await;

    debug!("Found UUIDs: {:?}", found_uuids);
    for (name_lcase, profile) in found_uuids {
        if let Some(index) = names_to_convert.remove(&name_lcase) {
            let uuid = Uuid::parse_str(&profile.id).unwrap();
            lines[index] = format!("{} # {}", uuid.hyphenated(), profile.name);
            valid_lines.push(index);
            debug!(
                "Updated line {} to UUID and name: {} # {}",
                index, profile.id, profile.name
            );
        }
    }

    let invalid_lines: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(index, _)| !valid_lines.contains(index))
        .map(|(index, _)| index)
        .collect();
    debug!("Invalid line indices: {:?}", invalid_lines);

    // comment out invalid lines, can attach a reason if needed
    for index in invalid_lines {
        lines[index] = format!("#Invalid Entry: {}", lines[index]);
    }

    let mut updated_whitelist = File::create(&whitelist_location).map_err(|e| {
        error!("Could not write updated whitelist file: {e}");
        ConfigError::IOError(e)
    })?;

    for line in lines {
        if let Err(e) = writeln!(updated_whitelist, "{}", line) {
            error!("Failed to write line to file: {e}");
            return Err(ConfigError::IOError(e));
        }
    }

    Ok(Vec::new())
}

#[derive(Deserialize, Debug)]
struct MojangProfile {
    id: String,
    name: String,
}
/// Queries mojang for the uuids of the given names, returned as a map of lowercase names to the full profile.
async fn query_mojang_for_uuid(names_to_convert: &[&str]) -> HashMap<String, MojangProfile> {
    debug!("Querying Mojang for UUIDs");

    if names_to_convert.is_empty() {
        return HashMap::new();
    }

    let client = reqwest::Client::new();
    let mut result_map = HashMap::new();

    for chunk in names_to_convert.chunks(10) {
        debug!("Querying batch of names: {:?}", chunk);

        let response = client
            .post("https://api.mojang.com/profiles/minecraft")
            .json(&chunk)
            .send()
            .await;

        let profiles: Vec<MojangProfile> = match response {
            Ok(response) => match response.json().await {
                Ok(json) => json,
                Err(e) => {
                    debug!("Mojang returned no profiles for names: {chunk:?}");
                    continue;
                }
            },
            Err(e) => {
                error!("Failed to parse response from Mojang: {e}");
                continue;
            }
        };

        debug!("Parsed response from Mojang: {} profiles", profiles.len());

        for profile in profiles {
            debug!("Found UUID for {}: {}", profile.name, profile.id);
            result_map.insert(profile.name.to_ascii_lowercase(), profile);
        }
    }
    result_map
}

pub fn add_to_whitelist(uuid: Uuid) {
    WHITELIST.insert(uuid.as_u128());
}

pub fn remove_from_whitelist(uuid: Uuid) {
    WHITELIST.remove(&uuid.as_u128());
}

pub fn create_blank_whitelist_file() {
    let whitelist_location = get_root_path().join("whitelist.txt");

    if let Err(e) = File::create(&whitelist_location).and_then(|mut file| {
        file.write_all(
            b"# This is the whitelist file.\n\
        # Each separate line contains a UUID or username, Eg.\n\
        # 00000000-0000-0000-0000-000000000000\n\
        #         Or\n\
        # DefinitelyARealUserName\n",
        )
    }) {
        error!("Failed to save whitelist: {e}");
    }
}
