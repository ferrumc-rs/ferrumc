use bevy_ecs::prelude::*;
use ferrumc_core::transform::position::Position;

/// Extension trait for bundles to set a position
pub trait SpawnBundleExt: Sized {
    /// Sets the `Position` on the bundle's `transform` component
    fn with_position(self, position: Position) -> Self;
}
