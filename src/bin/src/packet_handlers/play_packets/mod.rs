use crate::systems::connection_killer;
use bevy_ecs::prelude::IntoScheduleConfigs;
use bevy_ecs::schedule::Schedule;

mod chunk_batch_ack;
mod command;
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
    // Added separately so if we mess up the signature of one of the systems we can know exactly
    // which one
    schedule.add_systems(chunk_batch_ack::handle.before(connection_killer::connection_killer));
    schedule
        .add_systems(confirm_player_teleport::handle.before(connection_killer::connection_killer));
    schedule.add_systems(keep_alive::handle.before(connection_killer::connection_killer));
    schedule.add_systems(place_block::handle.before(connection_killer::connection_killer));
    schedule.add_systems(player_action::handle.before(connection_killer::connection_killer));
    schedule.add_systems(player_command::handle.before(connection_killer::connection_killer));
    schedule.add_systems(set_player_position::handle.before(connection_killer::connection_killer));
    schedule.add_systems(
        set_player_position_and_rotation::handle.before(connection_killer::connection_killer),
    );
    schedule.add_systems(set_player_rotation::handle.before(connection_killer::connection_killer));
    schedule.add_systems(swing_arm::handle.before(connection_killer::connection_killer));
    schedule.add_systems(command::handle.before(connection_killer::connection_killer));
}
