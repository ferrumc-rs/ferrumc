# CI/CD Pipeline

FerrumC uses GitHub Actions for continuous integration and release automation. The pipeline is split into two workflows: one for PR validation and one for tagged releases.

## Workflows

### `rust.yml` — PR & Push CI

**Triggers:** Pull requests to `master`, pushes to `master`, manual `workflow_dispatch`.

**Concurrency:** Grouped by PR head ref. New pushes to the same PR cancel in-progress runs.

| Job | Runner | What it does |
|---|---|---|
| **Formatting and Security** | `ubuntu-latest` | `cargo fmt --check`, `cargo clippy -D warnings`, `cargo audit` |
| **Run Tests** | `ubuntu-latest` | `cargo nextest run` with `--all-targets --all-features` (excludes benchmarks) |

Both jobs run in parallel with no dependency between them.

### `release.yml` — Tagged Release Pipeline

**Triggers:** Push of tags matching `v*` (e.g., `v1.0.0`, `v0.2.0-rc1`).

| Job | Runner | What it does |
|---|---|---|
| **Validate** | `ubuntu-latest` | Same checks as CI: fmt, clippy, nextest |
| **Build Release** | 4-platform matrix | Builds with `--profile production --all-features`, packages binaries, generates SHA256 checksums |
| **Publish Release** | `ubuntu-latest` | Creates GitHub Release with auto-generated notes, attaches all archives and checksums |

#### Build Matrix

| OS | Target | Archive Format |
|---|---|---|
| `ubuntu-latest` | `x86_64-unknown-linux-gnu` | `.tar.gz` |
| `ubuntu-24.04-arm` | `aarch64-unknown-linux-gnu` | `.tar.gz` |
| `windows-latest` | `x86_64-pc-windows-msvc` | `.zip` |
| `macos-14` | `aarch64-apple-darwin` | `.tar.gz` |

#### Release Artifact Naming

Archives: `ferrumc-{tag}-{target}.tar.gz` (or `.zip` for Windows)
Checksums: `ferrumc-{tag}-{target}.tar.gz.sha256` (or `.zip.sha256`)

Example: `ferrumc-v1.0.0-x86_64-unknown-linux-gnu.tar.gz`

## Build Profiles

| Profile | Use case | Key settings |
|---|---|---|
| `dev` | Local development | Default + per-package optimizations for heavy deps (yazi, bevy_ecs, tokio) |
| `release` | Standard release | Cargo defaults |
| `production` | CI release builds | release + `strip`, `lto`, `opt-level = 3`, `codegen-units = 1` |
| `profiling` | Tracy profiling | release + `debug = true` |
| `hyper` | Maximum performance | production + `panic = "abort"`, no overflow checks, no debug assertions |

The `production` profile is used by the release workflow. It's similar to `hyper` but keeps panic unwinding (no `panic = "abort"`), which is safer for a server binary — a panic unwinds and logs a backtrace instead of hard-crashing.

## Caching

Each job in `rust.yml` uses `Swatinem/rust-cache@v2` with **separate cache keys per job** (the default). This is intentional — the formatting job compiles to `target/debug/` while the test job compiles to `target/x86_64-unknown-linux-gnu/debug/` (due to the explicit `--target` flag). Sharing a cache key between them causes the test job to always miss, since GitHub Actions cache is immutable (first write wins).

The release workflow uses `shared-key: "ferrumc"` across its jobs since the validate and build jobs can benefit from shared artifacts.

### Cache performance

With warm caches, typical CI times are:
- Formatting and Security: ~30-40s
- Run Tests: ~50-60s

Cold cache (first run or after Cargo.lock changes): ~2-3 minutes per job.

## Creating a Release

1. Ensure `master` is in a releasable state (CI green).
2. Tag the commit: `git tag v1.0.0`
3. Push the tag: `git push origin v1.0.0`
4. The release workflow runs automatically: validate → build (4 platforms) → publish GitHub Release.
5. If the build fails on any platform, fix the issue and create a new tag.

## Suppressed Advisories

`cargo audit` ignores `RUSTSEC-2023-0071`. If this advisory is resolved upstream, the ignore can be removed from the workflow.
