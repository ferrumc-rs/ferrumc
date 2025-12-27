use bevy_ecs::prelude::Message;
use ferrumc_particles::ParticleType;

/// Represents a message to send particle data in the system.
///
/// This struct is used to define the properties of a particle effect
/// that can be sent as a message. It includes the type of particle,
/// its position, offset, speed, and the number of particles to generate.
#[derive(Message)]
pub struct SendParticle {
    /// The type of particle to be sent.
    pub particle_type: ParticleType,
    /// The position where the particle effect will be generated.
    pub position: bevy_math::Vec3A,
    /// The offset applied to the particle's position.
    pub offset: bevy_math::Vec3A,
    /// The speed at which the particle moves.
    pub speed: f32,
    /// The number of particles to generate.
    pub count: i32,
}
