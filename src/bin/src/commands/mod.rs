//! Binary-local commands (those that need ECS resources defined in the binary crate, such as the
//! fluid scheduler/control, which the shared `ferrumc-default-commands` crate cannot name).
//!
//! Commands register themselves at startup via the `#[command]` macro's `#[ctor]`. [`init`] only
//! exists to guarantee this module is linked into the final binary so those constructors run.

pub mod tick;

/// Ensures the binary-local command modules are linked so their `#[ctor]` registrations fire.
pub fn init() {}
