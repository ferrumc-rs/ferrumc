use bevy_ecs::prelude::Component;
use typename::TypeName;

/// age of an entity in ticks (1 tick = 50ms at 20 TPS)
#[derive(Debug, Clone, Component, TypeName, Default)]
pub struct Age(pub u64);

impl Age {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn tick(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    pub fn as_seconds(&self) -> f64 {
        self.0 as f64 / 20.0
    }
}
