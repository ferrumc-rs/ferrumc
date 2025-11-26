use serde::{Deserialize, Serialize};

/// Defines the rules of the world (sky color, fog, coordinate scale).
/// This is the "Type".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DimensionTypeData {
    Overworld,
    Nether,
    End,
    Custom(u32), // For custom JSON dimensions
}

/// Defines a unique identifier for a world instance (e.g. "lobby").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorldName(pub String);
