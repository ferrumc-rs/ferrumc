use crate::errors::ConfigError;
use crate::statics::WHITELIST;
use ferrumc_general_purpose::paths::get_root_path;
use futures::future::join_all;
use regex::Regex;
use reqwest::Client;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::time::Instant;
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
        if seen.contains(line) {
            false
        } else {
            seen.insert(line.clone());
            true
        }
    });

    //validates uuids, hyphenated or not
    let uuid_regex = Regex::new(r"^(?:[0-9a-fA-F]{8}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{12}|[0-9a-fA-F]{32})$").unwrap();
    //validates a mojang accepted name
    let valid_name_regex = Regex::new(r"^[a-zA-Z0-9_]{3,16}$").unwrap();

    let mut names_to_convert: HashMap<String, usize> = HashMap::new(); //pure name entries
    let mut uuids_to_convert: HashMap<Uuid, usize> = HashMap::new(); //pure uuid entries
    let mut valid_lines = Vec::new(); //lines that dont need changes
    let mut invalid_lines = Vec::new(); //entry isnt a valid uuid OR name

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

        if uuid_regex.is_match(uuid_or_name) {
            if let Ok(uuid) = Uuid::parse_str(uuid_or_name) {
                if valid_name_regex.is_match(commented_name) {
                    return_uuids.push(uuid);
                    valid_lines.push(index);
                } else {
                    uuids_to_convert.insert(uuid, index);
                }
            }
        } else if valid_name_regex.is_match(uuid_or_name) {
            names_to_convert.insert(uuid_or_name.to_ascii_lowercase(), index);
        } else {
            invalid_lines.push((index, line.clone()));
        }
    }

    let name_query_input: Vec<String> = names_to_convert
        .keys()
        .map(|name| name.to_ascii_lowercase())
        .collect();
    let uuid_query_input: Vec<&Uuid> = uuids_to_convert.keys().collect();

    let start_net_request = Instant::now();
    let found_usernames = query_mojang_for_usernames(uuid_query_input).await;
    let found_uuids = query_mojang_for_uuid(name_query_input).await;
    let net_request_time = start_net_request.elapsed().as_millis();
    debug!(
        "Querying Mojang for usernames and uuids took {} ms",
        net_request_time
    );

    for profile in found_usernames {
        if let Some(index) = uuids_to_convert.remove(&Uuid::try_parse(&profile.id).unwrap()) {
            let uuid = Uuid::try_parse(&profile.id).unwrap();
            lines[index] = format!("{} # {}", uuid.hyphenated(), profile.name);
            valid_lines.push(index);
            return_uuids.push(uuid);
        }
    }

    for (name_lcase, profile) in found_uuids {
        if let Some(index) = names_to_convert.remove(&name_lcase) {
            let uuid = Uuid::try_parse(&profile.id).unwrap();
            lines[index] = format!("{} # {}", uuid.hyphenated(), profile.name);
            valid_lines.push(index);
            return_uuids.push(uuid);
        }
    }

    //these are the lines that were not matched to a uuid or name
    for (name, index) in names_to_convert {
        //line matched name but mojang returned no uuid
        lines[index] = format!("# Invalid UUID: {name}");
    }

    for (uuid, index) in uuids_to_convert {
        //line matched uuid but mojang returned no name
        //currently any uuids that dont match a real name get added here and I cant tell why rn
        lines[index] = format!("# Invalid Username: {uuid}");
    }

    for (index, line) in invalid_lines {
        //line didnt match uuid or name regex
        lines[index] = format!("# Invalid entry: {line}");
    }

    let mut updated_whitelist = File::create(&whitelist_location).map_err(|e| {
        error!("Could not write updated whitelist file: {e}");
        ConfigError::IOError(e)
    })?;

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
/// Queries mojang for the uuids of the given names, returned as a map of lowercase names to the full profile.
async fn query_mojang_for_uuid(names: Vec<String>) -> HashMap<String, MojangProfile> {
    debug!("Querying Mojang for UUIDs");

    if names.is_empty() {
        return HashMap::new();
    }

    let client = Client::new();

    let futures = names.chunks(10).map(|chunk| {
        let client = &client;
        async move {
            let response = client
                .post("https://api.mojang.com/profiles/minecraft")
                .json(&chunk)
                .send()
                .await;

            match response {
                Ok(response) if response.status().is_success() => {
                    match response.json::<Vec<MojangProfile>>().await {
                        Ok(profiles) => profiles,
                        Err(e) => {
                            error!("Failed to parse JSON response for names {:?}: {e}", chunk);
                            Vec::new()
                        }
                    }
                }
                Ok(response) => {
                    error!(
                        "Mojang API returned HTTP {} for names {:?}",
                        response.status(),
                        chunk
                    );
                    Vec::new()
                }
                Err(e) => {
                    error!("Failed to query Mojang for names {:?}: {e}", chunk);
                    Vec::new()
                }
            }
        }
    });

    let results = join_all(futures).await;

    let mut result_map = HashMap::new();
    for profiles in results {
        for profile in profiles {
            result_map.insert(profile.name.to_ascii_lowercase(), profile);
        }
    }

    result_map
}

async fn query_mojang_for_usernames(uuids: Vec<&Uuid>) -> Vec<MojangProfile> {
    if uuids.is_empty() {
        return Vec::new();
    }

    let client = Client::new();

    let futures = uuids.into_iter().map(|uuid| {
        let client = &client;
        async move {
            let uuid = uuid.as_simple();
            let response = client
                .get(format!(
                    "https://sessionserver.mojang.com/session/minecraft/profile/{uuid}"
                ))
                .send()
                .await;

            match response {
                Ok(response) if response.status().is_success() => {
                    match response.json::<MojangProfile>().await {
                        Ok(parsed_response) => Some(parsed_response),
                        Err(e) => {
                            error!("Failed to parse JSON response for UUID {}: {e}", uuid);
                            None
                        }
                    }
                }
                Ok(response) => {
                    error!(
                        "Mojang API returned HTTP {} for UUID {}",
                        response.status(),
                        uuid
                    );
                    None
                }
                Err(e) => {
                    error!("Failed to query Mojang for UUID {}: {e}", uuid);
                    None
                }
            }
        }
    });

    let results = join_all(futures).await;
    results.into_iter().flatten().collect()
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
