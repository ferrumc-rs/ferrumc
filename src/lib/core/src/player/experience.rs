use serde::{Deserialize, Serialize};

/// Pure data representation of a player's experience.
/// Matches the fields required by the `Set Experience` packet.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExperienceData {
    /// The progress on the experience bar (between 0.0 and 1.0).
    pub bar_progress: f32,
    /// The player's current level.
    pub level: i32,
    /// The total experience points the player has collected.
    pub total_xp: i32,
}

impl Default for ExperienceData {
    fn default() -> Self {
        Self {
            bar_progress: 0.0,
            level: 0,
            total_xp: 0,
        }
    }
}

impl ExperienceData {
    pub fn new(bar_progress: f32, level: i32, total_xp: i32) -> Self {
        Self {
            bar_progress,
            level,
            total_xp,
        }
    }

    /// Adds experience points to the player, handling leveling up automatically.
    /// Returns `true` if the level changed (useful for playing the "ding" sound).
    pub fn add_xp(&mut self, amount: i32) -> bool {
        if amount <= 0 {
            return false;
        }

        self.total_xp += amount;
        let start_level = self.level;

        let xp_to_add = amount as f32;
        let mut xp_for_next = self.xp_needed_for_next_level(self.level) as f32;

        // Calculate current raw XP on the bar
        let mut current_bar_xp = self.bar_progress * xp_for_next;

        current_bar_xp += xp_to_add;

        // While we have enough XP to level up...
        while current_bar_xp >= xp_for_next {
            current_bar_xp -= xp_for_next;
            self.level += 1;
            xp_for_next = self.xp_needed_for_next_level(self.level) as f32;
        }

        // Update progress bar
        self.bar_progress = current_bar_xp / xp_for_next;

        self.level > start_level
    }

    /// Calculates the amount of XP needed to pass the *current* level `lvl`.
    ///
    /// Formulas based on Minecraft Wiki:
    /// - Level 0-15:  2*L + 7
    /// - Level 16-30: 5*L - 38
    /// - Level 31+:   9*L - 158
    pub fn xp_needed_for_next_level(&self, level: i32) -> i32 {
        if level >= 31 {
            9 * level - 158
        } else if level >= 16 {
            5 * level - 38
        } else {
            2 * level + 7
        }
    }

    /// Sets the level directly and resets the bar progress.
    pub fn set_level(&mut self, level: i32) {
        self.level = level;
        self.bar_progress = 0.0;
        // Note: Recalculating `total_xp` from level is complex/lossy
        // because `total_xp` includes the partial bar, but we can approximate
        // or just leave it as is if we only care about the level.
    }
}
