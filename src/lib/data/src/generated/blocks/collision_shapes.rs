#[derive(Clone, Copy, Debug)]
pub struct CollisionShape {
    pub min: [f64; 3],
    pub max: [f64; 3],
}
pub const COLLISION_SHAPES: &[CollisionShape] = &[
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.125f64],
        max: [0.875f64, 0.75f64, 0.875f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.8125f64, 0.1875f64],
        max: [0.3125f64, 1f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.3125f64, 1f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.625f64, 0.1875f64],
        max: [0.3125f64, 1f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.4375f64, 0.1875f64],
        max: [0.3125f64, 1f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0.1875f64],
        max: [0.3125f64, 1f64, 0.3125f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.1875f64, 0.5625f64, 0.1875f64],
    },
    CollisionShape {
        min: [0.8125f64, 0f64, 0f64],
        max: [1f64, 0.5625f64, 0.1875f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0.1875f64],
        max: [1f64, 0.5625f64, 1f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0f64],
        max: [0.8125f64, 0.5625f64, 0.1875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.8125f64],
        max: [0.1875f64, 0.5625f64, 1f64],
    },
    CollisionShape {
        min: [0.8125f64, 0f64, 0.8125f64],
        max: [1f64, 0.5625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0f64],
        max: [1f64, 0.5625f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0.8125f64],
        max: [0.8125f64, 0.5625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0.1875f64],
        max: [1f64, 0.5625f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0f64],
        max: [1f64, 0.5625f64, 0.1875f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0.8125f64],
        max: [1f64, 0.5625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0f64],
        max: [0.8125f64, 0.5625f64, 1f64],
    },
    CollisionShape {
        min: [0.8125f64, 0.1875f64, 0.1875f64],
        max: [1f64, 0.5625f64, 0.8125f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.125f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.5f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.25f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.75f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.75f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.25f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.125f64],
        max: [0.875f64, 0.8125f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.125f64],
        max: [0.875f64, 0.625f64, 0.875f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.9375f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.125f64],
        max: [0.875f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 1f64, 0.25f64],
    },
    CollisionShape {
        min: [0.375f64, 0.375f64, 0.25f64],
        max: [0.625f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0.375f64, 0.375f64, 0.25f64],
        max: [0.625f64, 0.625f64, 1.25f64],
    },
    CollisionShape {
        min: [0.75f64, 0f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.375f64, 0.375f64],
        max: [0.75f64, 0.625f64, 0.625f64],
    },
    CollisionShape {
        min: [-0.25f64, 0.375f64, 0.375f64],
        max: [0.75f64, 0.625f64, 0.625f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.75f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.375f64, 0.375f64, 0f64],
        max: [0.625f64, 0.625f64, 0.75f64],
    },
    CollisionShape {
        min: [0.375f64, 0.375f64, -0.25f64],
        max: [0.625f64, 0.625f64, 0.75f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.25f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.25f64, 0.375f64, 0.375f64],
        max: [1f64, 0.625f64, 0.625f64],
    },
    CollisionShape {
        min: [0.25f64, 0.375f64, 0.375f64],
        max: [1.25f64, 0.625f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.375f64],
        max: [0.625f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0f64, 0.75f64, 0f64],
        max: [0.375f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.375f64, 0.75f64, 0f64],
        max: [1f64, 1f64, 0.375f64],
    },
    CollisionShape {
        min: [0.375f64, 0.75f64, 0.625f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.625f64, 0.75f64, 0.375f64],
        max: [1f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, -0.25f64, 0.375f64],
        max: [0.625f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.25f64, 1f64],
    },
    CollisionShape {
        min: [0.375f64, 0.25f64, 0.375f64],
        max: [0.625f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0.25f64, 0.375f64],
        max: [0.625f64, 1.25f64, 0.625f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.4375f64, 0.625f64, 0.4375f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.3125f64],
        max: [0.6875f64, 0.375f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.375f64],
        max: [0.625f64, 0.625f64, 0.625f64],
    },
    CollisionShape {
        min: [0.34375f64, 0.1875f64, 0.6875f64],
        max: [0.65625f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.34375f64, 0.1875f64, 0f64],
        max: [0.65625f64, 0.8125f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.6875f64, 0.1875f64, 0.34375f64],
        max: [1f64, 0.8125f64, 0.65625f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0.34375f64],
        max: [0.3125f64, 0.8125f64, 0.65625f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.0625f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0f64],
        max: [1f64, 1f64, 0.0625f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.9375f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0f64, 0.0625f64],
        max: [1f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.9375f64, 0.0625f64],
        max: [0.9375f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 1f64, 0.0625f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.9375f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.9375f64, 0.0625f64],
        max: [0.9375f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.9375f64, 0f64, 0.0625f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.9375f64, 0.0625f64],
        max: [0.9375f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.9375f64, 0.0625f64],
        max: [0.9375f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0f64, 0f64],
        max: [1f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.9375f64, 0f64],
        max: [0.9375f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0.9375f64, 0f64],
        max: [0.9375f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.9375f64, 0f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.9375f64, 0f64],
        max: [0.9375f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.9375f64, 0f64],
        max: [0.9375f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.9375f64, 0.0625f64],
        max: [1f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0.9375f64, 0.0625f64],
        max: [1f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.9375f64, 0.0625f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.9375f64, 0.0625f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.9375f64, 0f64],
        max: [1f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0.9375f64, 0f64],
        max: [1f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.9375f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.9375f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.0625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 1f64, 0.5f64],
    },
    CollisionShape {
        min: [0f64, 0.5f64, 0.5f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.5f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.5f64, 0f64, 0f64],
        max: [1f64, 1f64, 0.5f64],
    },
    CollisionShape {
        min: [0.5f64, 0.5f64, 0.5f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.5f64, 0f64, 0.5f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.5f64, 0.5f64],
        max: [0.5f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.5f64, 1f64, 0.5f64],
    },
    CollisionShape {
        min: [0.5f64, 0.5f64, 0f64],
        max: [1f64, 1f64, 0.5f64],
    },
    CollisionShape {
        min: [0f64, 0.5f64, 0f64],
        max: [0.5f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.5f64, 0f64],
        max: [1f64, 1f64, 0.5f64],
    },
    CollisionShape {
        min: [0f64, 0.5f64, 0f64],
        max: [0.5f64, 1f64, 0.5f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.5f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.5f64],
        max: [0.5f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.5f64, 0.5f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.5f64, 0f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.875f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [1f64, 0.875f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.875f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0f64],
        max: [0.9375f64, 0.875f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.875f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.1875f64],
        max: [1f64, 0.0625f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0f64],
        max: [0.8125f64, 0.0625f64, 0.1875f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.8125f64],
        max: [0.8125f64, 0.0625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.0625f64, 0.1875f64],
        max: [0.0625f64, 1f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.0625f64, 0f64],
        max: [0.8125f64, 1f64, 0.0625f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.0625f64, 0.9375f64],
        max: [0.8125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.0625f64, 0.1875f64],
        max: [1f64, 1f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0f64],
        max: [0.8125f64, 0.0625f64, 1f64],
    },
    CollisionShape {
        min: [0.8125f64, 0f64, 0.1875f64],
        max: [1f64, 0.0625f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0f64],
        max: [0.8125f64, 0.0625f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.8125f64, 0.0625f64, 1f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [1f64, 0.0625f64, 0.8125f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.1875f64],
        max: [0.8125f64, 0.0625f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.8125f64, 0.0625f64, 0.8125f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.375f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.875f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.9375f64, 1f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0.25f64],
        max: [0.75f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.1875f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.8125f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.8125f64, 0f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 1f64, 0.1875f64],
    },
    CollisionShape {
        min: [0f64, 0.28125f64, 0.875f64],
        max: [1f64, 0.78125f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.28125f64, 0f64],
        max: [1f64, 0.78125f64, 0.125f64],
    },
    CollisionShape {
        min: [0.875f64, 0.28125f64, 0f64],
        max: [1f64, 0.78125f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.28125f64, 0f64],
        max: [0.125f64, 0.78125f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.4375f64],
        max: [0.9375f64, 0.625f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.8125f64, 1f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.0625f64],
        max: [0.5625f64, 0.625f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0.875f64, 0.375f64],
        max: [1f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0.875f64, 0f64],
        max: [0.625f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.25f64],
        max: [0.6875f64, 0.375f64, 0.75f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0.3125f64],
        max: [0.75f64, 0.375f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.25f64, 0.625f64],
        max: [0.6875f64, 0.75f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.25f64, 0f64],
        max: [0.6875f64, 0.75f64, 0.375f64],
    },
    CollisionShape {
        min: [0.625f64, 0.25f64, 0.3125f64],
        max: [1f64, 0.75f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0.25f64, 0.3125f64],
        max: [0.375f64, 0.75f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.625f64, 0.25f64],
        max: [0.6875f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0.25f64, 0.625f64, 0.3125f64],
        max: [0.75f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.03125f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.0625f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.375f64],
        max: [0.6875f64, 0.0625f64, 0.625f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.375f64],
        max: [0.6875f64, 0.125f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.3125f64],
        max: [0.625f64, 0.0625f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.3125f64],
        max: [0.625f64, 0.125f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.375f64, 0.9375f64],
        max: [0.6875f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.375f64, 0.875f64],
        max: [0.6875f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.375f64, 0f64],
        max: [0.6875f64, 0.625f64, 0.0625f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.375f64, 0f64],
        max: [0.6875f64, 0.625f64, 0.125f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.375f64, 0.3125f64],
        max: [1f64, 0.625f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.875f64, 0.375f64, 0.3125f64],
        max: [1f64, 0.625f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0.375f64, 0.3125f64],
        max: [0.0625f64, 0.625f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0.375f64, 0.3125f64],
        max: [0.125f64, 0.625f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.9375f64, 0.375f64],
        max: [0.6875f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.875f64, 0.375f64],
        max: [0.6875f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0.9375f64, 0.3125f64],
        max: [0.625f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.375f64, 0.875f64, 0.3125f64],
        max: [0.625f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.9375f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.75f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.375f64],
        max: [1f64, 1.5f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0f64],
        max: [0.625f64, 1.5f64, 0.375f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.625f64],
        max: [0.625f64, 1.5f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.375f64],
        max: [1f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0f64],
        max: [0.625f64, 1f64, 0.375f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.625f64],
        max: [0.625f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0f64],
        max: [0.625f64, 1.5f64, 1f64],
    },
    CollisionShape {
        min: [0.625f64, 0f64, 0.375f64],
        max: [1f64, 1.5f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0f64],
        max: [0.625f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.625f64, 0f64, 0.375f64],
        max: [1f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0f64],
        max: [0.625f64, 1.5f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0f64],
        max: [0.625f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.375f64],
        max: [0.625f64, 1.5f64, 1f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.375f64],
        max: [0.625f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.375f64],
        max: [1f64, 1.5f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.375f64],
        max: [1f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.375f64],
        max: [0.625f64, 1.5f64, 0.625f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.375f64],
        max: [0.625f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.375f64],
        max: [0.625f64, 1.5f64, 0.625f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.5f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.5f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.5f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.5f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.5625f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.5f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.6875f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.5f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.8125f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.5f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0.8125f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.1875f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.4375f64],
        max: [1f64, 1f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0f64],
        max: [0.5625f64, 1f64, 0.4375f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.5625f64],
        max: [0.5625f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0f64],
        max: [0.5625f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.5625f64, 0f64, 0.4375f64],
        max: [1f64, 1f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0f64],
        max: [0.5625f64, 1f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.4375f64],
        max: [0.5625f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.4375f64],
        max: [1f64, 1f64, 0.5625f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.4375f64],
        max: [0.5625f64, 1f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.4375f64],
        max: [0.5625f64, 1f64, 0.5625f64],
    },
    CollisionShape {
        min: [0f64, 0.40625f64, 0.40625f64],
        max: [1f64, 0.59375f64, 0.59375f64],
    },
    CollisionShape {
        min: [0.40625f64, 0f64, 0.40625f64],
        max: [0.59375f64, 1f64, 0.59375f64],
    },
    CollisionShape {
        min: [0.40625f64, 0.40625f64, 0f64],
        max: [0.59375f64, 0.59375f64, 1f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0f64],
        max: [0.625f64, 0.625f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.375f64],
        max: [0.625f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.375f64],
        max: [0.625f64, 0.625f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.375f64],
        max: [1f64, 0.625f64, 0.625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.4375f64],
        max: [0.5625f64, 0.125f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.4375f64],
        max: [0.5625f64, 0.25f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.4375f64],
        max: [0.5625f64, 0.375f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.4375f64],
        max: [0.5625f64, 0.5f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.4375f64],
        max: [0.5625f64, 0.625f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.4375f64],
        max: [0.5625f64, 0.75f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0f64, 0.4375f64],
        max: [0.5625f64, 0.875f64, 0.5625f64],
    },
    CollisionShape {
        min: [0f64, 0.0625f64, 0f64],
        max: [0.0625f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.0625f64, 0f64],
        max: [1f64, 1f64, 0.0625f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.0625f64, 0.9375f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.0625f64, 0.0625f64],
        max: [1f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0.0625f64, 0f64],
        max: [1f64, 1f64, 0.0625f64],
    },
    CollisionShape {
        min: [0f64, 0.0625f64, 0.9375f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.0625f64, 0.0625f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.0625f64, 0f64],
        max: [1f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.0625f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.375f64],
        max: [1f64, 0.8125f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0f64],
        max: [0.625f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.09375f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0.5f64, 0f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0.25f64],
        max: [0.75f64, 1.5f64, 0.75f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.3125f64],
        max: [0.75f64, 1.5f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0.25f64],
        max: [0.75f64, 1.5f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0.6875f64],
        max: [0.75f64, 1.5f64, 0.75f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.3125f64],
        max: [0.75f64, 0.875f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0.25f64],
        max: [0.75f64, 1f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0.6875f64],
        max: [0.75f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0.25f64, 0.875f64, 0.3125f64],
        max: [0.75f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.3125f64],
        max: [0.75f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.3125f64],
        max: [0.6875f64, 1.5f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.3125f64],
        max: [0.6875f64, 0.875f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.3125f64],
        max: [0.6875f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.75f64],
        max: [0.6875f64, 1.5f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.75f64],
        max: [0.6875f64, 0.875f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.3125f64],
        max: [0.6875f64, 1.5f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.3125f64],
        max: [0.6875f64, 0.875f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.6875f64],
        max: [0.6875f64, 1.5f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.6875f64],
        max: [0.6875f64, 0.875f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.75f64],
        max: [0.6875f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.3125f64],
        max: [0.6875f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.6875f64],
        max: [0.6875f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.875f64, 0.3125f64],
        max: [0.6875f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 1.5f64, 0.25f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 0.875f64, 0.25f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 1.5f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 0.875f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 1.5f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 0.875f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 1.5f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 0.875f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.875f64, 0.3125f64],
        max: [0.6875f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 1f64, 0.25f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 1f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.875f64, 0f64],
        max: [0.6875f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.75f64, 0f64, 0.3125f64],
        max: [1f64, 1.5f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.75f64, 0f64, 0.3125f64],
        max: [1f64, 0.875f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.3125f64],
        max: [1f64, 1.5f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.3125f64],
        max: [1f64, 0.875f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0.875f64, 0.3125f64],
        max: [0.75f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.3125f64],
        max: [1f64, 1.5f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.3125f64],
        max: [1f64, 0.875f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0.875f64, 0.3125f64],
        max: [0.6875f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.6875f64, 0f64, 0.3125f64],
        max: [1f64, 1.5f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.6875f64, 0f64, 0.3125f64],
        max: [1f64, 0.875f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.75f64, 0f64, 0.3125f64],
        max: [1f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.25f64, 0.875f64, 0.3125f64],
        max: [1f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.3125f64],
        max: [1f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.3125f64],
        max: [1f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.875f64, 0.3125f64],
        max: [1f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.6875f64, 0f64, 0.3125f64],
        max: [1f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.3125f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.6875f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.125f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.4375f64, 0.125f64, 0.4375f64],
        max: [0.5625f64, 0.875f64, 0.5625f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.125f64, 1f64, 0.25f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.75f64],
        max: [0.125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0f64],
        max: [0.25f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.875f64],
        max: [0.25f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.75f64, 0f64, 0f64],
        max: [1f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.75f64, 0f64, 0.875f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.875f64, 0f64, 0.125f64],
        max: [1f64, 1f64, 0.25f64],
    },
    CollisionShape {
        min: [0.875f64, 0f64, 0.75f64],
        max: [1f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0.25f64],
        max: [1f64, 0.25f64, 0.75f64],
    },
    CollisionShape {
        min: [0.125f64, 0.1875f64, 0.125f64],
        max: [0.875f64, 0.25f64, 0.25f64],
    },
    CollisionShape {
        min: [0.125f64, 0.1875f64, 0.75f64],
        max: [0.875f64, 0.25f64, 0.875f64],
    },
    CollisionShape {
        min: [0.25f64, 0.1875f64, 0f64],
        max: [0.75f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.25f64, 0.1875f64, 0.875f64],
        max: [0.75f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.25f64, 0.25f64],
        max: [0.125f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0.875f64, 0.25f64, 0.25f64],
        max: [1f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0f64, 0.375f64, 0f64],
        max: [1f64, 0.75f64, 1f64],
    },
    CollisionShape {
        min: [0.25f64, 0.8125f64, 0.25f64],
        max: [0.75f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0.375f64, 0.4375f64, 0.0625f64],
        max: [0.625f64, 0.75f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.375f64, 0.4375f64, 0.6875f64],
        max: [0.625f64, 0.75f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.4375f64, 0.375f64],
        max: [0.3125f64, 0.75f64, 0.625f64],
    },
    CollisionShape {
        min: [0.6875f64, 0.4375f64, 0.375f64],
        max: [0.9375f64, 0.75f64, 0.625f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.3125f64, 0.0625f64],
        max: [0.6875f64, 0.75f64, 0.4375f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.3125f64, 0.5625f64],
        max: [0.6875f64, 0.75f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.3125f64, 0.3125f64],
        max: [0.4375f64, 0.75f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.5625f64, 0.3125f64, 0.3125f64],
        max: [0.9375f64, 0.75f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.25f64, 0.1875f64, 0.0625f64],
        max: [0.75f64, 0.75f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.25f64, 0.1875f64, 0.4375f64],
        max: [0.75f64, 0.75f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.1875f64, 0.25f64],
        max: [0.5625f64, 0.75f64, 0.75f64],
    },
    CollisionShape {
        min: [0.4375f64, 0.1875f64, 0.25f64],
        max: [0.9375f64, 0.75f64, 0.75f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.625f64],
        max: [0.6875f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0f64],
        max: [0.6875f64, 0.625f64, 0.375f64],
    },
    CollisionShape {
        min: [0.625f64, 0f64, 0.3125f64],
        max: [1f64, 0.625f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.3125f64],
        max: [0.375f64, 0.625f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0.0625f64, 0f64],
        max: [1f64, 0.15625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.4375f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.5625f64, 1f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0.25f64],
        max: [0.75f64, 0.5f64, 0.75f64],
    },
    CollisionShape {
        min: [0.25f64, 0.25f64, 0.5f64],
        max: [0.75f64, 0.75f64, 1f64],
    },
    CollisionShape {
        min: [0.25f64, 0.25f64, 0f64],
        max: [0.75f64, 0.75f64, 0.5f64],
    },
    CollisionShape {
        min: [0.5f64, 0.25f64, 0.25f64],
        max: [1f64, 0.75f64, 0.75f64],
    },
    CollisionShape {
        min: [0f64, 0.25f64, 0.25f64],
        max: [0.5f64, 0.75f64, 0.75f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.8125f64, 0.5f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.25f64, 0.5f64],
        max: [0.8125f64, 0.75f64, 1f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.25f64, 0f64],
        max: [0.8125f64, 0.75f64, 0.5f64],
    },
    CollisionShape {
        min: [0.5f64, 0.25f64, 0.1875f64],
        max: [1f64, 0.75f64, 0.8125f64],
    },
    CollisionShape {
        min: [0f64, 0.25f64, 0.1875f64],
        max: [0.5f64, 0.75f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.125f64],
        max: [0.875f64, 0.25f64, 0.875f64],
    },
    CollisionShape {
        min: [0.25f64, 0.25f64, 0.1875f64],
        max: [0.75f64, 0.3125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.375f64, 0.3125f64, 0.25f64],
        max: [0.625f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.625f64, 0f64],
        max: [0.375f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.375f64, 0.625f64, 0f64],
        max: [0.8125f64, 1f64, 0.25f64],
    },
    CollisionShape {
        min: [0.375f64, 0.625f64, 0.75f64],
        max: [0.8125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.625f64, 0.625f64, 0.25f64],
        max: [0.8125f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.25f64, 0.25f64],
        max: [0.8125f64, 0.3125f64, 0.75f64],
    },
    CollisionShape {
        min: [0.25f64, 0.3125f64, 0.375f64],
        max: [0.75f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0f64, 0.625f64, 0.1875f64],
        max: [0.25f64, 1f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.25f64, 0.625f64, 0.1875f64],
        max: [1f64, 1f64, 0.375f64],
    },
    CollisionShape {
        min: [0.25f64, 0.625f64, 0.625f64],
        max: [1f64, 1f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.75f64, 0.625f64, 0.375f64],
        max: [1f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.375f64],
        max: [0.625f64, 0.6875f64, 0.625f64],
    },
    CollisionShape {
        min: [0.25f64, 0.25f64, 0.25f64],
        max: [0.375f64, 0.6875f64, 0.75f64],
    },
    CollisionShape {
        min: [0.375f64, 0.25f64, 0.25f64],
        max: [0.75f64, 0.6875f64, 0.375f64],
    },
    CollisionShape {
        min: [0.375f64, 0.25f64, 0.625f64],
        max: [0.75f64, 0.6875f64, 0.75f64],
    },
    CollisionShape {
        min: [0.625f64, 0.25f64, 0.375f64],
        max: [0.75f64, 0.6875f64, 0.625f64],
    },
    CollisionShape {
        min: [0f64, 0.625f64, 0f64],
        max: [0.25f64, 0.6875f64, 1f64],
    },
    CollisionShape {
        min: [0.25f64, 0.625f64, 0f64],
        max: [1f64, 0.6875f64, 0.25f64],
    },
    CollisionShape {
        min: [0.25f64, 0.625f64, 0.75f64],
        max: [1f64, 0.6875f64, 1f64],
    },
    CollisionShape {
        min: [0.75f64, 0.625f64, 0.25f64],
        max: [1f64, 0.6875f64, 0.75f64],
    },
    CollisionShape {
        min: [0f64, 0.6875f64, 0f64],
        max: [0.125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0.6875f64, 0f64],
        max: [1f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.125f64, 0.6875f64, 0.875f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.875f64, 0.6875f64, 0.125f64],
        max: [1f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0.25f64, 0.25f64, 0.25f64],
        max: [0.75f64, 0.6875f64, 0.75f64],
    },
    CollisionShape {
        min: [0.375f64, 0.25f64, 0f64],
        max: [0.625f64, 0.5f64, 0.25f64],
    },
    CollisionShape {
        min: [0.375f64, 0.25f64, 0.75f64],
        max: [0.625f64, 0.5f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.25f64, 0.375f64],
        max: [0.75f64, 0.5f64, 0.625f64],
    },
    CollisionShape {
        min: [0.25f64, 0.25f64, 0.25f64],
        max: [0.75f64, 0.6875f64, 0.375f64],
    },
    CollisionShape {
        min: [0.25f64, 0.25f64, 0.625f64],
        max: [0.75f64, 0.6875f64, 0.75f64],
    },
    CollisionShape {
        min: [0.25f64, 0.5f64, 0.375f64],
        max: [0.75f64, 0.6875f64, 0.625f64],
    },
    CollisionShape {
        min: [0.75f64, 0.25f64, 0.375f64],
        max: [1f64, 0.5f64, 0.625f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.875f64],
        max: [1f64, 0.78125f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.78125f64, 0.125f64],
    },
    CollisionShape {
        min: [0.875f64, 0f64, 0f64],
        max: [1f64, 0.78125f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.125f64, 0.78125f64, 1f64],
    },
    CollisionShape {
        min: [0.375f64, 0.375f64, 0f64],
        max: [0.625f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.375f64, 0.375f64],
        max: [1f64, 0.625f64, 0.625f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0.1875f64],
        max: [0.1875f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0f64],
        max: [0.8125f64, 0.8125f64, 0.1875f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0.8125f64],
        max: [0.8125f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.8125f64, 0.1875f64, 0.1875f64],
        max: [1f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.8125f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0.1875f64],
        max: [1f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.8125f64, 0.1875f64],
        max: [0.8125f64, 1f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0f64],
        max: [0.8125f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0f64],
        max: [0.8125f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0.1875f64],
        max: [0.8125f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0.1875f64],
        max: [1f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0.1875f64],
        max: [0.8125f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0.1875f64],
        max: [0.8125f64, 1f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0.1875f64],
        max: [0.8125f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.3125f64],
        max: [0.6875f64, 0.625f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, -0.0625f64, 0.3125f64],
        max: [0.6875f64, 0.1875f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.1875f64, -0.0625f64, 0.1875f64],
        max: [0.8125f64, 0.3125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, -0.0625f64, 0.1875f64],
        max: [0.8125f64, 0.875f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, -0.0625f64, 0.1875f64],
        max: [0.8125f64, 1f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.8125f64, 0.6875f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.8125f64, 0.9375f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.3125f64, 0.3125f64],
        max: [0.6875f64, 0.6875f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.75f64, 0.4375f64, 0.75f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.9375f64, 0.4375f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.125f64],
        max: [0.9375f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.8125f64, 0.625f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.125f64],
        max: [0.875f64, 0.9375f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0.25f64, 0.3125f64],
        max: [1f64, 0.75f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.25f64, 0f64],
        max: [1f64, 0.75f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.25f64, 0f64],
        max: [1f64, 0.75f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.25f64, 0f64],
        max: [0.6875f64, 0.75f64, 1f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.375f64],
        max: [0.625f64, 0.375f64, 0.625f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.8125f64, 0.375f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.125f64],
        max: [0.875f64, 0.375f64, 0.875f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.125f64],
        max: [0.875f64, 0.4375f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.5f64, 0.75f64, 0.5f64],
    },
    CollisionShape {
        min: [0.15625f64, 0f64, 0.15625f64],
        max: [0.34375f64, 1f64, 0.34375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.4375f64, 1f64, 0.4375f64],
    },
    CollisionShape {
        min: [-0.0625f64, 0f64, -0.0625f64],
        max: [0.5625f64, 1f64, 0.5625f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.125f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.875f64],
        max: [0.125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.875f64, 0f64, 0f64],
        max: [1f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.875f64, 0f64, 0.875f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.875f64, 0.125f64],
        max: [1f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0.125f64, 0.875f64, 0f64],
        max: [0.875f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.125f64, 0.875f64, 0.875f64],
        max: [0.875f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.125f64, 0f64],
        max: [0.125f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0f64, 0.125f64, 0.875f64],
        max: [0.125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.875f64, 0.125f64, 0f64],
        max: [1f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.875f64, 0.125f64, 0.875f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.375f64],
        max: [0.25f64, 0.8125f64, 0.625f64],
    },
    CollisionShape {
        min: [0.75f64, 0f64, 0.375f64],
        max: [0.875f64, 0.8125f64, 0.625f64],
    },
    CollisionShape {
        min: [0.25f64, 0.25f64, 0.125f64],
        max: [0.75f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0.125f64, 0.4375f64, 0.3125f64],
        max: [0.25f64, 0.8125f64, 0.375f64],
    },
    CollisionShape {
        min: [0.125f64, 0.4375f64, 0.625f64],
        max: [0.25f64, 0.8125f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.75f64, 0.4375f64, 0.3125f64],
        max: [0.875f64, 0.8125f64, 0.375f64],
    },
    CollisionShape {
        min: [0.75f64, 0.4375f64, 0.625f64],
        max: [0.875f64, 0.8125f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.125f64],
        max: [0.625f64, 0.8125f64, 0.25f64],
    },
    CollisionShape {
        min: [0.375f64, 0f64, 0.75f64],
        max: [0.625f64, 0.8125f64, 0.875f64],
    },
    CollisionShape {
        min: [0.125f64, 0.25f64, 0.25f64],
        max: [0.875f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.4375f64, 0.125f64],
        max: [0.375f64, 0.8125f64, 0.25f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.4375f64, 0.75f64],
        max: [0.375f64, 0.8125f64, 0.875f64],
    },
    CollisionShape {
        min: [0.625f64, 0.4375f64, 0.125f64],
        max: [0.6875f64, 0.8125f64, 0.25f64],
    },
    CollisionShape {
        min: [0.625f64, 0.4375f64, 0.75f64],
        max: [0.6875f64, 0.8125f64, 0.875f64],
    },
    CollisionShape {
        min: [0.25f64, 0.125f64, 0f64],
        max: [0.75f64, 0.875f64, 0.75f64],
    },
    CollisionShape {
        min: [0.125f64, 0.3125f64, 0.1875f64],
        max: [0.25f64, 0.6875f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.75f64, 0.3125f64, 0.1875f64],
        max: [0.875f64, 0.6875f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.125f64, 0.375f64, 0.5625f64],
        max: [0.25f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0.75f64, 0.375f64, 0.5625f64],
        max: [0.875f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0.25f64, 0.125f64, 0.25f64],
        max: [0.75f64, 0.875f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0.3125f64, 0.4375f64],
        max: [0.25f64, 0.6875f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.75f64, 0.3125f64, 0.4375f64],
        max: [0.875f64, 0.6875f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.125f64, 0.375f64, 0f64],
        max: [0.25f64, 0.625f64, 0.4375f64],
    },
    CollisionShape {
        min: [0.75f64, 0.375f64, 0f64],
        max: [0.875f64, 0.625f64, 0.4375f64],
    },
    CollisionShape {
        min: [0f64, 0.125f64, 0.25f64],
        max: [0.75f64, 0.875f64, 0.75f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.3125f64, 0.125f64],
        max: [0.5625f64, 0.6875f64, 0.25f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.3125f64, 0.75f64],
        max: [0.5625f64, 0.6875f64, 0.875f64],
    },
    CollisionShape {
        min: [0.5625f64, 0.375f64, 0.125f64],
        max: [1f64, 0.625f64, 0.25f64],
    },
    CollisionShape {
        min: [0.5625f64, 0.375f64, 0.75f64],
        max: [1f64, 0.625f64, 0.875f64],
    },
    CollisionShape {
        min: [0.25f64, 0.125f64, 0.25f64],
        max: [1f64, 0.875f64, 0.75f64],
    },
    CollisionShape {
        min: [0.4375f64, 0.3125f64, 0.125f64],
        max: [0.8125f64, 0.6875f64, 0.25f64],
    },
    CollisionShape {
        min: [0.4375f64, 0.3125f64, 0.75f64],
        max: [0.8125f64, 0.6875f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0.375f64, 0.125f64],
        max: [0.4375f64, 0.625f64, 0.25f64],
    },
    CollisionShape {
        min: [0f64, 0.375f64, 0.75f64],
        max: [0.4375f64, 0.625f64, 0.875f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0.125f64],
        max: [0.75f64, 0.75f64, 0.875f64],
    },
    CollisionShape {
        min: [0.125f64, 0.1875f64, 0.3125f64],
        max: [0.25f64, 0.5625f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.75f64, 0.1875f64, 0.3125f64],
        max: [0.875f64, 0.5625f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.125f64, 0.5625f64, 0.375f64],
        max: [0.25f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.75f64, 0.5625f64, 0.375f64],
        max: [0.875f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.25f64],
        max: [0.875f64, 0.75f64, 0.75f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.1875f64, 0.125f64],
        max: [0.6875f64, 0.5625f64, 0.25f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.1875f64, 0.75f64],
        max: [0.6875f64, 0.5625f64, 0.875f64],
    },
    CollisionShape {
        min: [0.375f64, 0.5625f64, 0.125f64],
        max: [0.625f64, 1f64, 0.25f64],
    },
    CollisionShape {
        min: [0.375f64, 0.5625f64, 0.75f64],
        max: [0.625f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0.25f64, 0.125f64, 0.25f64],
        max: [0.75f64, 0.875f64, 0.75f64],
    },
    CollisionShape {
        min: [0f64, 0.625f64, 0.0625f64],
        max: [0.25f64, 0.875f64, 0.3333333125f64],
    },
    CollisionShape {
        min: [0.25f64, 0.625f64, 0.0625f64],
        max: [1f64, 0.875f64, 0.25f64],
    },
    CollisionShape {
        min: [0.75f64, 0.625f64, 0.25f64],
        max: [1f64, 0.875f64, 0.3333333125f64],
    },
    CollisionShape {
        min: [0f64, 0.75f64, 0.3333333125f64],
        max: [0.25f64, 1f64, 0.6041666875f64],
    },
    CollisionShape {
        min: [0.75f64, 0.75f64, 0.3333333125f64],
        max: [1f64, 1f64, 0.6041666875f64],
    },
    CollisionShape {
        min: [0f64, 0.875f64, 0.6041666875f64],
        max: [1f64, 1.125f64, 0.875f64],
    },
    CollisionShape {
        min: [0.25f64, 0.875f64, 0.3333333125f64],
        max: [0.75f64, 1f64, 0.6041666875f64],
    },
    CollisionShape {
        min: [0f64, 0.625f64, 0.6666666875f64],
        max: [0.25f64, 0.875f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.25f64, 0.625f64, 0.75f64],
        max: [1f64, 0.875f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.75f64, 0.625f64, 0.6666666875f64],
        max: [1f64, 0.875f64, 0.75f64],
    },
    CollisionShape {
        min: [0f64, 0.75f64, 0.3958333125f64],
        max: [0.25f64, 1f64, 0.6666666875f64],
    },
    CollisionShape {
        min: [0.75f64, 0.75f64, 0.3958333125f64],
        max: [1f64, 1f64, 0.6666666875f64],
    },
    CollisionShape {
        min: [0f64, 0.875f64, 0.125f64],
        max: [1f64, 1.125f64, 0.3958333125f64],
    },
    CollisionShape {
        min: [0.25f64, 0.875f64, 0.3958333125f64],
        max: [0.75f64, 1f64, 0.6666666875f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.625f64, 0f64],
        max: [0.25f64, 0.875f64, 1f64],
    },
    CollisionShape {
        min: [0.25f64, 0.625f64, 0f64],
        max: [0.3333333125f64, 0.875f64, 0.25f64],
    },
    CollisionShape {
        min: [0.25f64, 0.625f64, 0.75f64],
        max: [0.3333333125f64, 0.875f64, 1f64],
    },
    CollisionShape {
        min: [0.3333333125f64, 0.75f64, 0f64],
        max: [0.6041666875f64, 1f64, 0.25f64],
    },
    CollisionShape {
        min: [0.3333333125f64, 0.75f64, 0.75f64],
        max: [0.6041666875f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.3333333125f64, 0.875f64, 0.25f64],
        max: [0.875f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0.6041666875f64, 0.875f64, 0f64],
        max: [0.875f64, 1.125f64, 0.25f64],
    },
    CollisionShape {
        min: [0.6041666875f64, 0.875f64, 0.75f64],
        max: [0.875f64, 1.125f64, 1f64],
    },
    CollisionShape {
        min: [0.6041666875f64, 1f64, 0.25f64],
        max: [0.875f64, 1.125f64, 0.75f64],
    },
    CollisionShape {
        min: [0.6666666875f64, 0.625f64, 0f64],
        max: [0.9375f64, 0.875f64, 0.25f64],
    },
    CollisionShape {
        min: [0.6666666875f64, 0.625f64, 0.75f64],
        max: [0.9375f64, 0.875f64, 1f64],
    },
    CollisionShape {
        min: [0.75f64, 0.625f64, 0.25f64],
        max: [0.9375f64, 0.875f64, 0.75f64],
    },
    CollisionShape {
        min: [0.3958333125f64, 0.75f64, 0f64],
        max: [0.6666666875f64, 1f64, 0.25f64],
    },
    CollisionShape {
        min: [0.3958333125f64, 0.75f64, 0.75f64],
        max: [0.6666666875f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0.875f64, 0f64],
        max: [0.3958333125f64, 1.125f64, 1f64],
    },
    CollisionShape {
        min: [0.3958333125f64, 0.875f64, 0.25f64],
        max: [0.6666666875f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.25f64],
        max: [1f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0f64],
        max: [0.75f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.25f64, 0.25f64, 0.25f64],
        max: [0.75f64, 0.375f64, 0.75f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.375f64, 0.3125f64],
        max: [0.6875f64, 0.8125f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.4375f64, 0.8125f64, 0.4375f64],
        max: [0.5625f64, 1f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0.8125f64, 0f64],
        max: [0.5625f64, 0.9375f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.4375f64, 0.8125f64, 0.1875f64],
        max: [0.5625f64, 0.9375f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.8125f64, 0.4375f64],
        max: [0.8125f64, 0.9375f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.8125f64, 0.4375f64],
        max: [1f64, 0.9375f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0.8125f64, 0f64],
        max: [0.5625f64, 0.9375f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.8125f64, 0.4375f64],
        max: [1f64, 0.9375f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.0625f64, 0.3125f64],
        max: [0.6875f64, 0.5f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.375f64, 0.5f64, 0.375f64],
        max: [0.625f64, 0.625f64, 0.625f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.3125f64],
        max: [0.6875f64, 0.4375f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.375f64, 0.4375f64, 0.375f64],
        max: [0.625f64, 0.5625f64, 0.625f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0.25f64],
        max: [0.75f64, 0.5625f64, 0.75f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.125f64],
        max: [0.875f64, 0.1875f64, 0.875f64],
    },
    CollisionShape {
        min: [0.25f64, 0.5625f64, 0.25f64],
        max: [0.75f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0.25f64],
        max: [0.75f64, 0.9375f64, 0.75f64],
    },
    CollisionShape {
        min: [0f64, 0.125f64, 0f64],
        max: [0.125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0.125f64, 0f64],
        max: [1f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.125f64, 0.125f64, 0.875f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.875f64, 0.125f64, 0.125f64],
        max: [1f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0f64],
        max: [0.125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0.1875f64, 0f64],
        max: [1f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.125f64, 0.1875f64, 0.875f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.875f64, 0.1875f64, 0.125f64],
        max: [1f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0.3125f64, 0f64],
        max: [0.125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0.3125f64, 0f64],
        max: [1f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.125f64, 0.3125f64, 0.875f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.875f64, 0.3125f64, 0.125f64],
        max: [1f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0.4375f64, 0f64],
        max: [0.125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0.4375f64, 0f64],
        max: [1f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.125f64, 0.4375f64, 0.875f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.875f64, 0.4375f64, 0.125f64],
        max: [1f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0.5625f64, 0f64],
        max: [0.125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0.5625f64, 0f64],
        max: [1f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.125f64, 0.5625f64, 0.875f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.875f64, 0.5625f64, 0.125f64],
        max: [1f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0.8125f64, 0f64],
        max: [0.125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0.8125f64, 0f64],
        max: [1f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.125f64, 0.8125f64, 0.875f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.875f64, 0.8125f64, 0.125f64],
        max: [1f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0.9375f64, 0f64],
        max: [0.125f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.125f64, 0.9375f64, 0f64],
        max: [1f64, 1f64, 0.125f64],
    },
    CollisionShape {
        min: [0.125f64, 0.9375f64, 0.875f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.875f64, 0.9375f64, 0.125f64],
        max: [1f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.375f64],
        max: [0.6875f64, 0.375f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.375f64],
        max: [0.625f64, 0.375f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.3125f64],
        max: [0.6875f64, 0.375f64, 0.625f64],
    },
    CollisionShape {
        min: [0.4375f64, 0.5f64, 0.4375f64],
        max: [0.5625f64, 0.875f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0.5625f64],
        max: [0.8125f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0.1875f64],
        max: [0.4375f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0f64],
        max: [0.8125f64, 0.8125f64, 0.4375f64],
    },
    CollisionShape {
        min: [0.5625f64, 0.1875f64, 0.1875f64],
        max: [1f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.8125f64, 0.4375f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.5625f64, 0.1875f64],
        max: [0.8125f64, 1f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0.6875f64],
        max: [0.8125f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0.1875f64],
        max: [0.3125f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0f64],
        max: [0.8125f64, 0.8125f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.6875f64, 0.1875f64, 0.1875f64],
        max: [1f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.8125f64, 0.3125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.6875f64, 0.1875f64],
        max: [0.8125f64, 1f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0.75f64],
        max: [0.8125f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.1875f64, 0.1875f64],
        max: [0.25f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.1875f64, 0f64],
        max: [0.8125f64, 0.8125f64, 0.25f64],
    },
    CollisionShape {
        min: [0.75f64, 0.1875f64, 0.1875f64],
        max: [1f64, 0.8125f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.8125f64, 0.25f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.75f64, 0.1875f64],
        max: [0.8125f64, 1f64, 0.8125f64],
    },
    CollisionShape {
        min: [0.25f64, 0.25f64, 0.8125f64],
        max: [0.75f64, 0.75f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.25f64, 0.25f64],
        max: [0.1875f64, 0.75f64, 0.75f64],
    },
    CollisionShape {
        min: [0.25f64, 0.25f64, 0f64],
        max: [0.75f64, 0.75f64, 0.1875f64],
    },
    CollisionShape {
        min: [0.8125f64, 0.25f64, 0.25f64],
        max: [1f64, 0.75f64, 0.75f64],
    },
    CollisionShape {
        min: [0.25f64, 0f64, 0.25f64],
        max: [0.75f64, 0.1875f64, 0.75f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.5625f64, 1f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.1875f64, 0f64, 0.1875f64],
        max: [0.5625f64, 0.6875f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.1875f64, 0.3125f64, 0.1875f64],
        max: [0.5625f64, 1f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.125f64, 0f64, 0.125f64],
        max: [0.625f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.0625f64],
        max: [0.6875f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.75f64, 1f64, 0.75f64],
    },
    CollisionShape {
        min: [0.125f64, 0.8125f64, 0.125f64],
        max: [0.875f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0.5f64, 0f64],
        max: [0.375f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.375f64, 0.5f64, 0f64],
        max: [1f64, 1f64, 0.375f64],
    },
    CollisionShape {
        min: [0.375f64, 0.5f64, 0.625f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.625f64, 0.5f64, 0.375f64],
        max: [1f64, 1f64, 0.625f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.5f64, 0.1875f64, 0.5f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.5f64, 0.1875f64, 1f64],
    },
    CollisionShape {
        min: [0.5f64, 0f64, 0.5f64],
        max: [1f64, 0.1875f64, 1f64],
    },
    CollisionShape {
        min: [0.5f64, 0f64, 0f64],
        max: [1f64, 0.1875f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.5f64],
        max: [0.5f64, 0.1875f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.5f64],
        max: [1f64, 0.1875f64, 1f64],
    },
    CollisionShape {
        min: [0.5f64, 0f64, 0f64],
        max: [1f64, 0.1875f64, 0.5f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.1875f64, 0.5f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.5f64, 0.0625f64, 0.5f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.5f64, 0.0625f64, 1f64],
    },
    CollisionShape {
        min: [0.5f64, 0f64, 0.5f64],
        max: [1f64, 0.0625f64, 1f64],
    },
    CollisionShape {
        min: [0.5f64, 0f64, 0f64],
        max: [1f64, 0.0625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.5f64],
        max: [0.5f64, 0.0625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.5f64],
        max: [1f64, 0.0625f64, 1f64],
    },
    CollisionShape {
        min: [0.5f64, 0f64, 0f64],
        max: [1f64, 0.0625f64, 0.5f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.0625f64, 0.5f64],
    },
    CollisionShape {
        min: [0f64, 0.6875f64, 0f64],
        max: [1f64, 0.9375f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.5625f64],
        max: [0.6875f64, 0.9375f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0.6875f64, 0f64],
        max: [0.3125f64, 0.9375f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.6875f64, 0f64],
        max: [1f64, 0.9375f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.6875f64, 0.9375f64],
        max: [1f64, 0.9375f64, 1f64],
    },
    CollisionShape {
        min: [0.6875f64, 0.6875f64, 0.5625f64],
        max: [1f64, 0.9375f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0.6875f64, 0f64],
        max: [1f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.5625f64],
        max: [0.6875f64, 0.8125f64, 0.9375f64],
    },
    CollisionShape {
        min: [0f64, 0.6875f64, 0f64],
        max: [0.3125f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.6875f64, 0f64],
        max: [1f64, 0.8125f64, 0.5625f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.6875f64, 0.9375f64],
        max: [1f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.6875f64, 0.6875f64, 0.5625f64],
        max: [1f64, 0.8125f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.0625f64],
        max: [0.6875f64, 0.9375f64, 0.4375f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.6875f64, 0f64],
        max: [1f64, 0.9375f64, 0.0625f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.6875f64, 0.4375f64],
        max: [1f64, 0.9375f64, 1f64],
    },
    CollisionShape {
        min: [0.6875f64, 0.6875f64, 0.0625f64],
        max: [1f64, 0.9375f64, 0.4375f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.0625f64],
        max: [0.6875f64, 0.8125f64, 0.4375f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.6875f64, 0f64],
        max: [1f64, 0.8125f64, 0.0625f64],
    },
    CollisionShape {
        min: [0.3125f64, 0.6875f64, 0.4375f64],
        max: [1f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.6875f64, 0.6875f64, 0.0625f64],
        max: [1f64, 0.8125f64, 0.4375f64],
    },
    CollisionShape {
        min: [0.5625f64, 0f64, 0.3125f64],
        max: [0.9375f64, 0.9375f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0.6875f64, 0f64],
        max: [0.5625f64, 0.9375f64, 1f64],
    },
    CollisionShape {
        min: [0.5625f64, 0.6875f64, 0f64],
        max: [1f64, 0.9375f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.5625f64, 0.6875f64, 0.6875f64],
        max: [1f64, 0.9375f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.6875f64, 0.3125f64],
        max: [1f64, 0.9375f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.5625f64, 0f64, 0.3125f64],
        max: [0.9375f64, 0.8125f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0.6875f64, 0f64],
        max: [0.5625f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.5625f64, 0.6875f64, 0f64],
        max: [1f64, 0.8125f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.5625f64, 0.6875f64, 0.6875f64],
        max: [1f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.6875f64, 0.3125f64],
        max: [1f64, 0.8125f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.3125f64],
        max: [0.4375f64, 0.9375f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0.6875f64, 0f64],
        max: [0.0625f64, 0.9375f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.6875f64, 0f64],
        max: [1f64, 0.9375f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.6875f64, 0.6875f64],
        max: [1f64, 0.9375f64, 1f64],
    },
    CollisionShape {
        min: [0.4375f64, 0.6875f64, 0.3125f64],
        max: [1f64, 0.9375f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.3125f64],
        max: [0.4375f64, 0.8125f64, 0.6875f64],
    },
    CollisionShape {
        min: [0f64, 0.6875f64, 0f64],
        max: [0.0625f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.6875f64, 0f64],
        max: [1f64, 0.8125f64, 0.3125f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.6875f64, 0.6875f64],
        max: [1f64, 0.8125f64, 1f64],
    },
    CollisionShape {
        min: [0.4375f64, 0.6875f64, 0.3125f64],
        max: [1f64, 0.8125f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.5625f64],
        max: [0.6875f64, 1f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.3125f64, 0f64, 0.0625f64],
        max: [0.6875f64, 1f64, 0.4375f64],
    },
    CollisionShape {
        min: [0.5625f64, 0f64, 0.3125f64],
        max: [0.9375f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.3125f64],
        max: [0.4375f64, 1f64, 0.6875f64],
    },
    CollisionShape {
        min: [0.125f64, 0.625f64, 0.125f64],
        max: [0.875f64, 1f64, 0.875f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.09375f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.0625f64, 0f64],
        max: [0.0625f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.0625f64, 0.9375f64],
        max: [1f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.0625f64, 0.9375f64],
        max: [1f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.625f64, 0.9375f64],
        max: [0.0625f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0.0625f64, 0f64],
        max: [1f64, 0.625f64, 0.0625f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.0625f64, 0f64],
        max: [1f64, 0.625f64, 0.0625f64],
    },
    CollisionShape {
        min: [0f64, 0.625f64, 0f64],
        max: [0.0625f64, 1f64, 0.0625f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.0625f64, 0f64],
        max: [1f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.0625f64, 0f64],
        max: [1f64, 0.625f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.0625f64, 0.0625f64],
        max: [1f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.0625f64, 0.0625f64],
        max: [1f64, 0.625f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.625f64, 0.9375f64],
        max: [1f64, 1f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0.625f64, 0f64],
        max: [1f64, 1f64, 0.0625f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [0.0625f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0.9375f64],
        max: [1f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0.9375f64],
        max: [1f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0f64, 0f64, 0f64],
        max: [1f64, 0.625f64, 0.0625f64],
    },
    CollisionShape {
        min: [0.0625f64, 0f64, 0f64],
        max: [1f64, 0.625f64, 0.0625f64],
    },
    CollisionShape {
        min: [0.9375f64, 0f64, 0f64],
        max: [1f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0f64, 0f64],
        max: [1f64, 0.625f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.9375f64, 0f64, 0.0625f64],
        max: [1f64, 0.625f64, 1f64],
    },
    CollisionShape {
        min: [0.9375f64, 0f64, 0.0625f64],
        max: [1f64, 0.625f64, 0.9375f64],
    },
    CollisionShape {
        min: [0.0625f64, 0.125f64, 0.0625f64],
        max: [0.9375f64, 1f64, 0.9375f64],
    },
];
