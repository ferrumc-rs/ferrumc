mod animations;
mod play_packets;
mod player;
mod player_leave;

pub fn register_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    play_packets::register_packet_handlers(schedule);
    schedule.add_systems(player::head_rot::handle_player_move);
}
