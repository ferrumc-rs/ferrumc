use ferrumc_macros::{Component, Constructor};

#[derive(Component, Constructor, Debug)]
pub struct Player {
    pub uuid: u128,
    pub username: String,
}

impl Player {
    pub fn get_uuid(&self) -> u128 {
        self.uuid
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
}