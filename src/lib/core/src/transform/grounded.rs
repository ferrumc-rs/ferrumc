#[derive(Debug, Default)]
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
