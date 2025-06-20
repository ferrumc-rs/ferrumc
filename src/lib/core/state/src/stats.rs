use atomic_float::AtomicF32;
use serde::Serialize;
use std::sync::atomic::{AtomicU32, AtomicU64};
use parking_lot::Mutex;
use sysinfo::System;

#[derive(Default, Serialize)]
pub struct Stats {
    #[serde(skip_serializing)]
    pub system: Mutex<System>,
    pub player_count: AtomicU32,
    pub uptime: AtomicU64, // in seconds
    pub memory_usage: AtomicU64, // in bytes
    pub cpu_usage: AtomicF32, // in percentage
    pub cores: AtomicU32, // number of CPU cores
}