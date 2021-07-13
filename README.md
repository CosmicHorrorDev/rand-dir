# _rand-dir_

Easily construct complex directory structures

This is primarily focused towards property testing where the exact contents of
different files or entry's names may not matter since you are only concerned
about your program upholding certain properties like a directory being
identical when compressed and then decompressed or that it traverses symlinks
properly

## Quickstart

```rust
use rand_dir::{Dir, File, RandDir};

fn main -> Result<(), Box<dyn std::error::Error>> {
    let rand_dir = RandDir::builder()
        .dir(
            Dir::real()
                .file(File::zeroed())
                .file(File::oned())
        )
        .dir(
            Dir::symlink()
                .file(File::random())
        )
        .file(File::custom(*b"Hello, World!"))
        .try_build()?;

    println!("Base of directory at {:?}", rand_dir.at());

    Ok(())
} // <-- Directory is cleaned up when it goes out of scope
```

Will create a temporary directory like below where the `base` directory can be
accessed with `rand_dir.at()`

```text
 Size Name
    - /tmp/rand-dir-.XHiQMQpuhnTH
    - ├── base
    - │  ├── dir-deep-goose
4.0Ki │  │  ├── file-ultimate-piglet.oned
4.0Ki │  │  └── file-verified-hamster.zeroed
   13 │  ├── file-grateful-flamingo.custom
   51 │  └── symlink-champion-shrimp -> /tmp/rand-dir-.XHiQMQpuhnTH/symlinks/symlink-dest-1
    - ├── broken-symlinks
    - └── symlinks
    -    └── symlink-dest-1
4.0Ki       └── file-legal-sunbeam.random
```

and if needed the names, permissions, and filesizes can all be customized by
calling the relevant methods on each entry

## License

Licensed under either of

 - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
