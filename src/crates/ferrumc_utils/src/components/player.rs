use ferrumc_macros::{Component, Constructor};

#[derive(Component, Constructor)]
pub struct Player {
    pub uuid: u128,
    pub username: String,
}