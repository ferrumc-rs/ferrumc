#![expect(clippy::type_complexity)]
use bevy_ecs::prelude::{MessageWriter, Query};
use ferrumc_commands::arg::primitive::float::Float;
use ferrumc_commands::Sender;
use ferrumc_commands::Sender::Player;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::command;
use ferrumc_messages::chunk_calc::ChunkCalc;
use ferrumc_messages::entity_update::SendEntityUpdate;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;

#[command("tp")]
fn tp_command(
    #[sender] sender: Sender,
    #[arg] x: Float,
    #[arg] y: Float,
    #[arg] z: Float,
    args: (
        Query<(&StreamWriter, &mut Position, &Rotation)>,
        MessageWriter<ChunkCalc>,
        MessageWriter<SendEntityUpdate>,
    ),
) {
    let (mut query, mut chunk_calc_msg, mut player_update_msg) = args;
    let Player(entity) = sender else {
        sender.send_message("This command can only be used by players.".into(), false);
        return;
    };

    let Ok((conn, mut pos, rot)) = query.get_mut(entity) else {
        sender.send_message("Could not find your player entity.".into(), false);
        return;
    };

    pos.x = *x as f64;
    pos.y = *y as f64;
    pos.z = *z as f64;

    // Notify the chunk calculation system to recalculate chunks for this player
    chunk_calc_msg.write(ChunkCalc(entity));

    // Notify the player update system to send the new position to the client
    player_update_msg.write(SendEntityUpdate(entity));

    if let Err(err) = conn.send_packet(SynchronizePlayerPositionPacket {
        teleport_id: rand::random::<i32>().into(),
        x: *x as f64,
        y: *y as f64,
        z: *z as f64,
        vel_x: 0.0,
        vel_y: 0.0,
        vel_z: 0.0,
        yaw: rot.yaw,
        pitch: rot.pitch,
        flags: 0,
    }) {
        sender.send_message(
            format!("Failed to send teleport packet: {}", err).into(),
            false,
        );
        return;
    }

    sender.send_message(
        format!("Teleported to ({}, {}, {}).", *x, *y, *z).into(),
        false,
    );
}
