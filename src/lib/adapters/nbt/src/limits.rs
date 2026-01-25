//! NBT size and depth limits to prevent memory exhaustion attacks.
//!
//! These limits match Minecraft's protocol constraints and protect against
//! malicious clients sending crafted NBT data designed to exhaust server memory.

/// Maximum total size in bytes for a single NBT value (2 MB).
///
/// This matches Minecraft's internal limits and prevents memory exhaustion
/// from malicious packets claiming huge array/string lengths.
pub const MAX_NBT_SIZE: usize = 2 * 1024 * 1024;

/// Maximum nesting depth for compound/list tags (512 levels).
///
/// This prevents stack overflow from deeply nested NBT structures.
/// Minecraft uses a similar limit internally.
pub const MAX_NBT_DEPTH: usize = 512;
