use ferrumc_config::server_config::get_global_config;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Duration;
use sysinfo::{Pid, ProcessesToUpdate, System};
use tokio::sync::broadcast::Sender;
use tokio::time::interval;
use tracing::{debug, error};

static DISK_SIZE: OnceLock<u64> = OnceLock::new();

#[derive(Clone, Debug, Serialize)]
pub struct ServerMetric {
    /// CPU usage percentage (0.0 - 100.0)
    pub cpu_usage: f32,
    /// Memory usage in bytes
    pub ram_usage: u64,
    /// Total RAM in bytes
    pub total_ram: u64,
    /// Uptime in seconds
    pub uptime: u64,
    /// Used storage in bytes
    pub storage_used: u64,
    /// Total storage in bytes
    pub storage_total: u64,
}

/// Events sent from the server to the dashboard (websocket)
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum DashboardEvent {
    Metric(ServerMetric),
    #[allow(unused)]
    Log(String),
}

pub async fn start_telemetry_loop(tx: Sender<DashboardEvent>) {
    debug!("Starting server telemetry");

    // Initialize the system monitor
    let mut sys = System::new_all();
    let pid = Pid::from(std::process::id() as usize);

    // Tick every second; should be configurable later
    const TICK_INTERVAL_SECS: u64 = 1;
    let mut ticker = interval(Duration::from_secs(TICK_INTERVAL_SECS));

    loop {
        ticker.tick().await;

        // Refresh system info for our PID
        sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

        let Some(process) = sys.process(pid) else {
            error!("Failed to get process info for dashboard telemetry");
            continue;
        };

        let config = get_global_config();
        let mut world_path = PathBuf::from(&config.database.db_path);
        let storage_used = if world_path.exists() {
            world_path = world_path.canonicalize().unwrap_or(world_path);
            dir_size::get_size_in_bytes(&world_path).unwrap_or(0)
        } else {
            0
        };

        let storage_total = DISK_SIZE.get_or_init(|| match get_total_disk_for_path(&world_path) {
            Some((available, kind)) => available,
            None => 0,
        });

        let metric = ServerMetric {
            cpu_usage: process.cpu_usage(),
            ram_usage: process.memory(),
            total_ram: sys.total_memory(),
            uptime: process.run_time(),
            storage_used,
            storage_total: *storage_total,
        };

        // Broadcast to all connected web clients
        // We ignore the error (it fails if no browsers are open, which is fine)
        let _ = tx.send(DashboardEvent::Metric(metric));
    }
}

fn get_total_disk_for_path(path: &Path) -> Option<(u64, String)> {
    let disks = sysinfo::Disks::new_with_refreshed_list();
    for disk in disks.list() {
        for segment in path.ancestors() {
            let segment_str = dunce::simplified(segment);
            let mount_point_str = dunce::simplified(disk.mount_point());
            if mount_point_str == segment_str {
                return Some((disk.available_space(), disk.kind().to_string()));
            }
        }
    }
    None
}
