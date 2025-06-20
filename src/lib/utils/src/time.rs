use std::time::{Duration, UNIX_EPOCH};

pub fn unix_timestamp() -> Duration {
    UNIX_EPOCH.elapsed().expect("Failed to get elapsed time since UNIX_EPOCH")
}