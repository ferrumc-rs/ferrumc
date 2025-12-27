use bevy_ecs::prelude::Message;
use ferrumc_particles::ParticleType;

#[derive(Message)]
pub struct SendParticle {
    pub particle_type: ParticleType,
    pub position: bevy_math::Vec3A,
    pub offset: bevy_math::Vec3A,
    pub speed: f32,
    pub count: i32,
}
