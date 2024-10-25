#[derive(Debug)]
pub struct PlayerIdentity {
    pub username: String,
    pub uuid: u128,
}

impl PlayerIdentity {
    pub fn new(username: String, uuid: u128) -> Self {
        Self { username, uuid }
    }
}