use bevy_ecs::schedule::Schedule;

mod change_game_mode;
mod chat_message;
mod chunk_batch_ack;
mod command;
mod command_suggestions;
mod confirm_player_teleport;
mod keep_alive;
mod pick_item_from_block;
mod place_block;
mod player_abilities;
mod player_action;
mod player_command;
mod player_loaded;
mod set_player_position;
mod set_player_position_and_rotation;
mod set_player_rotation;
mod swing_arm;

pub fn register_packet_handlers(schedule: &mut Schedule) {
    // Added separately so if we mess up the signature of one of the systems we can know exactly
    // which one
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
    schedule.add_systems(player_loaded::handle);
    schedule.add_systems(command::handle);
    schedule.add_systems(command_suggestions::handle);
    schedule.add_systems(chat_message::handle);
    schedule.add_systems(set_creative_mode_slot::handle);
    schedule.add_systems(set_held_item::handle);
    schedule.add_systems(player_abilities::handle);
    schedule.add_systems(change_game_mode::handle);
    schedule.add_systems(pick_item_from_block::handle);
}

pub mod set_creative_mode_slot;

pub mod set_held_item;
