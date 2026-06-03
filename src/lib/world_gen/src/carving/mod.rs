//! Surface-carving stages.
//!
//! After the column is filled solid with stone, these stages lower the surface to its final
//! height using broad noise fields, clearing the stone above the new surface. They run in order:
//! initial height, then erosion. Each updates the shared per-column [`crate::Heightmap`] and
//! records the noise it used in [`crate::ColumnNoise`] for later biome selection.

pub mod erosion;
pub mod initial_height;
