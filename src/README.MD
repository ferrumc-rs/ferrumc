# FerrumC source code

Welcome to FerrumC project source code. 

The FerrumC project source code is organized as follows:

## Code source layout

FerrumC will define the following primary crates inside its source code:
- `ferrumc-core`
- `ferrumc-ecs`
- `ferrumc-events`
- `ferrumc-net`
- `ferrumc-storage`
- `ferrumc-utils`
- `ferrumc-world`
- `ferrumc-plugins`

Each crate may contain subcrates within its directory to implement essential components. For example:
```md
- core/          # ferrumc-core
- net/           # ferrumc-net
    - encode     # ferrumc-net-encode
```

## Dependencies

Dependencies are defined in the workspace Cargo.toml file and imported into each crate using the following syntax:
```toml
crate = { workspace = true }
```

Workspace members dependencies are defined as follows:
```toml
member = { workspace = true } 
```

## Clippy

FerrumC enforce the usage of clippy and this source code should not contains any clippy warning.

## Components

Any system within FerrumC is built with the ability to be shutdown at some point in time.

## Tests

Tests are located within their respective crates and do not have a separate tests/ folder.
