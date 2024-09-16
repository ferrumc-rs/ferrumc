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


- [ ] Events
  - [ ] Clean & Nice impl
  - [ ] Runteim registration (for plugins)
  - [ ] Light and efficient (heavily relied on)
  - [ ] Cancellable Events


- [ ] ECS
  - [ ] Basic impl (components, etc)
  - [ ] Excellent iterators
  - [ ] [Tokio](https://github.com/tokio-rs/tokio) for better optimisation?


- [ ] Chunks
  - [ ] Better format?
  - [ ] Only read necessary data (tapes) - simdnbt?


- [ ] Network Encoding
  - [ ] Helpful derive macro like `#[derive(NetEncode)]` & `#[derive(NetDecode)]`
  - [ ] Support encoding options (omit packet size, compression, etc)
  

- [ ] NBT
  - [ ] Custom tapes based read + writer
    - [ ] GPU accelerated Parsing
  - [ ] Implement methods for nbt to be network encoded


- [ ] Database
  - [ ] [LMDB](https://en.wikipedia.org/wiki/Lightning_Memory-Mapped_Database) (K/V Store)
  - [ ] Hash(key) => Value


- [ ] Packets
  - [ ] Functional API
  - [ ] use Events system for handling
    - [ ] Server handles on priority u8::MAX/2
  - [ ] Plugin support (listening, interacting, canceling, etc)
  - [ ] Auto init with reflections
  - [ ] support multiple versions (map versions packet id etc)


- [ ] Redstone
  - [ ] Compile redstone, lightweight impl (fast & accurate)


- [ ] Misc
  - [ ] Horizontal Scaling?
  - [ ] consistent error handling
  - [ ] use workspaces (organisation)


- [ ] Encryption
  - [ ] Mojang Auth etc
