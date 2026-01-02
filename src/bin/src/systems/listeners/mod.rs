mod clear_player_inventory;
pub mod digging_system;
pub mod entity_spawn;
pub mod experience;
pub mod gamemode_change;
pub mod give_item_to_player;
pub mod player_join_message;
pub mod player_leave_message;

pub fn register_gameplay_listeners(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(player_leave_message::handle);
    schedule.add_systems(player_join_message::handle);
    schedule.add_systems(gamemode_change::handle);
    schedule.add_systems(entity_spawn::spawn_command_processor);
    schedule.add_systems(entity_spawn::handle_spawn_entity);
    schedule.add_systems(digging_system::handle_start_digging);
    schedule.add_systems(digging_system::handle_cancel_digging);
    schedule.add_systems(digging_system::handle_finish_digging);
    schedule.add_systems(clear_player_inventory::handle_clear_player_inventory);
    schedule.add_systems(experience::player_gained_xp_handler);
    schedule.add_systems(give_item_to_player::give_item_to_player_handler);
}
