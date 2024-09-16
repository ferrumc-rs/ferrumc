use ferrumc_macros::{Component, Constructor, Getter};

#[derive(Debug, Default, Component, Getter, Constructor)]
pub struct Grounded {
    pub is_grounded: bool,
}


impl Grounded {
    pub fn set_grounded(&mut self, is_grounded: bool) {
        self.is_grounded = is_grounded;
    }

    pub fn flip_grounded(&mut self) {
        self.is_grounded = !self.is_grounded;
    }
}
