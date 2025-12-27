use serde::Serialize;
use std::time::Duration;
use sysinfo::{Pid, ProcessesToUpdate, System};
use tokio::sync::broadcast::Sender;
use tokio::time::interval;
use tracing::{debug, error};

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

        let metric = ServerMetric {
            cpu_usage: process.cpu_usage(),
            ram_usage: process.memory(),
            total_ram: sys.total_memory(),
            uptime: process.run_time(),
        };

        // Broadcast to all connected web clients
        // We ignore the error (it fails if no browsers are open, which is fine)
        let _ = tx.send(DashboardEvent::Metric(metric));
    }
}
