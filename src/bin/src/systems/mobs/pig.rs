use bevy_ecs::prelude::{MessageWriter, Query, With};
use bevy_math::Vec3A;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::entity_types::Pig;
use ferrumc_messages::particle::SendParticle;
use ferrumc_particles::ParticleType;

pub fn tick_pig(
    query: Query<(&Position, &Velocity), With<Pig>>,
    mut msgs: MessageWriter<SendParticle>,
) {
    for (pos, vel) in query.iter() {
        let position = pos.as_vec3a() + Vec3A::new(0.0, 1.0, 0.0);
        let particle_message = SendParticle {
            particle_type: ParticleType::EndRod,
            position,
            offset: Vec3A::new(0.0, 0.0, 0.0),
            speed: 0.0,
            count: 1,
        };
        msgs.write(particle_message);
    }
}
