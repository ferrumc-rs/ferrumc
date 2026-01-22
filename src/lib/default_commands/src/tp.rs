use bevy_ecs::prelude::{MessageWriter, Query};
use ferrumc_commands::arg::primitive::float::Float;
use ferrumc_commands::Sender;
use ferrumc_commands::Sender::Player;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::command;
use ferrumc_messages::teleport_player::TeleportPlayer;

#[command("tp")]
fn tp_command(
    #[sender] sender: Sender,
    #[arg] x: Float,
    #[arg] y: Float,
    #[arg] z: Float,
    args: (Query<&Rotation>, MessageWriter<TeleportPlayer>),
) {
    let (mut query, mut tp_player_msg) = args;
    let Player(entity) = sender else {
        sender.send_message("This command can only be used by players.".into(), false);
        return;
    };

    let Ok(rot) = query.get_mut(entity) else {
        sender.send_message("Could not find your player entity.".into(), false);
        return;
    };

    tp_player_msg.write(TeleportPlayer {
        entity,
        x: *x as f64,
        y: *y as f64,
        z: *z as f64,
        vel_x: 0.0,
        vel_y: 0.0,
        vel_z: 0.0,
        yaw: rot.yaw,
        pitch: rot.pitch,
    });

    sender.send_message(
        format!("Teleported to ({}, {}, {}).", *x, *y, *z).into(),
        false,
    );
}
