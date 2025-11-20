use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct PlayerDimension {
    pub dimension_id: i32, // TODO: update this to reflect the actual dimension system
}
