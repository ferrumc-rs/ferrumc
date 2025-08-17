use crate::inventory::Inventory;

mod player;

pub enum DisplayType {
    Player,
}

impl Inventory {
    pub fn display(&self, display_type: DisplayType) {
        match display_type {
            DisplayType::Player => player::display_player(self),
        }
    }
}
