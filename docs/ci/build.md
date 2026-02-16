# Build Profiles & Features

## Cargo Features

Features are defined in `src/bin/Cargo.toml`:

| Feature | What it includes | Used in |
|---|---|---|
| `dashboard` | Web dashboard (Axum server) | Default, release builds |
| `tracy` | Tracy profiler client | Local profiling only |
| `dhat` | Heap profiler | Local profiling only |
| `release` | All features suitable for distribution (`dashboard`) | Release builds |

Release builds use `--features release` (scoped to `-p ferrumc`) instead of `--all-features`. This is intentional — `tracy` and `dhat` are profiling tools that include C/C++ code incompatible with musl static linking (`tracy-client-sys` uses glibc-specific functions like `__snprintf_chk` and `__memcpy_chk`). When adding new features intended for release builds, add them to the `release` feature list.

## Build Profiles

| Profile | Use case | Key settings |
|---|---|---|
| `dev` | Local development | Default + per-package optimizations for heavy deps (yazi, bevy_ecs, tokio) |
| `release` | Standard release | Cargo defaults |
| `production` | CI release builds | release + `strip`, `lto`, `opt-level = 3`, `codegen-units = 1` |
| `profiling` | Tracy profiling | release + `debug = true` |
| `hyper` | Maximum performance | production + `panic = "abort"`, no overflow checks, no debug assertions |

The `production` profile is used by the release workflow. It's similar to `hyper` but keeps panic unwinding (no `panic = "abort"`), which is safer for a server binary — a panic unwinds and logs a backtrace instead of hard-crashing.

## Static Linking (Linux)

Linux binaries are statically linked via musl libc, producing fully self-contained binaries that run on any Linux distribution regardless of glibc version. This avoids the common `GLIBC_X.XX not found` error on older distros.

Requirements for musl builds:
- `musl-tools` package (installed automatically in CI)
- `CC=musl-gcc` and `CXX=g++` environment variables (set in CI build step)
- `rustls` instead of `native-tls` for TLS (see below)

### TLS Backend

The project uses `rustls` (pure Rust TLS) instead of `native-tls` (OpenSSL) for the `reqwest` HTTP client. This is required for musl static builds — `native-tls` depends on system OpenSSL which cannot be statically linked with musl. The `rustls` backend has no native dependencies and works across all targets.

Configured in the root `Cargo.toml`:
```toml
reqwest = { version = "0.13.1", features = ["json", "rustls", "blocking", "http2"], default-features = false }
```
