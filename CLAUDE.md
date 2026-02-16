# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

FerrumC is a high-performance Minecraft 1.21.8 (protocol 772) server implementation written in Rust. Not a framework (unlike Valence/Minestom) — this is a full vanilla server replacement prioritizing speed and memory efficiency over 1:1 vanilla parity. Uses Bevy ECS for game logic, Tokio for async networking, and LMDB for chunk storage.

## Build & Development Commands

```bash
# Build (requires Rust nightly)
cargo build                          # debug build
cargo build --release                # release build
cargo build --profile production      # release + LTO, stripped, single codegen unit (used by CI releases)
cargo build --profile hyper          # max optimization (LTO, stripped, abort on panic)
cargo build --profile profiling      # release + debug symbols
cargo build --features dashboard     # include web dashboard

# Run server
cargo run --release                  # defaults to "run" subcommand
cargo run --release -- setup         # generate config.toml
cargo run --release -- import --import-path=/path/to/world  # import vanilla world
cargo run --release -- clear         # clear world data
cargo run --release -- --log=info    # custom log level (trace/debug/info/warn/error)

# Tests (CI uses cargo-nextest)
cargo nextest run --all-targets --all-features -E "not kind(bench)"
cargo nextest run -p ferrumc-nbt     # single crate
cargo test -p ferrumc-core           # alternative without nextest

# Lints (all enforced in CI)
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo audit --ignore RUSTSEC-2023-0071

# Profiling with Tracy
cargo run --release --features tracy
```

## Architecture

### Workspace Structure

Monorepo with ~30 crates under `src/`. All dependencies are declared at workspace level in the root `Cargo.toml` and referenced by member crates.

- **`src/bin`** — Main binary. CLI parsing, server launch, game loop, packet handlers, ECS system registration.
- **`src/lib/core`** — Core primitives (chunks, connections, identity, transforms). `ferrumc-state` holds the global `ServerState` shared via `Arc`.
- **`src/lib/net`** — Network layer with custom Minecraft protocol implementation. Sub-crates: `codec` (packet encoding/decoding), `encryption` (AES-128-CFB8 + RSA).
- **`src/lib/adapters/nbt`** — Custom NBT parser, hand-crafted for performance.
- **`src/lib/adapters/anvil`** — Custom Anvil chunk format reader using memory-mapped I/O (`memmap2`) and `yazi` compression.
- **`src/lib/storage`** — Persistent KV storage via `heed` (LMDB-based).
- **`src/lib/world`** / **`world_gen`** — World management (chunk cache: `DashMap` with `WyHash`) and terrain generation.
- **`src/lib/scheduler`** — Timed schedule system for the game loop.
- **`src/lib/components`** / **`entities`** — Bevy ECS components (Health, Gamemode, Abilities) and entity definitions.
- **`src/lib/registry`** — Block/item registry using perfect hashing (`phf`) with build-time codegen.
- **`src/lib/commands`** / **`default_commands`** — Trait-based command system.
- **`src/lib/messages`** — Inter-system event messages for ECS.
- **`src/lib/dashboard`** — Optional web dashboard (Axum, feature-gated behind `dashboard`).
- **`src/lib/derive_macros`** — Procedural macros (e.g., `#[packet(...)]`).
- **`src/lib/utils/`** — Logging (tracing + Tracy), profiling, thread pool, general-purpose utilities.
- **`src/tests`** — Integration tests (NBT, codec, protocol).

### Dependency Hierarchy

```
ferrumc-core → ferrumc-components → ferrumc-net → ferrumc (bin)
```

### Game Loop (`src/bin/src/game_loop.rs`)

Timed schedules via Bevy ECS, managed by a custom `Scheduler`:

- **tick** (configurable TPS, default 20): Main game logic — packet handling, player updates, commands, physics, mob AI. Uses `MissedTickBehavior::Burst` (catch up to 5 missed ticks).
- **world_sync** (15s): Persist world to disk. Skips if missed.
- **chunk_gc** (5s): Unload unused chunks from memory. Skips if missed.
- **keepalive** (1s, 250ms phase offset): Connection keepalives and ping updates. Skips if missed.

### Networking Flow

TCP connections → dedicated Tokio thread (single-threaded runtime) → `handle_connection` per client → crossbeam channels → ECS systems on main thread.

Connection states: Handshake → Login → Configuration → Play

### Key Architectural Decisions

- **Bevy ECS** is the core concurrency model — lockless, multithreaded, zero-copy where possible.
- **Custom serialization everywhere** — NBT, Anvil, and network codec are hand-written for performance, not using off-the-shelf Minecraft protocol libraries.
- **`DashMap` with `WyHash`** for concurrent chunk caching.
- **`phf` perfect hashing** for O(1) block/item lookups at runtime, compiled into the binary.
- Prefer `Arc` over `Mutex` for read-heavy shared data.

## Code Conventions

- **No `unwrap()`** — use `expect("descriptive context")` or proper error handling (`match`/`if let`).
- **Avoid `.clone()`** unless necessary or in one-time startup paths.
- **New crates must define their own `thiserror`-based error types.**
- **New dependencies go in the workspace `Cargo.toml`**, not individual crate manifests.
- **Use `#[expect(lint)]` instead of `#[allow(lint)]`** so suppressions are flagged when unnecessary.
- **Tests that only generate/dump data must be `#[ignore]`d.**
- **Use `get_root_path()`** instead of chaining `../` for project-relative paths. No absolute paths.

### Workspace Lints

Denied at workspace level (will fail CI): `wildcard_dependencies`, `cast_lossless`, `cast_ptr_alignment`, `match_bool`, `mut_mut`, `borrow_as_ptr`, `infinite_loop`, `unused_unsafe`, `missing_abi`, `future_incompatible`.

## Key Patterns

### ECS (Bevy)

- Components for entity data, Resources for global state, Systems for game logic.
- Messages (`src/lib/messages`) for inter-system event communication.

### Packets

Follow existing `#[packet(...)]` macro patterns in `ferrumc-net`.

### Async

- Tokio for async I/O (networking runs on a separate single-threaded Tokio runtime).
- crossbeam channels for thread-to-ECS communication (new connections, etc.).

## Branch Naming

- `feature/feature-name`, `fix/fixed-thing`, `rework/refactored-thing`, `housekeeping`, `docs`
- All PRs target `master`.

## Documentation

Project documentation lives in `docs/`, organized by topic in subdirectories (`docs/ci/`, `docs/networking/`, etc.). Use the `/document` command to add or update documentation.

**All non-trivial systems, pipelines, and architectural decisions must be documented.** When making significant changes (new workflows, new crates, new systems, architectural shifts), update or create the relevant docs. Documentation should reflect the current state of the code, not aspirational designs.

See `docs/` for the full documentation index.

## Rules for Generated Code

- Comments must be appropriate for an open source project with multiple contributors — they must NOT be aimed at any individual and must be timeless.
- Never co-author commits with Claude.
