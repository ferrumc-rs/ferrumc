use bevy_math::Vec3A;

pub const GRAVITY_ACCELERATION: Vec3A = Vec3A::new(0.0, -0.08, 0.0);

pub const TERMINAL_VELOCITY_Y: f64 = -3.92;

const WATER_BUOYANCY: f64 = 0.09;

const WATER_DRAG: f64 = 0.8;

const WATER_VERTICAL_DRAG: f64 = 0.95;

const GROUND_FRICTION: f64 = 0.85;

const AIR_RESISTANCE: f64 = 0.98;
