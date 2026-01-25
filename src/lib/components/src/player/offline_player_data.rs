use crate::active_effects::ActiveEffects;
use crate::health::Health;
use crate::player::abilities::PlayerAbilities;
use crate::player::experience::Experience;
use crate::player::gamemode::GameMode;
use crate::player::gameplay_state::ender_chest::{EnderChest, StorageEnderChest};
use crate::player::hunger::Hunger;
use bitcode_derive::{Decode, Encode};
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_inventories::StorageInventory;

/// Runtime player data (ECS-friendly, not directly persistable).
#[derive(Clone, Debug, Default)]
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

/// Storage-friendly player data for bitcode persistence.
#[derive(Clone, Debug, Encode, Decode)]
pub struct StorageOfflinePlayerData {
    pub abilities: PlayerAbilities,
    pub gamemode: GameMode,
    pub position: (f64, f64, f64),
    pub rotation: Rotation,
    pub inventory: StorageInventory,
    pub health: Health,
    pub hunger: Hunger,
    pub experience: Experience,
    pub ender_chest: StorageEnderChest,
    pub active_effects: ActiveEffects,
}

impl From<&OfflinePlayerData> for StorageOfflinePlayerData {
    fn from(data: &OfflinePlayerData) -> Self {
        Self {
            abilities: data.abilities,
            gamemode: data.gamemode,
            position: data.position,
            rotation: data.rotation,
            inventory: StorageInventory::from(&data.inventory),
            health: data.health,
            hunger: data.hunger,
            experience: data.experience,
            ender_chest: StorageEnderChest::from(&data.ender_chest),
            active_effects: data.active_effects.clone(),
        }
    }
}

impl From<StorageOfflinePlayerData> for OfflinePlayerData {
    fn from(storage: StorageOfflinePlayerData) -> Self {
        Self {
            abilities: storage.abilities,
            gamemode: storage.gamemode,
            position: storage.position,
            rotation: storage.rotation,
            inventory: Inventory::from(storage.inventory),
            health: storage.health,
            hunger: storage.hunger,
            experience: storage.experience,
            ender_chest: EnderChest::from(storage.ender_chest),
            active_effects: storage.active_effects,
        }
    }
}

impl Default for StorageOfflinePlayerData {
    fn default() -> Self {
        StorageOfflinePlayerData::from(&OfflinePlayerData::default())
    }
}
