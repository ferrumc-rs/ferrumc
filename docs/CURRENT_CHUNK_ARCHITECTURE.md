# Current Chunk Architecture (as of 2025-12-13)

## Data Structures

**Core structs (storage format)** — defined in [src/lib/world/src/chunk_format.rs](src/lib/world/src/chunk_format.rs):

```rust
pub struct Chunk {
    pub min_y: i16,
    pub sections: Vec<Section>,
    pub heightmaps: Heightmaps,
}

pub struct Section {
    pub block_states: BlockStates,
    pub biome_states: BiomeStates,
    pub block_light: Vec<u8>,
    pub sky_light: Vec<u8>,
}

pub struct BlockStates {
    pub non_air_blocks: u16,
    pub block_data: PaletteType,
    pub block_counts: HashMap<BlockStateId, i32>,
}

pub enum PaletteType {
    Single(VarInt),
    Indirect { bits_per_block: u8, data: Vec<i64>, palette: Vec<VarInt> },
    Direct { bits_per_block: u8, data: Vec<i64> },
}
```

**Palette & block storage layout**
- A `Section` always holds `block_states`, `biome_states`, and two 2048-byte light arrays (`block_light` / `sky_light`).
- `PaletteType::Single` stores one `VarInt` block ID and implies a 16×16×16 section filled with that block; `block_counts` tracks counts (default air=4096) and `non_air_blocks` stays 0 for air/void/cave air.
- `PaletteType::Indirect` bit-packs palette indices into `data: Vec<i64>`. Bits-per-entry start at 4 and grow as the palette grows (capped to protocol max 15). Packed index calculation: `blocks_per_i64 = floor(64 / bits_per_block)`; linear block index is `SectionBlockPos::pack() (0..4095)`, divided to find the `i64` slot and offset.
- `block_counts: HashMap<BlockStateId, i32>` mirrors palette usage for fast non-air counts and palette cleanup.
- `PaletteType::Direct` is declared but not implemented in set/get/optimise paths yet.

**Biome storage** — `BiomeStates` keeps `bits_per_biome`, packed `data: Vec<i64>`, and a palette (`Vec<VarInt>`). Current conversion sets `bits_per_biome = 4` and uses a single 0 entry.

**Vanilla import bridge** — [src/lib/world/src/vanilla_chunk_format.rs](src/lib/world/src/vanilla_chunk_format.rs) models NBT-loaded chunks (`VanillaChunk`, `Section`, `BlockStates`, `Biomes`) and feeds `VanillaChunk::to_custom_format` to produce the in-memory `Chunk`, translating palettes via `BLOCK2ID` and packing `Vec<i64>` data accordingly.

## State Management

**World container** — [src/lib/world/src/lib.rs](src/lib/world/src/lib.rs):

```rust
pub struct World {
    storage_backend: LmdbBackend,
    cache: Cache<(ChunkPos, String), Arc<Chunk>>,
}
```

- Storage is LMDB via `ferrumc_storage::lmdb::LmdbBackend` (one backend instance per `World`).
- In-memory cache is `moka::sync::Cache` keyed by `(ChunkPos, dimension)`, values `Arc<Chunk>`. The cache is size-weighted by `Chunk::deep_size_of`, supports TTL, and uses Moka’s internal concurrency (lock-free segments) for thread-safe reads/writes.
- Chunks travel as `Arc<Chunk>` for shared, read-only use; mutating flows clone to owned `Chunk` before save.

## Lifecycle Logic

**Initialization / creation**
- `Chunk::new(height: ChunkHeight)` builds one `Section` per Y slice, prefilled with `PaletteType::Single(air)` and `block_counts = {air:4096}`; `block_light`/`sky_light` are filled with 255. Each section immediately runs `optimise()` (no-op for `Single`).
- Converting a loaded vanilla chunk (`VanillaChunk::to_custom_format`) sets height from dimension, builds empty sections, then for each vanilla section: computes `bits_per_block = max(ceil(log2(|palette|)), 4)`, packs palette indices into `Vec<i64>`, builds `block_counts`, non-air count (subtract air/void/cave air), and copies light arrays. Missing data falls back to air.

**Loading**
- `World::load_chunk` consults cache, otherwise calls `load_chunk_internal` ([src/lib/world/src/db_functions.rs](src/lib/world/src/db_functions.rs)):
  - Key: `WyHash(dimension || 0xFF) << 96 | ChunkPos::pack()`.
  - Fetch from LMDB table `"chunks"`; stored bytes are `bitcode`-encoded `Chunk` compressed with `yazi` (zlib, `BestSpeed`).
  - Optional checksum verification (Adler32) if enabled in config.
  - On success, decoded `Chunk` is wrapped in `Arc` and inserted into cache.
  - `load_chunk_batch` parallels the same logic for multiple positions.

**Modification**
- High-level entry: `World::set_block_and_fetch` → `load_chunk_owned` (clone of cached `Arc`) → `Chunk::set_block` → `Section::set_block`:
  - If the section is in `Single`, it is converted to `Indirect { bits_per_block:4, data: vec![0;256], palette:[existing] }`.
  - Palette lookup/add: finds or appends `block.to_varint()`; computes `required_bits = clamp(ceil(log2(palette.len())), 4..15)`. If `required_bits` exceeds current `bits_per_block`, `BlockStates::resize` rewrites the packed `data` with the new bit width.
  - Packed write: calculates `i64_index` and `offset` from `SectionBlockPos::pack()`; writes palette index via `write_nbit_u32` into the `Vec<i64>` slot.
  - Block counts: decrements old block, increments new, and recomputes `non_air_blocks` from counts.
  - Post-step `Section::optimise` removes zero-count palette entries and collapses back to `Single` if only one palette entry remains.
  - `World::save_chunk` persists: compress/encode and `upsert` into LMDB, then replaces cache entry with the new `Arc`.
- Batch editing: `EditBatch` ([src/lib/world/src/edit_batch.rs](src/lib/world/src/edit_batch.rs)) groups many `ChunkBlockPos` edits. It pre-slices edits by section, ensures `PaletteType::Indirect`, reuses/extends the palette (`AHashMap` lookup), updates packed data with `write_nbit_u32`, adjusts counts, and only calls `optimise` when palette hash changes. Intended for bulk edits to reduce repeated palette recalculations.

**Reading**
- `World::get_block_and_fetch` → cached or loaded `Chunk` → `Section::get_block`: if `Single`, returns the stored `VarInt`; if `Indirect`, computes `i64_index`/`offset`, reads palette index via `read_nbit_u32`, and returns the palette entry. `Direct` is still unimplemented.

**Unloading / freeing**
- Explicit removal: `World::delete_chunk` evicts the cache entry and deletes from LMDB. `sync` flushes LMDB.
- Implicit: cache TTL/size eviction (Moka) drops `Arc<Chunk>` references; memory is reclaimed when no other `Arc` remains. There is no dedicated section/array free beyond normal Rust drop.

## Dependencies (chunk-related only)
- External crates: `bitcode` (+ `bitcode_derive`) for serialization; `yazi` for compression/checksum; `moka::sync::Cache` for concurrent caching; `ferrumc_storage::lmdb::LmdbBackend` for persistence; `ferrumc_general_purpose::data_packing::{u32,i32}` for bit packing; `ferrumc_net_codec::net_types::var_int::VarInt`; `deepsize` for cache weighting; `ahash` (EditBatch temporary maps).
- Internal modules: `block_state_id` (ID ↔ palette mapping), `pos` (ChunkPos/SectionBlockPos packing), `errors::WorldError`, `vanilla_chunk_format` (NBT import), `edit_batch`/`edits` (mutation helpers), `db_functions` (IO), `chunk_format` (core data model).

## Notes / Gaps to address in refactor
- `PaletteType::Direct` is declared but unused; set/get/optimise paths `todo!()`.
- Biome palette packing currently hardcoded to a single-entry palette with 4 bits; real biome storage is incomplete.
- `Section::optimise` recomputes `non_air_blocks` only via counts; incorrect counts will skew lighting/heightmap assumptions.
- Light arrays are stored but not mutated in the edit paths; any refactor should clarify ownership and lazy updates.
