// Entity bundles for spawning in Bevy ECS
// Organized by behavior category

pub mod hostile;
pub mod neutral;
pub mod passive;

// Re-export all bundles for convenience
pub use hostile::*;
pub use neutral::*;
pub use passive::*;
