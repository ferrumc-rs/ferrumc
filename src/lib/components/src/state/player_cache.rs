use crate::inventory::Inventory;
use crate::player::abilities::PlayerAbilities;
use crate::player::experience::Experience;
use crate::player::gamemode::GameModeComponent as GameMode;
use crate::player::gameplay_mechanics::active_effects::ActiveEffects;
use crate::player::gameplay_mechanics::ender_chest::EnderChest;
use crate::player::health::Health;
use crate::player::hunger::Hunger;
use crate::player::transform::position::Position;
use crate::player::transform::rotation::Rotation;

use dashmap::DashMap;
use uuid::Uuid;

/// A struct to hold all component data for an offline player.
#[derive(Clone, Debug)]
pub struct OfflinePlayerData {
    pub abilities: PlayerAbilities,
    pub gamemode: GameMode,
    pub position: Position,
    pub rotation: Rotation,
    pub inventory: Inventory,
    pub health: Health,
    pub hunger: Hunger,
    pub experience: Experience,
    pub ender_chest: EnderChest,
    pub active_effects: ActiveEffects,
}

/// The generic struct that holds all offline player data
#[derive(Debug, Default)]
pub struct PlayerCache {
    pub cache: DashMap<Uuid, OfflinePlayerData>,
}

// Helper methods
impl PlayerCache {
    pub fn get_and_remove(&self, uuid: &Uuid) -> Option<OfflinePlayerData> {
        self.cache.remove(uuid).map(|(_uuid, data)| data)
    }

    pub fn get(
        &self,
        uuid: &Uuid,
    ) -> Option<impl std::ops::Deref<Target = OfflinePlayerData> + '_> {
        self.cache.get(uuid)
    }

    pub fn insert(&self, uuid: Uuid, data: OfflinePlayerData) {
        self.cache.insert(uuid, data);
    }
}
