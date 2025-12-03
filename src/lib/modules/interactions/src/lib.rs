use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

mod inventory; // Slot clicking, Hotbar swapping
mod use_item; // Eating, Drinking, Right-clicking blocks

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Inventory Management
            .add_systems(Update, inventory::handle_set_held_item)
            .add_systems(Update, inventory::handle_creative_inventory_action)
            // .add_systems(Update, inventory::handle_window_click) // Future
            // Item Usage
            .add_systems(Update, use_item::handle_player_use_item)
            .add_systems(Update, use_item::handle_swing_arm)
            // Block Picking
            .add_systems(Update, inventory::handle_pick_item);
    }
}
