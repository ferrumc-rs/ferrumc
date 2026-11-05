use bevy_ecs::prelude::Component;
use ferrumc_data::generated::entities::EntityType as VanillaEntityType;

use super::metadata::EntityMetadata;

/// Combat properties for an entity.
///
/// # Examples
///
/// ```ignore
/// use ferrumc_entities::components::{EntityMetadata, CombatProperties};
/// use ferrumc_data::generated::entities::EntityType as VanillaEntityType;
///
/// let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::PIG);
/// let mut combat = CombatProperties::from_metadata(&metadata);
///
/// assert!(combat.attackable);
/// assert_eq!(combat.invulnerability_ticks, 0);
///
/// // After a hit
/// combat.set_invulnerable(10); // 10 ticks of invulnerability
/// assert!(!combat.can_be_damaged());
/// ```
#[derive(Component, Clone, Copy)]
pub struct CombatProperties {
    /// True if an entity is attackable
    ///
    /// Some entity (area_effect_cloud, markers) can not be attackable.
    /// This value come from vanilla data and normally do not change.
    pub attackable: bool,

    /// Number of invulnerability_ticks left
    ///
    /// When an entity take damage, she become invincible for a short time
    /// (generally 10 ticks = 0.5 seconds) to preserve entity from multiple
    /// hits
    ///
    /// This count is decremented every tick and the entity can't be damaged
    /// while (count > 0)
    pub invulnerability_ticks: u32,
}

impl CombatProperties {
    /// Standard invulnerability duration after a hit in ticks.
    ///
    /// In vanilla Minecraft, it's 10 ticks (0.5 seconds).
    pub const DEFAULT_INVULNERABILITY_TICKS: u32 = 10;

    /// Creates combat properties from vanilla metadatas.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let metadata = EntityMetadata::from_vanilla(&VanillaEntityType::PIG);
    /// let combat = CombatProperties::from_metadata(&metadata);
    /// assert!(combat.attackable);
    /// ```
    pub fn from_metadata(metadata: &EntityMetadata) -> Self {
        Self {
            attackable: metadata.vanilla_data().attackable.unwrap_or(false),
            invulnerability_ticks: 0,
        }
    }

    /// Create directly from vanilla datas.
    pub fn from_vanilla(data: &'static VanillaEntityType) -> Self {
        Self {
            attackable: data.attackable.unwrap_or(false),
            invulnerability_ticks: 0,
        }
    }

    /// Return true if the entity can't be damaged.
    pub const fn can_be_damaged(&self) -> bool {
        self.attackable && self.invulnerability_ticks == 0
    }

    /// Activate invulnerability for a certain amount of ticks.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut combat = CombatProperties::from_vanilla(&VanillaEntityType::PIG);
    /// combat.set_invulnerable(10);
    /// assert!(!combat.can_be_damaged());
    /// ```
    pub fn set_invulnerable(&mut self, ticks: u32) {
        self.invulnerability_ticks = ticks;
    }

    /// Activate default duration invulnerability
    pub fn set_default_invulnerability(&mut self) {
        self.set_invulnerable(Self::DEFAULT_INVULNERABILITY_TICKS);
    }

    /// Decream invulnerability count (call every ticks)
    ///
    /// # Return
    ///
    /// True if the entity was invulnerable and is no longer.
    pub fn tick(&mut self) -> bool {
        if self.invulnerability_ticks > 0 {
            self.invulnerability_ticks -= 1;
            self.invulnerability_ticks == 0
        } else {
            false
        }
    }

    /// Remove immediatly invulnerability
    pub fn clear_invulnerability(&mut self) {
        self.invulnerability_ticks = 0;
    }
}

impl std::fmt::Debug for CombatProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CombatProperties")
            .field("attackable", &self.attackable)
            .field("invulnerability_ticks", &self.invulnerability_ticks)
            .field("can_be_damaged", &self.can_be_damaged())
            .finish()
    }
}

/// Marks an entity that has died and is playing out its death animation before being removed.
///
/// On death the server broadcasts the death animation and tags the entity with this component
/// instead of removing it immediately, so the client has time to render the animation. The tick
/// counter is decremented every tick and the entity is despawned once it reaches zero.
#[derive(Component, Clone, Copy, Debug)]
pub struct Dying {
    /// Ticks left before the entity is removed.
    pub ticks_remaining: u32,
}

impl Dying {
    /// Length of the death animation in ticks, matching the vanilla ~1 second window.
    pub const DEATH_ANIMATION_TICKS: u32 = 20;
}

impl Default for Dying {
    fn default() -> Self {
        Self {
            ticks_remaining: Self::DEATH_ANIMATION_TICKS,
        }
    }
}
