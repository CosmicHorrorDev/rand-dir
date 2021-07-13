# _rand-dir_

A featureful library for easily constructing complex directory sturctures for
test purposes where the specific names and file contents typically aren't
important

## Quickstart

```rust
{
    use rand_dir::{Dir, File, RandDir};

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
} // <-- Directory cleaned up when it goes out of scope
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
