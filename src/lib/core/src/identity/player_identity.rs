#[derive(Debug)]
pub struct PlayerIdentity {
    pub username: String,
    pub uuid: u128,
    pub connection_id: usize,
}

impl PlayerIdentity {
    pub fn new(username: String, uuid: u128, connection_id: usize) -> Self {
        Self {
            username,
            uuid,
            connection_id,
        }
    }
}
