# Workflows

FerrumC uses GitHub Actions for continuous integration and release automation. The pipeline is split into two workflows: one for PR validation and one for tagged releases.

## `rust.yml` — PR & Push CI

**Triggers:** Pull requests to `master`, pushes to `master`, manual `workflow_dispatch`.

**Concurrency:** Grouped by PR head ref. New pushes to the same PR cancel in-progress runs.

| Job | Runner | What it does |
|---|---|---|
| **Formatting and Security** | `ubuntu-latest` | `cargo fmt --check`, `cargo clippy -D warnings`, `cargo audit` |
| **Run Tests** | `ubuntu-latest` | `cargo nextest run` with `--all-targets --all-features` (excludes benchmarks) |

Both jobs run in parallel with no dependency between them.

Tests use `--all-features` to ensure gated features (dashboard, tracy, etc.) are always tested. This is safe on the gnu target used by CI — the musl incompatibility with tracy only affects release builds.

## `release.yml` — Tagged Release Pipeline

**Triggers:** Push of tags matching `v*` (e.g., `v1.0.0`, `v0.2.0-rc1`).

| Job | Runner | What it does |
|---|---|---|
| **Validate** | `ubuntu-latest` | Same checks as CI: fmt, clippy, nextest (with `--all-features` on gnu target) |
| **Build Release** | 4-platform matrix | Builds with `--profile production --features release`, packages binaries, generates SHA256 checksums |
| **Publish Release** | `ubuntu-latest` | Creates GitHub Release with auto-generated notes, attaches all archives and checksums |

### Build Matrix

| OS | Target | Archive Format |
|---|---|---|
| `ubuntu-latest` | `x86_64-unknown-linux-musl` | `.tar.gz` |
| `ubuntu-24.04-arm` | `aarch64-unknown-linux-musl` | `.tar.gz` |
| `windows-latest` | `x86_64-pc-windows-msvc` | `.zip` |
| `macos-14` | `aarch64-apple-darwin` | `.tar.gz` |

### Artifact Naming

Archives: `ferrumc-{tag}-{target}.tar.gz` (or `.zip` for Windows)
Checksums: `ferrumc-{tag}-{target}.tar.gz.sha256` (or `.zip.sha256`)

Example: `ferrumc-v1.0.0-x86_64-unknown-linux-musl.tar.gz`

### Platform-Specific Packaging

| Platform | Archive tool | Checksum tool |
|---|---|---|
| Linux | `tar -czf` | `sha256sum` |
| macOS | `tar -czf` | `shasum -a 256` (macOS lacks `sha256sum`) |
| Windows | PowerShell `Compress-Archive` (runs under `shell: pwsh`) | PowerShell `Get-FileHash` |

Windows steps use `shell: pwsh` explicitly since the workflow default shell is `bash`.

## Caching

Jobs use `Swatinem/rust-cache@v2`. Cache keys are set up intentionally:

- **`rust.yml` formatting job**: default key (job-name based). Compiles to `target/debug/` (no `--target` flag).
- **`rust.yml` test job**: `shared-key: "ferrumc-test"`. Compiles to `target/x86_64-unknown-linux-gnu/debug/` (explicit `--target` flag).
- **`release.yml` validate job**: `shared-key: "ferrumc-test"`. Reuses the test job's cache from `rust.yml`, since both compile with the same target and features.
- **`release.yml` build-release jobs**: `shared-key: "ferrumc"`. Separate from CI caches since these use `--profile production` on different platforms.

The formatting and test jobs in `rust.yml` must NOT share a cache key — they compile to different target directories, and GitHub Actions cache is immutable (first write wins). Sharing a key causes whichever job saves second to permanently lose its artifacts.

### Cache Performance

With warm caches, typical CI times are:
- Formatting and Security: ~30-40s
- Run Tests: ~50-60s

Cold cache (first run or after Cargo.lock changes): ~2-3 minutes per job.

## Suppressed Advisories

`cargo audit` ignores `RUSTSEC-2023-0071`. If this advisory is resolved upstream, the ignore can be removed from the workflow.
