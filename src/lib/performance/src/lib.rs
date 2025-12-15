use bevy_ecs::resource::Resource;

use crate::{memory::MemoryUsage, tps::TPSMonitor};

pub mod memory;
pub mod tick;
pub mod tps;

pub const WINDOW_SECONDS: usize = 60;

/// Core ECS resource for all server performance metrics.
///
/// This resource is updated once per tick by the main scheduler
/// loop and can be queried by commands, debug tools, or plugins.
/// As shown below.
///
/// For grabbing Memory Usage it has to be mutable.
///
/// ```rs
/// fn test(performance: Res<ServerPerformance>) {
///     let tps = &performance.tps;
/// }
/// ```
///
/// ```rs
/// fn test(performance: ResMut<ServerPerformance>) {
///     let (current, peak) = performance.memory.get_memory(MemoryUnit::Megabytes);
/// }
/// ```
///
/// Currently tracks:
/// - Tick durations
/// - Rolling TPS (1s / 5s / 15s windows)
/// - Memory Usage (Current / Peak)
///
/// Intended to expand in the future to include:
/// - Lag spike detection
/// - Scheduler overrun statistics (will be moved here)
/// - Sampler-based profiling (maybe i doubt it to be honest)
#[derive(Resource)]
pub struct ServerPerformance {
    pub tps: TPSMonitor,
    pub memory: MemoryUsage,
}

impl ServerPerformance {
    pub fn new(tps: u32) -> Self {
        Self {
            tps: TPSMonitor::new(tps),
            memory: MemoryUsage::default(),
        }
    }
}
