> [!WARNING]
> # FerrumC isn't actively maintained
>
> This is a hobby project and nobody is actively working on it right now, so please be wary of possible security vulnerabilities, especially if you run it on a public server.
>
> This branch is an unfinished rewrite. It's on hold for now and may not get finished. The main version of FerrumC is on [`master`](https://github.com/ferrumc-rs/ferrumc/tree/master).

# FerrumC

A high-performance Minecraft Java Edition server implementation written in Rust.

## Status: v2 rework (creative/minigame core)

FerrumC v2 is a **deterministic, Rust-native, observable creative/minigame server core** for vanilla **Minecraft: Java Edition 1.21.8** clients. It is a clean-room core meant for creative and minigame servers — not a drop-in vanilla replacement.

It is **not** vanilla parity, **not** survival, and **not** a Paper/Spigot replacement. Scope today: **flat-world only**, **offline/local alpha**. The only supported client is Minecraft: Java Edition 1.21.8 (protocol 772) — see [`SUPPORTED_VERSION.md`](SUPPORTED_VERSION.md).

> **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**

## FerrumC v2 (rework branch) — current status

A playable vertical slice targeting **Minecraft Java 1.21.8 (protocol 772)**, **offline mode only**. This is honest about what is and isn't done — see [`docs/FEATURES.md`](docs/FEATURES.md) for the full, test-by-test breakdown.

**What works today:**

- Join a vanilla 1.21.8 client in offline mode (no "Loading terrain" hang).
- A flat creative world; build by placing blocks from the hotbar with **correct block states** — logs/slabs/stairs/torches/fences rotate and orient the way a real client expects.
- Multiplayer: other players are visible and move, **face the right way** (body + head rotation), and you can **see their held main-hand item**.
- In-game chat and `/` command autocomplete (`/spawn`, `/gamemode`, permission-filtered).
- Persistence across **leave/rejoin and restart** — placed blocks stick.
- In-process plugins that can allow / deny / replace block edits (a sample turns glass into tinted glass; another protects spawn).
- A read-only observability dashboard at `http://127.0.0.1:9090` (loopback-only by design).

**Limitations (not done yet):**

- Flat world only — no real terrain generation.
- Offline mode only — no online-mode authentication/encryption.
- Full-bright lighting (placeholder, no light engine).
- No survival, mobs, redstone, or full vanilla parity.
- Player state (position/gamemode/inventory) is not yet persisted.

**Roadmap (next):** Anvil world import/export, online-mode authentication, and real lighting + terrain.

See `docs/architecture/overview.md` for the design, [`docs/public-alpha.md`](docs/public-alpha.md) for the public-alpha checklist, and `CLAUDE.md` for development instructions.

## Building

```bash
cargo build --release
```

## License

MIT — see [LICENSE](LICENSE).

## Links

- **Website:** [ferrumc.com](https://ferrumc.com)
- **Discord:** [Join](https://discord.gg/ferrumc)
- **GitHub:** [ferrumc-rs/ferrumc](https://github.com/ferrumc-rs/ferrumc)
