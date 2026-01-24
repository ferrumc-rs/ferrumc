use std::collections::HashMap;
use serde::Deserialize;
use crate::BlockState;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum SingleOrMultiple {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize)]
pub struct BuildConfig {
    pub name_overrides: HashMap<String, String>,
    pub block_overrides: HashMap<String, String>,
    pub property_types: HashMap<String, SingleOrMultiple>,
}

pub const BUILD_CONFIG: &str = include_str!("../../../build_config.toml");
pub const BLOCK_STATES: &str = include_str!("../../../../../../assets/data/blockstates.json");

pub fn get_build_config() -> BuildConfig {
    toml::from_str(BUILD_CONFIG).unwrap()
}

pub fn get_block_states() -> HashMap<u32, BlockState> {
    let out: HashMap<String, BlockState> = serde_json::from_str(BLOCK_STATES).unwrap();

    out
        .into_iter()
        .map(|(k, v)| (k.parse::<u32>().unwrap(), v))
        .collect()
}

impl<'a> IntoIterator for &'a SingleOrMultiple {
    type Item = &'a String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            SingleOrMultiple::Single(v) => vec![v].into_iter(),
            SingleOrMultiple::Multiple(vals) => vals.iter().collect::<Vec<_>>().into_iter(),
        }
    }
}