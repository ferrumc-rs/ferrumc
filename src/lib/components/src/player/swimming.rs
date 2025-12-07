use bevy_ecs::prelude::Component;

/// Component tracking whether a player is currently swimming
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SwimmingState {
    pub is_swimming: bool,
}

impl SwimmingState {
    pub fn new(is_swimming: bool) -> Self {
        Self { is_swimming }
    }

    pub fn start_swimming(&mut self) {
        self.is_swimming = true;
    }

    pub fn stop_swimming(&mut self) {
        self.is_swimming = false;
    }
}
