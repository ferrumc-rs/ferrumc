use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::packet_handlers::player;

mod send_shutdown_packet;

pub fn register_shutdown_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(
        (
            send_shutdown_packet::handle,
            player::player_disconnect::handle,
        )
            .chain(),
    );
}
