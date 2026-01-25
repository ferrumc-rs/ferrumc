pub mod digging_system;
pub mod entity_spawn;
pub mod gamemode_change;
pub mod player_despawn;
pub mod player_join_message;
pub mod player_leave_message;
pub mod player_spawn;
pub mod player_tp;

pub fn register_gameplay_listeners(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(player_leave_message::handle);
    schedule.add_systems(player_join_message::handle);
    schedule.add_systems(player_spawn::handle);
    schedule.add_systems(player_despawn::handle);
    schedule.add_systems(gamemode_change::handle);
    schedule.add_systems(entity_spawn::spawn_command_processor);
    schedule.add_systems(entity_spawn::handle_spawn_entity);
    schedule.add_systems(digging_system::handle_start_digging);
    schedule.add_systems(digging_system::handle_cancel_digging);
    schedule.add_systems(digging_system::handle_finish_digging);
    schedule.add_systems(player_tp::teleport_player);
}
