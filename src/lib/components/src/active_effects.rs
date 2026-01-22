use bevy_ecs::prelude::Component;
use bitcode_derive::{Decode, Encode};
use std::collections::HashMap;

// --- Placeholders ---
// TODO: fill this out in some way
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Decode, Encode)]
pub enum EffectType {
    Speed,
    Poison,
    Regeneration,
}

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct EffectState {
    pub amplifier: u8,
    /// Duration in server ticks
    pub duration_ticks: u32,
}

/// Tracks all active potion effects on the player.
#[derive(Component, Debug, Clone, Default, Decode, Encode)]
pub struct ActiveEffects {
    pub effects: HashMap<EffectType, EffectState>,
}
