use bevy_ecs::schedule::Schedule;

mod chunk_batch_ack;
mod confirm_player_teleport;
mod keep_alive;
mod place_block;
mod player_action;
mod player_command;
mod set_player_position;
mod set_player_position_and_rotation;
mod set_player_rotation;
mod swing_arm;

pub fn register_packet_handlers(schedule: &mut Schedule) {
    schedule.add_systems(chunk_batch_ack::handle);
    schedule.add_systems(confirm_player_teleport::handle);
    schedule.add_systems(keep_alive::handle);
    schedule.add_systems(place_block::handle);
    schedule.add_systems(player_action::handle);
    schedule.add_systems(player_command::handle);
    schedule.add_systems(set_player_position::handle);
    schedule.add_systems(set_player_position_and_rotation::handle);
    schedule.add_systems(set_player_rotation::handle);
    schedule.add_systems(swing_arm::handle);
}
