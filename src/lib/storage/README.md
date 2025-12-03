# FerrumC Storage

**Layer:** 4 (Infrastructure)  
**Dependencies:** `ferrumc-core`, `ferrumc-utils`, `ferrumc-nbt`

## Purpose

This crate manages the **Persistence** of the Minecraft world. It is responsible for saving `Chunk` structs to disk and loading them back into memory efficiently.

It uses **LMDB** (Lightning Memory-Mapped Database) for high-performance, concurrent data access.

## Module Structure

### `db/` (The Backend)
Handles the raw bytes.
* **`lmdb.rs`**: Wraps the underlying C-library/crate for LMDB.
* **`ops.rs`**: Handles serialization (Bitcode), compression (Zlib), and key generation (Coordinate packing).

### `chunk/` (The Interface)
Handles the high-level `Chunk` objects defined in `ferrumc-core`.
* **`api.rs`**: Implements the `ChunkStorage` struct methods (`get_chunk`, `save_chunk`).
* **`edits.rs`**: Implements direct block modification logic (`chunk.set_block`), which triggers auto-saving.
* **`batch.rs`**: Optimized bulk-editing tools.

### `import/` (The Migration Tool)
Handles converting legacy Minecraft worlds (Anvil/MCA format) into FerrumC's format.
* **`anvil.rs`**: Reads `.mca` files and iterates over chunks.
* **`legacy.rs`**: Defines the NBT structure of Vanilla chunks (1.21+).

## Usage

### Initialization (Bevy Plugin)
This crate exports `StoragePlugin`. When added to the App, it initializes the database at the path specified in the config and inserts `GlobalChunkStorage` as a Resource.

### Accessing Data
Systems should access storage via the `ChunkSource` trait (from `core`) or the `ChunkStorage` struct directly if inside the infrastructure layer.

```rust
// Inside a system
fn my_system(storage: Res<GlobalChunkStorage>) {
    if let Ok(chunk) = storage.0.get_chunk(0, 0) { // x, z
        println!("Loaded chunk!");
    }
}