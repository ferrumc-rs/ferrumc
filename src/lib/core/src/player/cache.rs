use super::abilities::PlayerAbilities;

/// A struct to hold all compnent data for an offline player.
#[derive(Clone, Debug)]
pub struct OfflinePlayerData {
    pub abilities: PlayerAbilities,
    // TODO: add other components to cache here
}

impl OfflinePlayerData {
    /// Creates a new OfflinePlayerData bundle from a set of components.
    pub fn new(abilities: PlayerAbilities) -> Self {
        Self { abilities }
    }
}
