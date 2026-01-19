use crate::active_effects::ActiveEffects;
use crate::health::Health;
use crate::player::abilities::PlayerAbilities;
use crate::player::experience::Experience;
use crate::player::gamemode::GameMode;
use crate::player::gameplay_state::ender_chest::EnderChest;
use crate::player::hunger::Hunger;
use bitcode_derive::{Decode, Encode};
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_inventories::inventory::Inventory;

#[derive(Clone, Debug, Encode, Decode, Default)]
pub struct OfflinePlayerData {
    pub abilities: PlayerAbilities,
    pub gamemode: GameMode,
    pub position: (f64, f64, f64),
    pub rotation: Rotation,
    pub inventory: Inventory,
    pub health: Health,
    pub hunger: Hunger,
    pub experience: Experience,
    pub ender_chest: EnderChest,
    pub active_effects: ActiveEffects,
}
