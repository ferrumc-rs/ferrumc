//! Block Interaction System
//!
//! This module handles direct block interactions in the world (chunks).
//! For simple toggleable blocks (doors, levers, trapdoors, buttons),
//! the system modifies block states directly without creating ECS entities.
//!
//! ## How it works
//!
//! 1. Player right-clicks on a block (PlaceBlock packet)
//! 2. `try_interact()` checks if the block is interactive
//! 3. If yes, toggles the relevant property and returns the new state
//! 4. The packet handler updates the chunk and broadcasts to players

pub mod block_interactions;
