use bevy_ecs::prelude::*;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::bundles::ZombieBundle;
use ferrumc_entities::events::SpawnZombieEvent;

pub fn handle_spawn_zombie(
    mut commands: Commands,
    mut ev_zombie: EventReader<SpawnZombieEvent>,
) {
    for ev in ev_zombie.read() {
        commands.spawn(ZombieBundle {
            transform: ZombieBundle::default().transform, // copy defaults
            ..ZombieBundle::default()                     // other defaults
        }.with_position(ev.position.clone()));                    // see builder below
    }
}

// quick builder helper
trait WithPosition {
    fn with_position(self, pos: Position) -> Self;
}
impl WithPosition for ZombieBundle {
    fn with_position(mut self, pos: Position) -> Self {
        self.transform.position = pos;
        self
    }
}