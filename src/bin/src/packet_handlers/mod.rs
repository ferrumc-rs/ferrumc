mod play_packets;
mod player;

pub fn register_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    play_packets::register_packet_handlers(schedule);
    schedule.add_systems(player::head_rot::handle_player_move);
}
