use bevy_ecs::schedule::Schedule;

mod chunk_batch_ack;
mod confirm_player_teleport;
mod keep_alive;
mod place_block;
mod player_action;

pub fn register_packet_handlers(schedule: &mut Schedule) {
    schedule.add_system((
        chunk_batch_ack::handle,
        confirm_player_teleport::handle,
        keep_alive::handle,
        place_block::handle,
        player_action::handle,
    ));
}
