## Access a file randomly using a memory map

[![memmap-badge]][memmap] [![cat-filesystem-badge]][cat-filesystem]

Add the `memmap2` crate to your own project:

```
cargo add memmap2
```

Creates a memory map of a file using [memmap] and simulates some non-sequential
reads from the file. Using a memory map means you just index into a slice rather
than dealing with [`seek`] to navigate a File.

The [`Mmap::map`] function assumes the file
behind the memory map is not being modified at the same time by another process
or else a [race condition] occurs. This is why it needs to be enclosed
in an [`unsafe`] block.

```rust
{{#include examples/memmap.rs}}
```

[`Mmap::map`]: https://docs.rs/memmap/*/memmap/struct.Mmap.html#method.map
[`seek`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.seek
[`unsafe`]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html

[race condition]: https://en.wikipedia.org/wiki/Race_condition#File_systems
