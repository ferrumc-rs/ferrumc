# simdnbt

Simdnbt is a very fast [NBT](https://minecraft.wiki/w/NBT_format) serializer and deserializer.

It was originally made as a joke but it ended up being too good of a joke so it's actually a thing now.

## Usage

```sh
cargo add simdnbt
```

### Deserializing

For deserializing, you'll likely want either [simdnbt::borrow::read](https://docs.rs/simdnbt/latest/simdnbt/borrow/fn.read.html) or [simdnbt::owned::read](https://docs.rs/simdnbt/latest/simdnbt/owned/fn.read.html).
The difference is that the "borrow" variant requires you to keep a reference to the original buffer, but is significantly faster.

```rust,no_run
use std::borrow::Cow;
use std::io::Cursor;

fn example(item_bytes: &[u8]) {
    let nbt = simdnbt::borrow::read(&mut Cursor::new(item_bytes))
        .unwrap()
        .unwrap();
    let skyblock_id: Cow<str> = nbt
        .list("i")
        .and_then(|i| i.compounds())
        .and_then(|i| i.first())
        .and_then(|i| i.compound("tag"))
        .and_then(|tag| tag.compound("ExtraAttributes"))
        .and_then(|ea| ea.string("id"))
        .map(|id| id.to_string_lossy())
        .unwrap_or_default();
}
```

### Serializing

```rust
use simdnbt::owned::{BaseNbt, Nbt, NbtCompound, NbtTag};

let nbt = Nbt::Some(BaseNbt::new(
    "",
    NbtCompound::from_values(vec![
        ("key".into(), NbtTag::String("value".into())),
    ]),
));
let mut buffer = Vec::new();
nbt.write(&mut buffer);
```

## Performance guide

Use the borrow variant of `Nbt` if possible, and avoid allocating unnecessarily (for example, keep strings as `Cow<str>` if you can).

If you're using the owned variant of simdnbt, switching to a faster allocator like [mimalloc](https://docs.rs/mimalloc/latest/mimalloc/) may help a decent amount (it's ~20% faster on my machine). Setting `RUSTFLAGS='-C target-cpu=native'` when running your code may also help a little bit.

## Implementation details

Simdnbt currently makes use of SIMD instructions for two things:

-   swapping the endianness of int arrays
-   checking if a string is plain ascii for faster MUTF-8 to UTF-8 conversion

Simdnbt ~~cheats~~ takes some shortcuts to be this fast:

1. it requires a reference to the original data (to avoid cloning)
2. it doesn't validate/decode the MUTF-8 strings at decode-time

Several ideas are borrowed from simdjson, notably the usage of a [tape](https://github.com/simdjson/simdjson/blob/master/doc/tape.md).

## Benchmarks

Simdnbt is the fastest NBT parser in Rust.

Here's a benchmark comparing Simdnbt against a few of the other fastest NBT crates for decoding [`complex_player.dat`](https://github.com/azalea-rs/simdnbt/blob/master/simdnbt/tests/complex_player.dat):

| Library                                                                     | Throughput   |
| --------------------------------------------------------------------------- | ------------ |
| [simdnbt::borrow](https://docs.rs/simdnbt/latest/simdnbt/borrow/index.html) | 3.9493 GiB/s |
| [simdnbt::owned](https://docs.rs/simdnbt/latest/simdnbt/owned/index.html)   | 825.59 MiB/s |
| [shen_nbt5](https://docs.rs/shen-nbt5/latest/shen_nbt5/)                    | 606.68 MiB/s |
| [graphite_binary](https://docs.rs/graphite_binary/latest/graphite_binary/)  | 363.94 MiB/s |
| [azalea_nbt](https://docs.rs/azalea-nbt/latest/azalea_nbt/)                 | 330.46 MiB/s |
| [valence_nbt](https://docs.rs/valence_nbt/latest/valence_nbt/)              | 279.58 MiB/s |
| [hematite_nbt](https://docs.rs/hematite-nbt/latest/nbt/)                    | 180.22 MiB/s |
| [fastnbt](https://docs.rs/fastnbt/latest/fastnbt/)                          | 162.92 MiB/s |

And for writing `complex_player.dat`:

| Library         | Throughput   |
| --------------- | ------------ |
| simdnbt::owned  | 2.5033 GiB/s |
| azalea_nbt      | 2.4152 GiB/s |
| simdnbt::borrow | 2.1317 GiB/s |
| graphite_binary | 1.8804 GiB/s |

The tables above were made from the [compare benchmark](https://github.com/azalea-rs/simdnbt/tree/master/simdnbt/benches) in this repo.
Note that the benchmark is somewhat unfair, since `simdnbt::borrow` doesn't fully decode some things like strings and integer arrays until they're used.
Also keep in mind that if you run your own benchmark you'll get different numbers, but the speeds should be about the same relative to each other.
