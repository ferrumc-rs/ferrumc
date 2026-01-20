use bevy_ecs::prelude::{MessageWriter, Query};
use ferrumc_commands::arg::position::CommandPosition;
use ferrumc_commands::Sender;
use ferrumc_commands::Sender::Player;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::command;
use ferrumc_messages::teleport_player::TeleportPlayer;

#[command("tp")]
fn tp_command(
    #[sender] sender: Sender,
    #[arg] pos: CommandPosition,
    args: (Query<(&Rotation, &Position)>, MessageWriter<TeleportPlayer>),
) {
    let (mut query, mut tp_player_msg) = args;
    let Player(entity) = sender else {
        sender.send_message("This command can only be used by players.".into(), false);
        return;
    };

    let Ok((rot, position)) = query.get_mut(entity) else {
        sender.send_message("Could not find your player entity.".into(), false);
        return;
    };
    let resolved_pos = pos.resolve(position);

    tp_player_msg.write(TeleportPlayer {
        entity,
        x: resolved_pos.x,
        y: resolved_pos.y,
        z: resolved_pos.z,
        vel_x: 0.0,
        vel_y: 0.0,
        vel_z: 0.0,
        yaw: rot.yaw,
        pitch: rot.pitch,
    });

    sender.send_message(format!("Teleported to ({}).", resolved_pos).into(), false);
}
