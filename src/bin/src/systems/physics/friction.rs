use bevy_ecs::prelude::{Query, With};
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::velocity::Velocity;
use ferrumc_entities::markers::HasGravity;

const GROUND_FRICTION: f32 = 0.6;
const AIR_FRICTION: f32 = 0.91;
const DRAG: f32 = 0.98;

const STOP_THRESHOLD: f32 = 0.005;

// Gated on `HasGravity` to stay consistent with the rest of the physics pipeline (gravity and water
// drag are gated the same way). Friction is the companion to gravity: it damps the velocity that
// gravity and knockback build up, so it applies to the same physical entities and leaves
// non-physical ones (e.g. flying/swimming mobs spawned without gravity) untouched.
pub fn handle(mut query: Query<(&mut Velocity, &OnGround), With<HasGravity>>) {
    for (mut vel, on_ground) in query.iter_mut() {
        let h_friction = if on_ground.0 {
            GROUND_FRICTION
        } else {
            AIR_FRICTION
        };

        vel.vec.x *= h_friction;
        vel.vec.y *= DRAG;
        vel.vec.z *= h_friction;

        // Stop moving completely if velocity is very small
        if vel.vec.x.abs() < STOP_THRESHOLD {
            vel.vec.x = 0.0;
        }
        if vel.vec.y.abs() < STOP_THRESHOLD {
            vel.vec.y = 0.0;
        }
        if vel.vec.z.abs() < STOP_THRESHOLD {
            vel.vec.z = 0.0;
        }
    }
}
