use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

mod handlers; // Packet handlers (set_player_position, etc.)
mod validation; // Physics checks (gravity, collision)

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            // 1. Handle incoming movement packets
            .add_systems(Update, handlers::handle_set_player_position)
            .add_systems(Update, handlers::handle_set_player_rotation)
            .add_systems(Update, handlers::handle_set_player_position_and_rotation)
            .add_systems(Update, handlers::handle_teleport_confirm)
            // 2. Validate physics (Anti-Cheat / Server Authority)
            // This runs AFTER handlers update the "proposed" position
            .add_systems(
                Update,
                validation::validate_movement.after(handlers::handle_set_player_position),
            );
    }
}
