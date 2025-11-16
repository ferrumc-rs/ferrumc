use dashmap::DashMap;
use ferrumc_components::active_effects::ActiveEffects;
use ferrumc_components::health::Health;
use ferrumc_components::player::abilities::PlayerAbilities;
use ferrumc_components::player::experience::Experience;
use ferrumc_components::player::gamemode::GameMode;
use ferrumc_components::player::gameplay_state::ender_chest::EnderChest;
use ferrumc_components::player::hunger::Hunger;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_inventories::inventory::Inventory;
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
