use bevy_ecs::resource::Resource;
use tracing::warn;

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
///
///     tps.tps(Duration::from_secs(1));
///     tps.tick_duration(0.50);
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
#[derive(Resource)]
pub struct ServerPerformance {
    pub tps: TPSMonitor,
    pub memory: MemoryUsage,
}

impl ServerPerformance {
    pub fn new(tps: u32) -> Self {
        if !sysinfo::IS_SUPPORTED_SYSTEM {
            warn!("System does not support 'sysinfo', disabling server performance statisics.");
        }

        Self {
            tps: TPSMonitor::new(tps),
            memory: MemoryUsage::default(),
        }
    }
}
