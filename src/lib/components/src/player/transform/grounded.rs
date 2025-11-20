use bevy_ecs::prelude::Component;
use std::ops::{Deref, DerefMut};
use typename::TypeName;

#[derive(TypeName, Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OnGround(pub bool);

impl OnGround {
    pub fn new(is_grounded: bool) -> Self {
        Self(is_grounded)
    }
}

/// Standard Deref to bool
impl Deref for OnGround {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OnGround {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Allow clean conversion from bool
impl From<bool> for OnGround {
    fn from(b: bool) -> Self {
        Self(b)
    }
}
