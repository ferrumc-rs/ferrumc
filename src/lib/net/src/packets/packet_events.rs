use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::Event;

#[derive(Event)]
pub struct TransformEvent {
    pub conn_id: usize,
    pub position: Option<Position>,
    pub rotation: Option<Rotation>,
    pub on_ground: Option<bool>,
}

impl TransformEvent {
    pub fn new(conn_id: usize) -> Self {
        Self {
            conn_id,
            position: None,
            rotation: None,
            on_ground: None,
        }
    }
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
    
    pub fn rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = Some(rotation);
        self
    }
    
    pub fn on_ground(mut self, on_ground: bool) -> Self {
        self.on_ground = Some(on_ground);
        self
    }
}