use ferrumc_config::server_config::get_global_config;
use serde::Serialize;
use sysinfo::System;

/// Hardware/system information
#[derive(Clone, Debug, Serialize)]
pub struct SystemData {
    /// CPU model name (e.g., "AMD Ryzen 9 5900X 12-Core Processor")
    pub cpu_model: String,
    /// Number of physical CPU cores
    pub cpu_cores: usize,
    /// Number of logical threads (including hyperthreading)
    pub cpu_threads: usize,
}

/// Server configuration data
#[derive(Clone, Debug, Serialize)]
pub struct ConfigData {
    /// Maximum number of players allowed on the server
    pub max_players: u32,
}

/// One-time handshake data sent when a WebSocket client connects.
/// Contains static information that doesn't change during server runtime.
#[derive(Clone, Debug, Serialize)]
pub struct Handshake {
    pub system: SystemData,
    pub config: ConfigData,
}

impl Handshake {
    /// Gathers handshake data. Should be called once at startup.
    pub fn gather() -> Self {
        let sys = System::new_all();

        let cpu_model = sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let cpu_cores = System::physical_core_count().unwrap_or(1);
        let cpu_threads = sys.cpus().len();

        let system = SystemData {
            cpu_model,
            cpu_cores,
            cpu_threads,
        };

        let config = ConfigData {
            max_players: get_global_config().max_players,
        };

        Self { system, config }
    }
}
