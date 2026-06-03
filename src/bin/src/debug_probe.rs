//! TEMPORARY single-point debug probe.
//!
//! Traces every server-side event that touches one specific world position so the
//! placement-flicker / section-disappearance investigation produces a tiny, focused log
//! instead of drowning in output.
//!
//! To silence without removing call sites: set [`PROBE_POS`] to `None`.
//! To remove entirely: delete this file, the `mod debug_probe;` line in `main.rs`, and the
//! `debug_probe::` call sites (grep for `debug_probe`).

use ferrumc_world::pos::BlockPos;
use tracing::info;

/// The single world position to trace, as `(x, y, z)`. `None` disables all probe output.
pub const PROBE_POS: Option<(i32, i32, i32)> = Some((26, 64, 40));

/// True if `pos` is the traced position.
#[inline]
pub fn is_probe(pos: BlockPos) -> bool {
    matches!(PROBE_POS, Some((x, y, z)) if pos.pos.x == x && pos.pos.y == y && pos.pos.z == z)
}

/// True if `pos` lies in the same chunk column as the traced position (used to catch full-chunk
/// (re)sends that would carry the probe's section).
#[inline]
pub fn is_probe_chunk(chunk_x: i32, chunk_z: i32) -> bool {
    match PROBE_POS {
        Some((x, _, z)) => x.div_euclid(16) == chunk_x && z.div_euclid(16) == chunk_z,
        None => false,
    }
}

/// Logs a tagged event for the traced position. `tag` names the call site; `msg` carries detail.
#[inline]
pub fn log(tag: &str, pos: BlockPos, msg: impl AsRef<str>) {
    if is_probe(pos) {
        let section = (pos.pos.y + 64).div_euclid(16);
        info!(
            target: "probe",
            "[PROBE {tag}] pos=({},{},{}) section_idx={} | {}",
            pos.pos.x,
            pos.pos.y,
            pos.pos.z,
            section,
            msg.as_ref()
        );
    }
}

/// Logs a chunk-level event (e.g. a full chunk send) when it covers the probe's column.
#[inline]
pub fn log_chunk(tag: &str, chunk_x: i32, chunk_z: i32, msg: impl AsRef<str>) {
    if is_probe_chunk(chunk_x, chunk_z) {
        info!(
            target: "probe",
            "[PROBE {tag}] chunk=({chunk_x},{chunk_z}) | {}",
            msg.as_ref()
        );
    }
}
