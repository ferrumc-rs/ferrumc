use bevy_ecs::prelude::{MessageWriter, Query, With};
use bevy_math::Vec3A;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::markers::entity_types::Pig;
use ferrumc_messages::particle::SendParticle;
use ferrumc_particles::ParticleType;

pub fn tick_pig(
    query: Query<&Position, With<Pig>>,
    players: Query<&Position, With<PlayerIdentity>>,
    mut msgs: MessageWriter<SendParticle>,
) {
    for pos in query.iter() {
        for player_pos in players.iter() {
            let distance_sq = player_pos.as_vec3a().distance_squared(pos.as_vec3a());
            // Only spawn particles if a player is within 256 blocks
            if distance_sq > 16.0 * 256.0 {
                return;
            }
            // Spawn end rod particles from the pig to the player
            let steps = ferrumc_utils::maths::step::step_between(
                pos.coords.as_vec3a(),
                player_pos.coords.as_vec3a(),
                0.5,
            );
            // Limit to 32 particles to avoid spamming (16 blocks with a 0.5 step)
            for step_pos in steps.iter().take(32) {
                let particle_message = SendParticle {
                    particle_type: ParticleType::EndRod,
                    position: *step_pos,
                    offset: Vec3A::new(0.0, 0.0, 0.0),
                    speed: 0.0,
                    count: 1,
                };
                msgs.write(particle_message);
            }
        }
    }
}
