use bevy_ecs::prelude::Component;
use std::collections::HashMap;

// --- Placeholders ---
// TODO: fill this out in some way
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectType {
    Speed,
    Poison,
    Regeneration,
}

#[derive(Debug, Clone, Copy)]
pub struct EffectState {
    pub amplifier: u8,
    /// Duration in server ticks
    pub duration_ticks: u32,
}

/// Tracks all active potion effects on the player.
#[derive(Component, Debug, Clone, Default)]
pub struct ActiveEffects {
    pub effects: HashMap<EffectType, EffectState>,
}
