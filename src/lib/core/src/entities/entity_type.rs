use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)] // Use u32 for easy casting to protocol IDs if needed
pub enum EntityType {
    Player = 0,
    Zombie,
    Skeleton,
    Creeper,
    Item,

}