mod send_shutdown_packet;

pub fn register_shutdown_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(send_shutdown_packet::handle);
}
