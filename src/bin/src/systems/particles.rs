use bevy_ecs::prelude::{MessageReader, Query};
use ferrumc_core::transform::position::Position;
use ferrumc_messages::particle::SendParticle;
use ferrumc_net::connection::StreamWriter;

pub fn handle(mut msgs: MessageReader<SendParticle>, query: Query<(&Position, &StreamWriter)>) {
    for message in msgs.read() {
        let double_pos = message.position.as_dvec3();
        let packet = ferrumc_net::packets::outgoing::particle::Particle {
            long_distance: false,
            always_visible: false,
            x: double_pos.x,
            y: double_pos.y,
            z: double_pos.z,
            offset_x: message.offset.x,
            offset_y: message.offset.y,
            offset_z: message.offset.z,
            max_speed: message.speed,
            count: message.count,
            particle_type: message.particle_type.clone(),
        };
        for (pos, writer) in query.iter() {
            let distance_sq = pos.as_vec3a().distance_squared(message.position);
            // 256 blocks radius
            if distance_sq <= 256.0 * 256.0 {
                writer
                    .send_packet_ref(&packet)
                    .expect("Failed to send particle packet");
            }
        }
    }
}
