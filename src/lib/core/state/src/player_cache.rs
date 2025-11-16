use dashmap::DashMap;
use ferrumc_components::player::abilities::PlayerAbilities;
use ferrumc_components::player::gamemode::{GameMode, GameModeComponent};
use uuid::Uuid;

/// A struct to hold all component data for an offline player.
#[derive(Clone, Debug)]
pub struct OfflinePlayerData {
    pub abilities: PlayerAbilities,
    pub gamemode: GameMode,
    // TODO: Add other necessary components to cache here like gamemode
}

impl OfflinePlayerData {
    /// Creates a new OfflinePlayerData bundle from a set of components
    pub fn new(abilities: PlayerAbilities, gamemode: GameModeComponent) -> Self {
        Self {
            abilities,
            gamemode: gamemode.0, // Store the inner enum
        }
    }
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
