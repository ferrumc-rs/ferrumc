use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_ecs::prelude::*;
use ferrumc_config::server_config::get_global_config;
use ferrumc_logging::init_logging;
use std::time::Duration;

// Plugins
use ferrumc_net::NetPlugin;
use ferrumc_plugin_core::CorePlugin;
use ferrumc_plugin_replication::ReplicationPlugin;
use ferrumc_storage::StoragePlugin;
// todo: import other plugins ...

fn main() {
    // 1. Imperative Setup
    let config = get_global_config();
    init_logging(config.log_level.into());

    info!("Starting FerrumC...");

    // 2. Build App
    App::new()
        // A. The Runner (Headless Loop)
        // This replaces the custom `game_loop.rs` logic.
        // It runs the schedule as fast as possible (or capped).
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 20.0,
        )))
        // B. Infrastructure Plugins (Layer 4)                                                                
        .add_plugins(StoragePlugin::new(config.database.clone()))
        .add_plugins(NetPlugin::new(config.network.clone()))
        // C. Game Logic Plugins (Layer 5)
        .add_plugins((
            CorePlugin, // Connection handling
            ReplicationPlugin, // Sending chunks/entities
                        // MiningPlugin,
                        // MovementPlugin,
                        // ChatPlugin,
        ))
        // D. Run
        .run();
}
