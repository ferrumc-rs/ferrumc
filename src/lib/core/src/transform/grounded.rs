use bevy_ecs::prelude::Component;
use typename::TypeName;

#[derive(TypeName, Debug, Default, Component, Copy, Clone)]
pub struct OnGround(pub bool);

impl From<bool> for OnGround {
    fn from(on_ground: bool) -> Self {
        Self(on_ground)
    }
}

impl From<OnGround> for bool {
    fn from(on_ground: OnGround) -> Self {
        on_ground.0
    }
}
