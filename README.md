FerrumC rewrite v3

repo link:

dev branch:
https://github.com/ferrumc-rs/ferrumc/tree/dev
or
https://github.ferrumc.com



====================

# FerrumC rewrite v3

### Plans:

plugins
- basic api right away
- wasm
- extism??

events
- nice and clean events system
- runtime registration (to allow plugins to work)
- extremely fast (a lot of systems would heavily rely on this )
- cancellable events

ecs
- basic stuff like components
- excellent iterators
- with tokio? optimized stuff?

chunks
- better format? throw away generation status and stuff
- only read the necessary data (tapes) ; simdnbt?

network encoding
- helpful derive macro like `#[derive(NetEncode)]` and `#[derive(NetDecode)]`
- support encoding options (omit packet size, compressed, and shit like that)

nbt
- custom tapes based reader and writer + GPU accelerated parsing
- implement methods for nbt to be network encoded, since i forgot that the first time

database
-  lmdb (k/v store)
- hash(key) => value

packet
- have a nice API (current way for parsing if fine, with derive macros)
- use events to handle them
- allow plugins to listen / interact with packets. cancel them etc
- both in and out
- server handles event on priority u8::MAX/2
- support compression (handled by network encoding)
- auto initialized with "reflections", read through every file basically
- support multi versions directly. map versions packet id and shit

redstone
- compile redstone and make it fast and accurate


misc
- horizontal scaling?

encryption
- mojang authentication and stuff


error handling
- actually consistent error handling

organization
- use workspaces



Tips:
- Pull request instead of direct pushings.