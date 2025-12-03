use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataPackEntry {
    pub namespace: String,
    pub id: String,
    pub version: String,
}
