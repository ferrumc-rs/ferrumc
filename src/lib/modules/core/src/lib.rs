use bevy_app::{App, Plugin, PreUpdate, Update};
use bevy_ecs::prelude::*;

mod systems; // Contains new_connections, connection_killer, keep_alive

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Handle new TCP connections (High priority)
            .add_systems(PreUpdate, systems::new_connections::accept_new_connections)
            // Handle disconnects & Caching (Cleanup)
            .add_systems(PreUpdate, systems::connection_killer::connection_killer)
            // Maintenance
            .add_systems(Update, systems::keep_alive_system::keep_alive_system)
            .add_systems(Update, systems::player_count_update::player_count_updater);
    }
}
