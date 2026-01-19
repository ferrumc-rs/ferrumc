use bevy_ecs::prelude::{MessageReader, MessageWriter, Query};
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_messages::chunk_calc::ChunkCalc;
use ferrumc_messages::entity_update::SendEntityUpdate;
use ferrumc_messages::teleport_player::TeleportPlayer;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use tracing::error;

pub fn teleport_player(
    mut query: Query<(&StreamWriter, &mut Position, &Rotation)>,
    mut message_reader: MessageReader<TeleportPlayer>,
    mut chunk_calc_msg: MessageWriter<ChunkCalc>,
    mut player_update_msg: MessageWriter<SendEntityUpdate>,
) {
    for message in message_reader.read() {
        let entity = message.entity;

        // Notify the chunk calculation system to recalculate chunks for this player
        chunk_calc_msg.write(ChunkCalc(entity));

        // Notify the player update system to send the new position to the client
        player_update_msg.write(SendEntityUpdate(entity));
        let Ok((conn, mut pos, rot)) = query.get_mut(entity) else {
            continue;
        };
        pos.x = message.x;
        pos.y = message.y;
        pos.z = message.z;
        if let Err(err) = conn.send_packet(SynchronizePlayerPositionPacket {
            teleport_id: rand::random::<i32>().into(),
            x: message.x,
            y: message.y,
            z: message.z,
            vel_x: message.vel_x,
            vel_y: message.vel_y,
            vel_z: message.vel_z,
            yaw: rot.yaw,
            pitch: rot.pitch,
            flags: 0,
        }) {
            error!("Failed to send teleport packet: {}", err);
            continue;
        }
    }
}
