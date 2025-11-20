use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Represents the data for a specific block state ID.
/// e.g. { "name": "minecraft:grass_block", "properties": { "snowy": "false" } }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct BlockData {
    pub name: String,
    #[serde(default)]
    pub properties: BTreeMap<String, String>,
    #[serde(default)]
    pub default: bool,
}
