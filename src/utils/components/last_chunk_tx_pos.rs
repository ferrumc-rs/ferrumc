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
            x: 0,
            z: 0,
        }
    }
}

impl LastChunkTxPos {
    pub fn set_last_chunk_tx_pos(&mut self, x: i32, z: i32) {
        self.x = x;
        self.z = z;
    }

    pub fn distance_to(&self, x: i32, z: i32) -> f64 {
        let dx = (self.x - x) as f64;
        let dz = (self.z - z) as f64;

        (dx * dx + dz * dz).sqrt()
    }
}
