pub mod digging_system;
pub mod gamemode_change;
pub mod player_join_message;
pub mod player_leave_message;

pub fn register_gameplay_listeners(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(player_leave_message::handle);
    schedule.add_systems(player_join_message::handle);
    schedule.add_systems(gamemode_change::handle);
    schedule.add_systems(digging_system::handle_start_digging);
    schedule.add_systems(digging_system::handle_cancel_digging);
    schedule.add_systems(digging_system::handle_finish_digging);
}
