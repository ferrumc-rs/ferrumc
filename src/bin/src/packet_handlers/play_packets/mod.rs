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
    schedule.add_system((
        chunk_batch_ack::handle,
        confirm_player_teleport::handle,
        keep_alive::handle,
        place_block::handle,
        player_action::handle,
        player_command::handle,
        set_player_position::handle,
        set_player_position_and_rotation::handle,
        set_player_rotation::handle,
        swing_arm::handle,
    ));
}
