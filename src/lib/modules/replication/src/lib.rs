use bevy_app::{App, Plugin, PostUpdate, Update};
use bevy_ecs::prelude::*;

mod chunks; // Sending terrain
mod entities; // Sending movement/spawning
mod inventory; // Sending slot updates

pub struct ReplicationPlugin;

impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut App) {
        app
            // --- Chunks ---
            // Handles loading/sending chunks when players move
            .add_systems(Update, chunks::handle_chunk_boundaries)
            .add_systems(Update, chunks::handle_chunk_batch_ack)
            // --- Entities ---
            // Syncs entity movement to other players (The Reflector)
            // Runs in PostUpdate to ensure all movement logic has finished
            .add_systems(PostUpdate, entities::sync_entity_movement)
            .add_systems(PostUpdate, entities::sync_head_rotation)
            // --- Inventory ---
            // Syncs inventory changes (Changed<Inventory>)
            .add_systems(PostUpdate, inventory::sync_inventory_changes);
    }
}
