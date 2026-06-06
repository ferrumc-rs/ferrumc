use bevy_ecs::prelude::Query;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::velocity::Velocity;

const GROUND_FRICTION: f32 = 0.6;
const AIR_FRICTION: f32 = 0.91;
const DRAG: f32 = 0.98;

const STOP_THRESHOLD: f32 = 0.005;

pub fn handle(mut query: Query<(&mut Velocity, &OnGround)>) {
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
