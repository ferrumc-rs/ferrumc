//! Surface-carving stage.
//!
//! After the column is filled solid with stone, this stage lowers the surface to its final height
//! and clears the stone above it. The final height is composed from three noise fields:
//!
//! * the climate **continentalness** field, which sets an absolute base height (deep ocean basin →
//!   raised inland) — see [`crate::climate`];
//! * a higher-frequency **local detail** field that adds hills and texture, with its amplitude
//!   scaled by both continentalness (oceans stay flat, inland gets hilly) and erosion;
//! * the **erosion** field, which damps the detail amplitude so high-erosion regions flatten out.
//!
//! The composition is a pure function of the world seed and global coordinates (no cross-column
//! interaction), so any column can be evaluated in isolation — which the cross-chunk tree overscan
//! relies on. The single [`crate::WorldGenerator::carve_surface`] pass clears the stone above each
//! column once, recording the surface height and the climate sample used for biome selection.

pub mod erosion;
pub mod initial_height;
