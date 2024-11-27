<img style="width: 100%" src="https://github.com/ferrumc-rs/ferrumc/blob/dev/README/assets/header.svg?raw=true" alt="FerrumC Header">

# Ferrumc Rewrite
> [!WARNING]
> FerrumC is going through a major rewrite. This branch contains all of the new code.\
> To view the original code, go to the [dev branch](https://github.com/ferrumc-rs/ferrumc/tree/dev).

> [!IMPORTANT]
> Use pull requests instead of direct pushes.

## Roadmap (subject to change):

- [ ] Plugins
  - [ ] Basic API (right away)
  - [ ] WASM
  - [ ] [Extism???](https://extism.org/)


- [x] Events
  - [x] Clean & Nice impl
  - [x] Runtime registration (for plugins)
  - [x] Light and efficient (heavily relied on)
  - [x] Cancellable Events


- [ ] ECS
  - [x] Basic impl (components, etc)
  - [x] Excellent iterators
  - [-] [Tokio](https://github.com/tokio-rs/tokio) for better optimisation?
  - [x] Rayon parallel iterators on components


- [ ] Chunks
  - [ ] Better format?
  - [ ] Only read necessary data (tapes) - simdnbt?


- [x] Network Encoding
  - [x] Helpful derive macro like `#[derive(NetEncode)]` & `#[derive(NetDecode)]`
  - [x] Support encoding options (omit packet size, ~~compression~~, etc)
  

- [x] NBT
  - [x] Custom tapes based read + writer
  - [x] Implement methods for nbt to be network encoded


- [ ] Database
  - [x] [redb](https://www.redb.org/)
  - [ ] [LMDB](https://en.wikipedia.org/wiki/Lightning_Memory-Mapped_Database) (K/V Store)
  - [ ] Hash(key) => Value


- [ ] Packets
  - [ ] Functional API
  - [x] use Events system for handling
    - [x] Server handles on priority u8::MAX/2
  - [ ] Plugin support (listening, interacting, canceling, etc)
  - [x] Auto init with reflections
  - [ ] support multiple versions (map versions packet id etc)

- [ ] Redstone
  - [ ] Compile redstone, lightweight impl (fast & accurate)


- [ ] Misc
  - [ ] Horizontal Scaling?
  - [ ] consistent error handling
  - [ ] use workspaces (organisation)


- [ ] Encryption
  - [ ] Mojang Auth etc
