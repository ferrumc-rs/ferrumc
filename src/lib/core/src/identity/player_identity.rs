#[derive(Debug)]
pub struct PlayerIdentity {
    pub username: String,
    pub uuid: u128,
    pub failed_keep_alive: bool,
}

impl PlayerIdentity {
    pub fn new(username: String, uuid: u128, failed_keep_alive: bool) -> Self {
        Self {
            username,
            uuid,
            failed_keep_alive,
        }
    }
}
