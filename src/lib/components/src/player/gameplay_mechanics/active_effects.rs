use bevy_ecs::prelude::Component;
use ferrumc_core::player::gameplay_mechanics::effects::{EffectData, EffectType};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

/// Tracks all active potion effects on the player.
#[derive(Component, Debug, Clone, Default)]
pub struct ActiveEffects(pub HashMap<EffectType, EffectData>);

impl ActiveEffects {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Helper to add an effect cleanly
    pub fn add(&mut self, effect_type: EffectType, amplifier: u8, duration_ticks: u32) {
        self.0.insert(
            effect_type,
            EffectData {
                amplifier,
                duration: duration_ticks,
                show_particles: true, // Defaults
                show_icon: true,
            },
        );
    }
}

impl Deref for ActiveEffects {
    type Target = HashMap<EffectType, EffectData>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ActiveEffects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
