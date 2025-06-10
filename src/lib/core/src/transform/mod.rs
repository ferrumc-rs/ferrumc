use bevy_ecs::prelude::Bundle;

pub mod grounded;
pub mod position;
pub mod rotation;

#[derive(Bundle)]
pub struct Transform {
    pub position: position::Position,
    pub rotation: rotation::Rotation,
    pub grounded: grounded::OnGround,
}
impl Transform {
    pub fn new(
        position: impl Into<position::Position>,
        rotation: impl Into<rotation::Rotation>,
    ) -> Self {
        Transform {
            position: position.into(),
            rotation: rotation.into(),
            grounded: grounded::OnGround::default(),
        }
    }
}
