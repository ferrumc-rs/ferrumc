use ferrumc_macros::{packet, NetEncode};
#[derive(NetEncode)]
#[packet(packet_id = "level_particles", state = "play")]
pub struct Particle {
    pub long_distance: bool,
    pub always_visible: bool,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
    pub max_speed: f32,
    pub count: i32,
    pub particle_type: ferrumc_particles::ParticleType,
}
