mod animations;
mod chat_message;
mod commands;
mod handshake;
mod login_process;
pub(crate) mod play_packets;
mod player;

pub fn register_player_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(player::head_rot::handle_player_move);
}
