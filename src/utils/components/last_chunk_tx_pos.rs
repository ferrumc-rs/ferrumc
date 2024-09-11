use ferrumc_macros::{Component, Constructor, Getter};

#[derive(Debug, Component, Getter, Constructor)]
pub struct LastChunkTxPos {
    pub x: i32,
    pub z: i32,
}

impl Default for LastChunkTxPos {
    fn default() -> Self {
        // The player has not moved yet.
        // So, when player joins the world, it sends chunks instantly since
        // the threshold is passed by lots.
        Self {
            x: i32::MAX,
            z: i32::MAX,
        }
    }
}

impl LastChunkTxPos {
    pub fn set_last_chunk_tx_pos(&mut self, x: i32, z: i32) {
        self.x = x;
        self.z = z;
    }

    pub fn distance_to(&self, x: i32, z: i32) -> f64 {
        let dx = self.x - x;
        let dz = self.z - z;

        ((dx * dx + dz * dz) as f64).sqrt()
    }
}
