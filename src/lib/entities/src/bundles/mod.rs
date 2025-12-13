// Entity bundles for spawning in Bevy ECS
pub mod pig;

use bevy_ecs::prelude::Component;
// Re-exports
pub use pig::PigBundle;

pub trait Entity {
    fn load() -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn save(&self) {
        unimplemented!()
    }

    fn type_marker() -> impl Component;
}
