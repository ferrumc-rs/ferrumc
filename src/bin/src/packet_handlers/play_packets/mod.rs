use bevy_ecs::schedule::Schedule;

mod chunk_batch_ack;
mod confirm_player_teleport;
mod keep_alive;

pub fn register_packet_handlers(schedule: &mut Schedule) {
    schedule
        .add_system((chunk_batch_ack::handle, confirm_player_teleport::confirm_player_teleport));
}